use crate::{
    ast::*,
    context::workspace_context::WorkspaceContext,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::*;
use std::{collections::BTreeSet, fmt::Debug};

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
pub struct LightWeightStateVariableManipulationFinder<'a> {
    directly_manipulated_state_variables: BTreeSet<NodeID>,
    manipulated_storage_pointers: BTreeSet<NodeID>,
    context: &'a WorkspaceContext,
}

impl<'a> Debug for LightWeightStateVariableManipulationFinder<'a> {
    // Do not print context. Hence, debug is custom derived for this struct
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Manipulated directly: : {:?}",
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

impl<'a> LightWeightStateVariableManipulationFinder<'a> {
    pub fn from<T: Node + ?Sized>(context: &'a WorkspaceContext, node: &T) -> Self {
        let mut extractor = LightWeightStateVariableManipulationFinder {
            directly_manipulated_state_variables: BTreeSet::new(),
            manipulated_storage_pointers: BTreeSet::new(),
            context,
        };
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }

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

impl<'a> ASTConstVisitor for LightWeightStateVariableManipulationFinder<'a> {
    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        // Catch delete operations
        if node.operator == "delete" {
            for id in find_base(node.sub_expression.as_ref()) {
                match is_storage_variable_or_storage_pointer(self.context, id) {
                    Some(AssigneeType::StorageVariable) => {
                        self.directly_manipulated_state_variables.insert(id);
                    }
                    Some(AssigneeType::StorageVariablePointer) => {
                        self.manipulated_storage_pointers.insert(id);
                    }
                    None => {}
                };
            }
        }
        Ok(true)
    }

    fn visit_member_access(&mut self, member: &MemberAccess) -> Result<bool> {
        for id in find_base(member.expression.as_ref()) {
            if member.member_name == "push" || member.member_name == "pop" {
                match is_storage_variable_or_storage_pointer(self.context, id) {
                    Some(AssigneeType::StorageVariable) => {
                        self.directly_manipulated_state_variables.insert(id);
                    }
                    Some(AssigneeType::StorageVariablePointer) => {
                        self.manipulated_storage_pointers.insert(id);
                    }
                    None => {}
                };
            }
        }
        Ok(true)
    }

    fn visit_assignment(&mut self, assignment: &Assignment) -> Result<bool> {
        // When something is assigned to an expression of type "storage pointer", no state variable's value changes.
        // The only value changed is the thing which the storage pointer points to.
        // The value of a storage variable changes if te expression's type string contains "storage ref"
        if assignment
            .left_hand_side
            .type_descriptions()
            .is_some_and(|type_desc| {
                type_desc
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| type_string.ends_with("storage pointer"))
            })
        {
            return Ok(true);
        }

        for id in find_base(assignment.left_hand_side.as_ref()) {
            match is_storage_variable_or_storage_pointer(self.context, id) {
                Some(AssigneeType::StorageVariable) => {
                    self.directly_manipulated_state_variables.insert(id);
                }
                Some(AssigneeType::StorageVariablePointer) => {
                    self.manipulated_storage_pointers.insert(id);
                }
                None => {}
            };
        }
        Ok(true)
    }
}

fn find_base(expr: &Expression) -> Vec<NodeID> {
    let mut node_ids = vec![];
    match expr {
        Expression::Identifier(Identifier {
            referenced_declaration: Some(id),
            ..
        }) => {
            node_ids.push(*id);
        }
        // Handle mappings assignment
        Expression::IndexAccess(IndexAccess {
            base_expression, ..
        }) => {
            node_ids.extend(find_base(base_expression.as_ref()));
        }
        // Handle struct member assignment
        Expression::MemberAccess(MemberAccess { expression, .. }) => {
            node_ids.extend(find_base(expression.as_ref()));
        }
        // Handle tuple form lhs while assigning
        Expression::TupleExpression(TupleExpression { components, .. }) => {
            for component in components.iter().flatten() {
                node_ids.extend(find_base(component))
            }
        }
        _ => (),
    };
    node_ids
}

enum AssigneeType {
    StorageVariable,
    StorageVariablePointer,
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
            return Some(AssigneeType::StorageVariablePointer);
        }
    }
    None
}

#[cfg(test)]
mod light_weight_state_variables_finder_tests {
    use crate::detect::test_utils::load_solidity_source_unit;

    use super::LightWeightStateVariableManipulationFinder;

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

        let finder = LightWeightStateVariableManipulationFinder::from(&context, func.into());
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

        let finder = LightWeightStateVariableManipulationFinder::from(&context, func.into());
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

        let finder = LightWeightStateVariableManipulationFinder::from(&context, func1.into());
        println!(
            "FixedSizeArraysAssignmentExample::manipulateDirectly()\n{:?}",
            finder
        );

        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test manipulateViaIndexAccess() function

        let finder = LightWeightStateVariableManipulationFinder::from(&context, func2.into());
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
        let func_helper = contract.find_function_by_name("manipulateHelper");

        // Test manipulateStateVariables
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 3);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test manipulateStateVariables2
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func2.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables2()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables3
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func3.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables3()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables4
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func4.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables4()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables5
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func5.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables5()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test funcHelper
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func_helper.into());
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateHelper()\n{:?}",
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
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func.into());
        println!("SVManipulationLibrary::manipulateLib()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateLib2()
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func2.into());
        println!("SVManipulationLibrary::manipulateLib2()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateLib3()
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func3.into());
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
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func.into());
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariables()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test dontManipulateStateVariablesPart2()
        let finder = LightWeightStateVariableManipulationFinder::from(&context, func2.into());
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariablesPart2()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());
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
