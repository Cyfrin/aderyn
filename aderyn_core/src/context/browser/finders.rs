use crate::{
    ast::*,
    context::workspace_context::WorkspaceContext,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    iter::zip,
};

/// Given an AST Block, it tries to detect any state variable inside it that may have been manipulated.
/// Now it's important to know that manipulations can occur either directly by assigning to a state variable
/// or, it may occur by assigning to a storage pointer that points to some state variable.
/// This light weight state variable manipulation finder captures both of the above kinds of assignments.
/// However, it's not smart enough to use a data dependency graph to determine the exact state variables
/// these storage pointers would be pointing to, in the context of the block-flow.
///
/// NOTE - Asignment is not the only avenue for manipulating state variables, but also operations like
/// `push()` and `pop()` on arrays, `M[i] = x` on mappings, `delete X` imply the same.
///
/// Here, the term manipulation covers all kinds of changes discussed above.
pub struct ApproxiamateStateVariableManipulationFinder<'a> {
    directly_manipulated_state_variables: BTreeSet<NodeID>,
    manipulated_storage_pointers: BTreeSet<NodeID>,
    /// Key => State Variable ID, Value => Storage Pointer ID (Heuristics based, this map is NOT exhaustive)
    /// It leaves out a lot of links especially in cases where storage pointers are passed to and fro internal functions.
    /// But on average, for most cases the way code is generally written, this should contain a decent chunk of links to lookup.
    state_variables_to_storage_pointers: BTreeMap<NodeID, BTreeSet<NodeID>>,
    context: &'a WorkspaceContext,
}

impl<'a> Debug for ApproxiamateStateVariableManipulationFinder<'a> {
    // Do not print context. Hence, debug is custom derived for this struct
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Manipulated directly: {:?}",
            self.directly_manipulated_state_variables
        )?;

        if !self.directly_manipulated_state_variables.is_empty() {
            writeln!(f, "↓↓")?;
            for id in &self.directly_manipulated_state_variables {
                if let Some(node) = self.context.nodes.get(id) {
                    let loc_info = self.context.get_node_sort_key(node);
                    writeln!(f, "Line {:?}", (loc_info.1, loc_info.2))?;
                } else {
                    writeln!(f, "<uknown_node_id_{}>\n", id)?;
                }
            }
            writeln!(f)?;
        }

        writeln!(
            f,
            "Manipulated through storage pointers: {:?}",
            self.manipulated_storage_pointers
        )?;

        if !self.manipulated_storage_pointers.is_empty() {
            writeln!(f, "↓↓")?;
            for id in &self.manipulated_storage_pointers {
                if let Some(node) = self.context.nodes.get(id) {
                    let loc_info = self.context.get_node_sort_key(node);
                    writeln!(f, "Line {:?}", (loc_info.1, loc_info.2))?;
                } else {
                    writeln!(f, "<uknown_node_id_{}>", id)?;
                }
            }
            writeln!(f)?;
        }

        std::fmt::Result::Ok(())
    }
}

impl<'a> ApproxiamateStateVariableManipulationFinder<'a> {
    pub fn from<T: Node + ?Sized>(context: &'a WorkspaceContext, node: &T) -> Self {
        let mut extractor = ApproxiamateStateVariableManipulationFinder {
            directly_manipulated_state_variables: BTreeSet::new(),
            manipulated_storage_pointers: BTreeSet::new(),
            state_variables_to_storage_pointers: BTreeMap::new(),
            context,
        };
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }

    // TODO: Use the links to get out better information
    pub fn state_variables_have_been_manipulated(&self) -> bool {
        !self.directly_manipulated_state_variables.is_empty()
            || !self.manipulated_storage_pointers.is_empty()
    }

    pub fn variable_declaration_has_been_manipulated(
        &self,
        var: &VariableDeclaration,
    ) -> Option<bool> {
        if self.directly_manipulated_state_variables.contains(&var.id) {
            return Some(true);
        }
        if self.manipulated_storage_pointers.is_empty() {
            return Some(false);
        }
        // At this point, we don't know if any of the storage pointers refer to [`var`], so we cannot say for
        // sure, if it has been manipulated or not.
        None
    }

    pub fn variable_declaration_has_not_been_manipulated(
        &self,
        var: &VariableDeclaration,
    ) -> Option<bool> {
        if self.directly_manipulated_state_variables.contains(&var.id) {
            return Some(false);
        }
        if self.manipulated_storage_pointers.is_empty() {
            return Some(true);
        }
        // At this point, we don't know if any of the storage pointers refer to [`var`], so we cannot say for
        // sure, if it has been manipulated or not.
        None
    }
}

impl<'a> ASTConstVisitor for ApproxiamateStateVariableManipulationFinder<'a> {
    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        // Catch delete operations
        if node.operator == "delete" {
            for id in find_base(node.sub_expression.as_ref()).0 {
                match is_storage_variable_or_storage_pointer(self.context, id) {
                    Some(AssigneeType::StorageVariable) => {
                        self.directly_manipulated_state_variables.insert(id);
                    }
                    Some(AssigneeType::StorageVariableReference) => {
                        self.manipulated_storage_pointers.insert(id);
                    }
                    None => {}
                };
            }
        }
        Ok(true)
    }

    fn visit_member_access(&mut self, member: &MemberAccess) -> Result<bool> {
        if !member
            .expression
            .type_descriptions()
            .is_some_and(|type_desc| {
                type_desc.type_string.as_ref().is_some_and(|type_string| {
                    type_string.ends_with("[] storage ref")
                        || type_string.ends_with("[] storage pointer")
                })
            })
        {
            return Ok(true);
        }

        let (base_variable_ids, _) = find_base(member.expression.as_ref());

        for id in base_variable_ids {
            if member.member_name == "push" || member.member_name == "pop" {
                match is_storage_variable_or_storage_pointer(self.context, id) {
                    Some(AssigneeType::StorageVariable) => {
                        self.directly_manipulated_state_variables.insert(id);
                    }
                    Some(AssigneeType::StorageVariableReference) => {
                        self.manipulated_storage_pointers.insert(id);
                    }
                    None => {}
                };
            }
        }
        Ok(true)
    }

    fn visit_assignment(&mut self, assignment: &Assignment) -> Result<bool> {
        let (base_variable_lhs_ids, type_strings) = find_base(assignment.left_hand_side.as_ref());
        let (base_variable_rhs_ids, _) = find_base(assignment.right_hand_side.as_ref());

        for (id, type_string) in zip(base_variable_lhs_ids.iter(), type_strings.iter()) {
            // When something is assigned to an expression of type "storage pointer", no state variable's value changes.
            // The only value changed is the thing which the storage pointer points to.
            // The value of a storage variable changes if the expression's type string contains "storage ref" in case of structs, arrays, etc

            if type_string.ends_with("storage pointer") {
                continue;
            }

            match is_storage_variable_or_storage_pointer(self.context, *id) {
                Some(AssigneeType::StorageVariable) => {
                    self.directly_manipulated_state_variables.insert(*id);
                }
                Some(AssigneeType::StorageVariableReference) => {
                    self.manipulated_storage_pointers.insert(*id);
                }
                None => {}
            };
        }

        // Now, on a separate note, let's look for a heuristic to link up state variables with storage pointers.
        // This heuristic is tested on function `manipulateStateVariables5()` in `StateVariablesManipulation.sol`
        for (lhs_id, rhs_id) in zip(base_variable_lhs_ids, base_variable_rhs_ids) {
            if let (
                Some(AssigneeType::StorageVariableReference),
                Some(AssigneeType::StorageVariable),
            ) = (
                is_storage_variable_or_storage_pointer(self.context, lhs_id),
                is_storage_variable_or_storage_pointer(self.context, rhs_id),
            ) {
                match self.state_variables_to_storage_pointers.entry(rhs_id) {
                    std::collections::btree_map::Entry::Vacant(v) => {
                        v.insert(BTreeSet::from_iter([lhs_id]));
                    }
                    std::collections::btree_map::Entry::Occupied(mut o) => {
                        (*o.get_mut()).insert(lhs_id);
                    }
                };
            }
        }
        Ok(true)
    }

    // For heurstics: Try to link up as much storage pointers and state variables as possible
    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        if let Some(Expression::Identifier(Identifier {
            referenced_declaration: Some(id),
            ..
        })) = node.initial_value.as_ref()
        {
            if is_storage_variable_or_storage_pointer(self.context, *id)
                .is_some_and(|t| t == AssigneeType::StorageVariable)
                && node.declarations.len() == 1
            {
                if let Some(n) = node.declarations[0].as_ref() {
                    if is_storage_variable_or_storage_pointer(self.context, n.id)
                        .is_some_and(|t| t == AssigneeType::StorageVariableReference)
                    {
                        match self.state_variables_to_storage_pointers.entry(*id) {
                            std::collections::btree_map::Entry::Vacant(v) => {
                                v.insert(BTreeSet::from_iter([n.id]));
                            }
                            std::collections::btree_map::Entry::Occupied(mut o) => {
                                (*o.get_mut()).insert(n.id);
                            }
                        };
                    }
                }
            }
        }

        Ok(true)
    }
}

fn find_base(expr: &Expression) -> (Vec<NodeID>, Vec<&String>) {
    let mut node_ids = vec![];
    let mut type_strings = vec![];
    match expr {
        Expression::Identifier(Identifier {
            referenced_declaration: Some(id),
            type_descriptions:
                TypeDescriptions {
                    type_string: Some(type_string),
                    ..
                },
            ..
        }) => {
            node_ids.push(*id);
            type_strings.push(type_string);
        }
        // Handle mappings assignment
        Expression::IndexAccess(IndexAccess {
            base_expression,
            type_descriptions:
                TypeDescriptions {
                    type_string: Some(type_string),
                    ..
                },
            ..
        }) => {
            node_ids.extend(find_base(base_expression.as_ref()).0);
            type_strings.push(type_string);
        }
        // Handle struct member assignment
        Expression::MemberAccess(MemberAccess {
            expression,
            type_descriptions:
                TypeDescriptions {
                    type_string: Some(type_string),
                    ..
                },
            ..
        }) => {
            node_ids.extend(find_base(expression.as_ref()).0);
            type_strings.push(type_string);
        }
        // Handle tuple form lhs while assigning
        Expression::TupleExpression(TupleExpression { components, .. }) => {
            for component in components.iter().flatten() {
                let (component_node_ids, component_type_strings) = find_base(component);
                node_ids.extend(component_node_ids);
                type_strings.extend(component_type_strings);
            }
        }
        _ => (),
    };
    assert_eq!(node_ids.len(), type_strings.len());
    (node_ids, type_strings)
}

#[derive(PartialEq)]
enum AssigneeType {
    StorageVariable,
    StorageVariableReference,
}

fn is_storage_variable_or_storage_pointer(
    context: &WorkspaceContext,
    node_id: NodeID,
) -> Option<AssigneeType> {
    let node = context.nodes.get(&node_id)?;
    if let ASTNode::VariableDeclaration(variable) = node {
        // Assumption
        // variable.state_variable is true when it's an actual state variable
        // variable.storage_location is StorageLocation::Storage when it's a storage reference pointer
        if variable.state_variable {
            return Some(AssigneeType::StorageVariable);
        } else if variable.storage_location == StorageLocation::Storage {
            return Some(AssigneeType::StorageVariableReference);
        }
    }
    None
}

#[cfg(test)]
mod light_weight_state_variables_finder_tests {
    use crate::detect::test_utils::load_solidity_source_unit;

    use super::ApproxiamateStateVariableManipulationFinder;

    #[test]
    fn has_variable_declarations() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        assert!(!context.variable_declarations().is_empty());
    }

    #[test]
    fn test_no_state_variable_manipulations_found() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("NoStateVarManipulationExample");
        let func = contract.find_function_by_name("dontManipulateStateVar");

        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        println!("{:?}", finder);
        assert!(no_changes_found);
    }

    #[test]
    fn test_simple_state_variable_manipulations_found() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("SimpleStateVarManipulationExample");
        let func = contract.find_function_by_name("manipulateStateVarDirectly");

        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        let changes_found = finder.state_variables_have_been_manipulated();
        println!(
            "SimpleStateVarManipulationExample::manipulateStateVarDirectly()\n{:?}",
            finder
        );
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 5);
        assert!(finder.manipulated_storage_pointers.is_empty());
    }

    #[test]
    fn test_fixed_size_array_assignments() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("FixedSizeArraysAssignmentExample");

        let func1 = contract.find_function_by_name("manipulateDirectly");
        let func2 = contract.find_function_by_name("manipulateViaIndexAccess");

        // Test manipulateDirectly() function

        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func1.into());
        println!(
            "FixedSizeArraysAssignmentExample::manipulateDirectly()\n{:?}",
            finder
        );

        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test manipulateViaIndexAccess() function

        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func2.into());
        println!(
            "FixedSizeArraysAssignmentExample::manipulateViaIndexAccess()\n{:?}",
            finder
        );

        let changes_found2 = finder.state_variables_have_been_manipulated();
        assert!(changes_found2);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 2);
        assert!(finder.manipulated_storage_pointers.is_empty());
    }

    #[test]
    fn test_struct_plus_fixed_array_assignment_example() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("StructPlusFixedArrayAssignmentExample");

        let func = contract.find_function_by_name("manipulateStateVariables");
        let func2 = contract.find_function_by_name("manipulateStateVariables2");
        let func3 = contract.find_function_by_name("manipulateStateVariables3");
        let func4 = contract.find_function_by_name("manipulateStateVariables4");
        let func5 = contract.find_function_by_name("manipulateStateVariables5");
        let func6 = contract.find_function_by_name("manipulateStateVariables6");
        let func_helper = contract.find_function_by_name("manipulateHelper");

        // Test manipulateStateVariables
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 3);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test manipulateStateVariables2
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func2.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables2()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables3
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func3.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables3()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());
        assert_eq!(finder.state_variables_to_storage_pointers.len(), 1); // person3 links to ptr_person3

        // Test manipulateStateVariables4
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func4.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables4()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables5
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func5.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables5()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());
        assert_eq!(finder.state_variables_to_storage_pointers.len(), 1);

        // Test funcHelper
        let finder =
            ApproxiamateStateVariableManipulationFinder::from(&context, func_helper.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateHelper()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 2);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables5
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func6.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables6()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 2);
        assert!(finder.directly_manipulated_state_variables.is_empty());
    }

    #[test]
    fn test_sv_manipulation_library() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("SVManipulationLibrary");

        let func = contract.find_function_by_name("manipulateLib");
        let func2 = contract.find_function_by_name("manipulateLib2");
        let func3 = contract.find_function_by_name("manipulateLib3");

        // Test manipulateLib()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        println!("SVManipulationLibrary::manipulateLib()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateLib2()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func2.into());
        println!("SVManipulationLibrary::manipulateLib2()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateLib3()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func3.into());
        println!("SVManipulationLibrary::manipulateLib3()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 2);
        assert!(finder.directly_manipulated_state_variables.is_empty());
    }

    #[test]
    fn test_no_struct_plus_fixed_array_assignment_example() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("NoStructPlusFixedArrayAssignmentExample");

        let func = contract.find_function_by_name("dontManipulateStateVariables");
        let func2 = contract.find_function_by_name("dontManipulateStateVariablesPart2");

        // Test dontManipulateStateVariables()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariables()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test dontManipulateStateVariablesPart2()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func2.into());
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariablesPart2()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());
    }

    #[test]
    fn test_dynamic_array_push_changes() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("DynamicArraysPushExample");

        let func = contract.find_function_by_name("manipulateDirectly");
        let func2 = contract.find_function_by_name("manipulateViaIndexAccess");
        let func3 = contract.find_function_by_name("manipulateViaMemberAccess");
        let func4 = contract.find_function_by_name("manipulateViaMemberAccess2");

        // Test manipulateDirectly()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        println!(
            "DynamicArraysPushExample::manipulateDirectly()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);

        // Test manipulateViaIndexAccess()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func2.into());
        println!(
            "DynamicArraysPushExample::manipulateViaIndexAccess()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 3);

        // Test manipulateViaMemberAccess()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func3.into());
        println!(
            "DynamicArraysPushExample::manipulateViaMemberAccess()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);

        // Test manipulateViaMemberAccess2()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func4.into());
        println!(
            "DynamicArraysPushExample::manipulateViaMemberAccess2()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1); // we only want to capture p1
        assert!(finder.directly_manipulated_state_variables.is_empty());
    }

    #[test]
    fn test_dynamic_mappings_array_push_changes() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("DynamicMappingsArrayPushExample");
        let func = contract.find_function_by_name("add");

        // Test add()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        println!("DynamicMappingsArrayPushExample::add()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);
        assert!(finder.manipulated_storage_pointers.is_empty());
    }

    #[test]
    fn test_fixed_size_arrays_deletion_example() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesManipulation.sol",
        );

        let contract = context.find_contract_by_name("FixedSizeArraysDeletionExample");
        let func = contract.find_function_by_name("manipulateDirectly");
        let func2 = contract.find_function_by_name("manipulateViaIndexAccess");

        // Test func()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func.into());
        println!(
            "FixedSizeArraysDeletionExample::manipulateDirectly()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test func2()
        let finder = ApproxiamateStateVariableManipulationFinder::from(&context, func2.into());
        println!(
            "FixedSizeArraysDeletionExample::manipulateViaIndexAccess()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 2);
        assert!(finder.manipulated_storage_pointers.is_empty());
    }
}

#[cfg(test)]
mod state_variables_tests_helper {
    use crate::context::workspace_context::WorkspaceContext;

    use super::{ContractDefinition, FunctionDefinition};

    impl WorkspaceContext {
        pub fn find_contract_by_name(&self, name: &str) -> &ContractDefinition {
            self.contract_definitions()
                .into_iter()
                .find(|c| c.name.as_str() == name)
                .unwrap()
        }
    }

    impl ContractDefinition {
        pub fn find_function_by_name(&self, name: &str) -> &FunctionDefinition {
            self.function_definitions()
                .iter()
                .find(|func| func.name == name)
                .unwrap()
        }
    }
}
