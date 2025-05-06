use crate::{
    ast::*,
    context::workspace::WorkspaceContext,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    iter::zip,
    ops::Add,
};

/// Given an AST Block, it tries to detect any state variable inside it that may have been
/// manipulated. Now it's important to know that manipulations can occur either directly by
/// assigning to a state variable or, it may occur by assigning to a storage pointer that points to
/// some state variable. This light weight state variable manipulation finder captures both of the
/// above kinds of assignments. However, it's not smart enough to use a data dependency graph to
/// determine the exact state variables these storage pointers would be pointing to, in the context
/// of the block-flow.
///
/// NOTE - Assignment is not the only avenue for manipulating state variables, but also operations
/// like `push()` and `pop()` on arrays, `M[i] = x` on mappings, `delete X` imply the same.
///
/// Here, the term manipulation covers all kinds of changes discussed above.
///
/// IMPORTANT: DO NOT MAKE THESE MEMBERS PUBLIC. Use the public methods implemented on this
/// structure only.
pub struct ApproximateStorageChangeFinder<'a> {
    directly_manipulated_state_variables: BTreeSet<NodeID>,
    manipulated_storage_pointers: BTreeSet<NodeID>,
    /// Key => State Variable ID, Value => Storage Pointer ID (Heuristics based, this map is NOT
    /// exhaustive) It leaves out a lot of links especially in cases where storage pointers are
    /// passed to and fro internal functions. But on average, for most cases the way code is
    /// generally written, this should contain a decent chunk of links to lookup.
    state_variables_to_storage_pointers: BTreeMap<NodeID, BTreeSet<NodeID>>,
    context: &'a WorkspaceContext,
}

/// This trait implementation will be useful when we run it through our callgraph and try to
/// aggregate state variable changes.
impl<'a> Add<ApproximateStorageChangeFinder<'_>> for ApproximateStorageChangeFinder<'a> {
    type Output = ApproximateStorageChangeFinder<'a>;

    fn add(mut self, rhs: ApproximateStorageChangeFinder) -> Self::Output {
        self.directly_manipulated_state_variables
            .extend(rhs.directly_manipulated_state_variables.iter());
        self.manipulated_storage_pointers.extend(rhs.manipulated_storage_pointers.iter());
        // For state_variables_to_storage_pointers, we have to "add" the storage point entry vectors
        for (state_var_id, storage_pointer_ids) in &rhs.state_variables_to_storage_pointers {
            match self.state_variables_to_storage_pointers.entry(*state_var_id) {
                std::collections::btree_map::Entry::Vacant(v) => {
                    v.insert(BTreeSet::from_iter(storage_pointer_ids.iter().copied()));
                }
                std::collections::btree_map::Entry::Occupied(mut o) => {
                    (*o.get_mut()).extend(storage_pointer_ids);
                }
            };
        }
        self
    }
}

impl Debug for ApproximateStorageChangeFinder<'_> {
    // Do not print context. Hence, debug is custom derived for this struct
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Manipulated directly: {:?}", self.directly_manipulated_state_variables)?;

        if !self.directly_manipulated_state_variables.is_empty() {
            writeln!(f, "↓↓")?;
            for id in &self.directly_manipulated_state_variables {
                if let Some(node) = self.context.nodes.get(id) {
                    let loc_info = self.context.get_node_sort_key(node);
                    writeln!(f, "Line {:?}", (loc_info.1, loc_info.2))?;
                } else {
                    writeln!(f, "<unknown_node_id_{}>\n", id)?;
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
                    writeln!(f, "<unknown_node_id_{}>", id)?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "Links heuristics: {:?}", self.state_variables_to_storage_pointers)?;

        if !self.state_variables_to_storage_pointers.is_empty() {
            writeln!(f, "↓↓")?;
            for (state_variable_id, storage_pointer_references) in
                &self.state_variables_to_storage_pointers
            {
                if let Some(node) = self.context.nodes.get(state_variable_id) {
                    let loc_info = self.context.get_node_sort_key(node);
                    writeln!(f, "Links to {:?}", (loc_info.1, loc_info.2))?;
                    for node_id in storage_pointer_references {
                        if let Some(node) = self.context.nodes.get(node_id) {
                            let loc_info = self.context.get_node_sort_key(node);
                            writeln!(f, "\t <> {:?}", (loc_info.1, loc_info.2))?;
                        }
                    }
                } else {
                    writeln!(f, "<unknown_node_id_{}>", state_variable_id)?;
                }
            }
            writeln!(f)?;
        }

        std::fmt::Result::Ok(())
    }
}

/// Interface to be used by other modules defined here.
impl<'a> ApproximateStorageChangeFinder<'a> {
    /// Initialize
    pub fn from<T: Node + ?Sized>(context: &'a WorkspaceContext, node: &T) -> Self {
        let mut extractor = ApproximateStorageChangeFinder {
            directly_manipulated_state_variables: BTreeSet::new(),
            manipulated_storage_pointers: BTreeSet::new(),
            state_variables_to_storage_pointers: BTreeMap::new(),
            context,
        };
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }

    pub fn state_variables_have_been_manipulated(&self) -> bool {
        !self.directly_manipulated_state_variables.is_empty()
            || !self.manipulated_storage_pointers.is_empty()
    }

    pub fn no_state_variable_has_been_manipulated(&self) -> bool {
        self.directly_manipulated_state_variables.is_empty()
            && self.manipulated_storage_pointers.is_empty()
    }

    pub fn fetch_non_exhaustive_manipulated_state_variables(&self) -> Vec<&VariableDeclaration> {
        let mut manipulated_state_vars: BTreeSet<NodeID> = BTreeSet::new();
        manipulated_state_vars.extend(self.directly_manipulated_state_variables.iter());
        for (state_variable_id, storage_pointers) in self.state_variables_to_storage_pointers.iter()
        {
            if storage_pointers.iter().any(|ptr| self.manipulated_storage_pointers.contains(ptr)) {
                manipulated_state_vars.insert(*state_variable_id);
            }
        }
        manipulated_state_vars
            .into_iter()
            .flat_map(|v| self.context.nodes.get(&v))
            .flat_map(|n| {
                if let ASTNode::VariableDeclaration(variable_declaration) = n {
                    assert!(variable_declaration.state_variable);
                    return Some(variable_declaration);
                }
                None
            })
            .collect()
    }

    pub fn state_variable_has_been_manipulated(&self, var: &VariableDeclaration) -> Option<bool> {
        if self.directly_manipulated_state_variables.contains(&var.id) {
            return Some(true);
        }
        if self.manipulated_storage_pointers.is_empty() {
            return Some(false);
        }
        // Now use our heuristics
        if self.state_variables_to_storage_pointers.get(&var.id).is_some_and(|entry| {
            entry.iter().any(|e| self.manipulated_storage_pointers.contains(e))
        }) {
            return Some(true);
        }

        // At this point, we don't know if any of the storage pointers refer to [`var`], so we
        // cannot say for sure, if it has been manipulated or not.
        None
    }

    pub fn state_variable_has_not_been_manipulated(
        &self,
        var: &VariableDeclaration,
    ) -> Option<bool> {
        if self.directly_manipulated_state_variables.contains(&var.id) {
            return Some(false);
        }
        if self.manipulated_storage_pointers.is_empty() {
            return Some(true);
        }
        // Now use our heuristics
        if self.state_variables_to_storage_pointers.get(&var.id).is_some_and(|entry| {
            entry.iter().any(|e| self.manipulated_storage_pointers.contains(e))
        }) {
            return Some(false);
        }

        // At this point, we don't know if any of the storage pointers refer to [`var`], so we
        // cannot say for sure, if it has been manipulated or not.
        None
    }
}

impl ASTConstVisitor for ApproximateStorageChangeFinder<'_> {
    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        // WRITE HEURISTICS
        // Catch unary operations that manipulate variables
        if node.operator == "delete" || node.operator == "++" || node.operator == "--" {
            for id in find_base(node.sub_expression.as_ref()).0 {
                match is_storage_variable_or_storage_pointer(self.context, id) {
                    Some(AssigneeType::StateVariable) => {
                        self.directly_manipulated_state_variables.insert(id);
                    }
                    Some(AssigneeType::StorageLocationVariable) => {
                        self.manipulated_storage_pointers.insert(id);
                    }
                    None => {}
                };
            }
        }

        Ok(true)
    }

    fn visit_member_access(&mut self, member: &MemberAccess) -> Result<bool> {
        if !member.expression.type_descriptions().is_some_and(|type_desc| {
            type_desc.type_string.as_ref().is_some_and(|type_string| {
                type_string.ends_with("[] storage ref")
                    || type_string.ends_with("[] storage pointer")
            })
        }) {
            return Ok(true);
        }

        let (base_variable_ids, _) = find_base(member.expression.as_ref());
        // assert!(base_variable_ids.len() == 1);

        // WRITE HEURISTICS
        if member.member_name == "push" || member.member_name == "pop" {
            for id in base_variable_ids.iter() {
                match is_storage_variable_or_storage_pointer(self.context, *id) {
                    Some(AssigneeType::StateVariable) => {
                        self.directly_manipulated_state_variables.insert(*id);
                    }
                    Some(AssigneeType::StorageLocationVariable) => {
                        self.manipulated_storage_pointers.insert(*id);
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
            // When something is assigned to an expression of type "storage pointer", no state
            // variable's value changes. The only value changed is the thing which the
            // storage pointer points to. The value of a storage variable changes if the
            // expression's type string contains "storage ref" in case of structs, arrays, etc

            if type_string.ends_with("storage pointer") {
                continue;
            }

            match is_storage_variable_or_storage_pointer(self.context, *id) {
                Some(AssigneeType::StateVariable) => {
                    self.directly_manipulated_state_variables.insert(*id);
                }
                Some(AssigneeType::StorageLocationVariable) => {
                    self.manipulated_storage_pointers.insert(*id);
                }
                None => {}
            };
        }

        // Now, on a separate note, let's look for a heuristic to link up state variables with
        // storage pointers. But here, we only handle the cases when there are equal number
        // of elements on either side of `=` . This allows us to assume 1:1 relationship.
        if base_variable_lhs_ids.len() == base_variable_rhs_ids.len() {
            for (lhs_id, rhs_id) in zip(base_variable_lhs_ids, base_variable_rhs_ids) {
                if let (
                    Some(AssigneeType::StorageLocationVariable),
                    Some(AssigneeType::StateVariable),
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
        }

        // READ HEURISTICS
        // Pretty much the same logic as described in `visit_variable_declaration_statement`
        //

        // let (left_node_ids, _) = find_base(assignment.left_hand_side.as_ref());
        // let right_node_ids = flatten_expression(assignment.right_hand_side.as_ref());

        // See if it's a 1:1 relationship. Only then, proceed. Later, we can think of heuristics to
        // handle x:y (but likely we will have to rely on a dependency graph for that. Case:
        // `(x, y) = func()` is out of scope for this).
        //
        // When it comes to tracking WRITEs, it doesn't matter what's on RHS of `=`
        // When it comes to tracking READs, it does! If the LHS type is storage then you are simply
        // carrying a reference at compile time, you are not actually reading. Whereas, if
        // the LHS is memory, you are performing "sload"! But again, this logic changes
        // based on type of value in RHS. If it's a function call you should look at
        // return values and nature of the corresponding variable where that value will be stored.
        // Likewise, different for different nodes albeit identifier one is probably the
        // most straightforward if left_node_ids.len() == right_node_ids.len() {}

        Ok(true)
    }

    // For heurstics: Try to link up as much storage pointers and state variables as possible
    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        if let Some(initial_value) = node.initial_value.as_ref() {
            let corresponding_variable_declaration_ids = node
                .declarations
                .iter()
                .map(|v| {
                    if let Some(variable_declaration) = v {
                        variable_declaration.id
                    } else {
                        i64::MIN
                    }
                })
                .collect::<Vec<_>>();
            let initial_value_node_ids = flatten_expression(initial_value); // For READ heuristics
            let (initial_value_bases, _) = find_base(initial_value); // For LINK heuristics

            // Let's first support 1:1 relationships only
            if corresponding_variable_declaration_ids.len() == initial_value_node_ids.len() {
                let common_len = corresponding_variable_declaration_ids.len();

                // This for loop takes care of recording instances in VDS (Var Decl Stmnt) where
                // there is a read from the storage.
                //
                //  READ HEURISTICS
                //
                // TODO: Write tests for these
                // Then creates `passes_read_check()` before matching the var id on extracting
                // reference declarations Then replicate the logic for assignments
                // use lvaluerequested = false and islvalue = true to check if it's being read
                // in case of visiting indexaccess, memberaccess, etc (expr_node! variants)
                // So that it can detect stuff outside of just assignments. Example -
                // functionCall(a.b) where a is state var (Technically you are
                // reading a.b's value to make that function call)
                //
                // for i in 0..common_len {
                //     let variable_declaration_id = corresponding_variable_declaration_ids[i];
                //     let corresponding_initial_value_id = initial_value_node_ids[i];

                //     if is_storage_variable_or_storage_pointer(
                //         &self.context,
                //         variable_declaration_id,
                //     )
                //     .is_some()
                //     {
                //         // If we are not assigning something to a storage pointer or a storage
                // reference, that means         // we're storing it in memory.
                // Therefore, we can consider that the corresponding initialValue
                //         // is being "read". Otherwise we are just creating pointers, not
                // "sload"ing.         continue;
                //     }

                //     if let Some(node) = self.context.nodes.get(&corresponding_initial_value_id) {
                //         // In case of internal function call, it's complex to analyze
                //         if node.node_type() != NodeType::FunctionCall || it is{
                //             let referenced_declarations =
                //                 ExtractReferencedDeclarations::from(node).extracted;

                //             for variable_id in referenced_declarations {
                //                 match is_storage_variable_or_storage_pointer(
                //                     self.context,
                //                     variable_id,
                //                 ) {
                //                     // Assumption: At this point in code we know that it could be
                // a storage pointer/variable that represents                     //
                // uint, bool, array element, etc. so it's technically being read.
                //                     Some(AssigneeType::StateVariable) => {
                //                         self.directly_read_state_variables.insert(variable_id);
                //                     }
                //                     Some(AssigneeType::StorageLocationVariable) => {
                //                         self.read_storage_pointers.insert(variable_id);
                //                     }
                //                     None => {}
                //                 }
                //             }
                //         }
                //     }
                // }

                assert!(initial_value_bases.len() == common_len);

                // LINK heuristics
                for i in 0..common_len {
                    let variable_declaration_id = corresponding_variable_declaration_ids[i];
                    let corresponding_initial_value_base_id = initial_value_bases[i];

                    if let (
                        Some(AssigneeType::StorageLocationVariable),
                        Some(AssigneeType::StateVariable),
                    ) = (
                        is_storage_variable_or_storage_pointer(
                            self.context,
                            variable_declaration_id,
                        ),
                        is_storage_variable_or_storage_pointer(
                            self.context,
                            corresponding_initial_value_base_id,
                        ),
                    ) {
                        match self
                            .state_variables_to_storage_pointers
                            .entry(corresponding_initial_value_base_id)
                        {
                            std::collections::btree_map::Entry::Vacant(v) => {
                                v.insert(BTreeSet::from_iter([variable_declaration_id]));
                            }
                            std::collections::btree_map::Entry::Occupied(mut o) => {
                                (*o.get_mut()).insert(variable_declaration_id);
                            }
                        };
                    }
                }
            }
        }

        Ok(true)
    }
}

fn flatten_expression(expr: &Expression) -> Vec<NodeID> {
    let mut node_ids = vec![];
    match expr {
        Expression::TupleExpression(TupleExpression { components, .. }) => {
            for component in components.iter().flatten() {
                let component_node_ids = flatten_expression(component);
                node_ids.extend(component_node_ids);
            }
        }
        _ => {
            node_ids.push(expr.get_node_id().unwrap_or(i64::MIN));
        }
    }
    node_ids
}

fn find_base(expr: &Expression) -> (Vec<NodeID>, Vec<String>) {
    let mut node_ids = vec![];
    let mut type_strings = vec![];
    match expr {
        Expression::Identifier(Identifier {
            referenced_declaration: Some(id),
            type_descriptions: TypeDescriptions { type_string: Some(type_string), .. },
            ..
        }) => {
            node_ids.push(*id);
            type_strings.push(type_string.clone());
        }
        // Handle mappings assignment
        Expression::IndexAccess(IndexAccess {
            base_expression,
            type_descriptions: TypeDescriptions { type_string: Some(type_string), .. },
            ..
        }) => {
            node_ids.extend(find_base(base_expression.as_ref()).0);
            type_strings.push(type_string.clone());
        }
        // Handle struct member assignment
        Expression::MemberAccess(MemberAccess {
            expression,
            type_descriptions: TypeDescriptions { type_string: Some(type_string), .. },
            ..
        }) => {
            node_ids.extend(find_base(expression.as_ref()).0);
            type_strings.push(type_string.clone());
        }
        // Handle tuple form lhs while assigning
        Expression::TupleExpression(TupleExpression { components, .. }) => {
            for component in components.iter() {
                if let Some(component) = component {
                    let (component_node_ids, component_type_strings) = find_base(component);
                    node_ids.extend(component_node_ids);
                    type_strings.extend(component_type_strings);
                } else {
                    node_ids.push(i64::MIN);
                    type_strings.push(String::from("irrelevant"));
                }
            }
        }
        // Handle assignment with values like ++i, --j, i++, etc
        Expression::UnaryOperation(UnaryOperation {
            sub_expression,
            type_descriptions: TypeDescriptions { type_string: Some(type_string), .. },
            ..
        }) => {
            node_ids.extend(find_base(sub_expression.as_ref()).0);
            type_strings.push(type_string.clone());
        }
        _ => {
            node_ids.push(i64::MIN);
            type_strings.push(String::from("irrelevant"));
        }
    };
    assert_eq!(node_ids.len(), type_strings.len());
    (node_ids, type_strings)
}

#[derive(PartialEq)]
enum AssigneeType {
    StateVariable,
    StorageLocationVariable,
}

fn is_storage_variable_or_storage_pointer(
    context: &WorkspaceContext,
    node_id: NodeID,
) -> Option<AssigneeType> {
    let node = context.nodes.get(&node_id)?;
    if let ASTNode::VariableDeclaration(variable) = node {
        if variable.storage_location == StorageLocation::Transient {
            return None;
        }
        // Assumption
        // variable.state_variable is true when it's an actual state variable
        // variable.storage_location is StorageLocation::Storage when it's a storage reference
        // pointer
        if variable.state_variable {
            return Some(AssigneeType::StateVariable);
        } else if variable.storage_location == StorageLocation::Storage {
            return Some(AssigneeType::StorageLocationVariable);
        }
    }
    None
}

#[cfg(test)]
mod approximate_storage_change_finder_tests {
    use crate::detect::test_utils::load_solidity_source_unit;

    use super::ApproximateStorageChangeFinder;

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

        let finder = ApproximateStorageChangeFinder::from(&context, func);
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        println!("NoStateVarManipulationExample::dontManipulateStateVar()\n{:?}", finder);
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
        let func2 = contract.find_function_by_name("readSimpleStateVars");

        let finder = ApproximateStorageChangeFinder::from(&context, func);
        let changes_found = finder.state_variables_have_been_manipulated();
        println!("SimpleStateVarManipulationExample::manipulateStateVarDirectly()\n{:?}", finder);
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 5);
        assert!(finder.manipulated_storage_pointers.is_empty());

        let finder = ApproximateStorageChangeFinder::from(&context, func2);
        let changes_found = finder.state_variables_have_been_manipulated();
        println!("SimpleStateVarManipulationExample::readSimpleStateVars()\n{:?}", finder);
        assert!(!changes_found);
        assert!(finder.directly_manipulated_state_variables.is_empty());
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

        let finder = ApproximateStorageChangeFinder::from(&context, func1);
        println!("FixedSizeArraysAssignmentExample::manipulateDirectly()\n{:?}", finder);

        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test manipulateViaIndexAccess() function

        let finder = ApproximateStorageChangeFinder::from(&context, func2);
        println!("FixedSizeArraysAssignmentExample::manipulateViaIndexAccess()\n{:?}", finder);

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
        let func7 = contract.find_function_by_name("manipulateStateVariables7");
        let func8 = contract.find_function_by_name("manipulateStateVariables8");
        let func_helper = contract.find_function_by_name("manipulateHelper");

        // Test manipulateStateVariables
        let finder = ApproximateStorageChangeFinder::from(&context, func);
        println!("StructPlusFixedArrayAssignmentExample::manipulateStateVariables()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 3);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test manipulateStateVariables2
        let finder = ApproximateStorageChangeFinder::from(&context, func2);
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables2()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert_eq!(
            finder
                .state_variables_to_storage_pointers
                .get(&contract.find_state_variable_node_id_by_name("person3"))
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            finder
                .state_variables_to_storage_pointers
                .get(&contract.find_state_variable_node_id_by_name("persons"))
                .unwrap()
                .len(),
            1
        );
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables3
        let finder = ApproximateStorageChangeFinder::from(&context, func3);
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables3()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());
        assert_eq!(finder.state_variables_to_storage_pointers.len(), 1); // person3 links to ptr_person3
        assert_eq!(
            finder
                .state_variables_to_storage_pointers
                .get(&contract.find_state_variable_node_id_by_name("person3"))
                .unwrap()
                .len(),
            1
        );

        // Test manipulateStateVariables4
        let finder = ApproximateStorageChangeFinder::from(&context, func4);
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables4()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateStateVariables5
        let finder = ApproximateStorageChangeFinder::from(&context, func5);
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables5()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());
        assert_eq!(finder.state_variables_to_storage_pointers.len(), 1);
        assert_eq!(
            finder
                .state_variables_to_storage_pointers
                .get(&contract.find_state_variable_node_id_by_name("person3"))
                .unwrap()
                .len(),
            1
        );

        // Test funcHelper
        let finder = ApproximateStorageChangeFinder::from(&context, func_helper);
        println!("StructPlusFixedArrayAssignmentExample::manipulateHelper()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 2);
        assert!(finder.directly_manipulated_state_variables.is_empty());
        assert_eq!(
            finder
                .state_variables_to_storage_pointers
                .get(&contract.find_state_variable_node_id_by_name("person3"))
                .unwrap()
                .len(),
            1
        );

        // Test manipulateStateVariables6
        let finder = ApproximateStorageChangeFinder::from(&context, func6);
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables6()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 2);
        assert!(finder.directly_manipulated_state_variables.is_empty());
        assert_eq!(
            finder
                .state_variables_to_storage_pointers
                .get(&contract.find_state_variable_node_id_by_name("person3"))
                .unwrap()
                .len(),
            4
        );

        // Test manipulateStateVariables7
        let finder = ApproximateStorageChangeFinder::from(&context, func7);
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables7()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 3);

        // Test manipulateStateVariables8
        let finder = ApproximateStorageChangeFinder::from(&context, func8);
        println!(
            "StructPlusFixedArrayAssignmentExample::manipulateStateVariables8()\n{:?}",
            finder
        );
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 2);
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
        let finder = ApproximateStorageChangeFinder::from(&context, func);
        println!("SVManipulationLibrary::manipulateLib()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateLib2()
        let finder = ApproximateStorageChangeFinder::from(&context, func2);
        println!("SVManipulationLibrary::manipulateLib2()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.manipulated_storage_pointers.len(), 1);
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test manipulateLib3()
        let finder = ApproximateStorageChangeFinder::from(&context, func3);
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
        let func3 = contract.find_function_by_name("dontManipulateStateVariablesPart3");
        let func4 = contract.find_function_by_name("dontManipulateStateVariablesPart4");
        let func5 = contract.find_function_by_name("dontManipulateStateVariablesPart5");

        // Test dontManipulateStateVariables()
        let finder = ApproximateStorageChangeFinder::from(&context, func);
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariables()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test dontManipulateStateVariablesPart2()
        let finder = ApproximateStorageChangeFinder::from(&context, func2);
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariablesPart2()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test dontManipulateStateVariablesPart3()
        let finder = ApproximateStorageChangeFinder::from(&context, func3);
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariablesPart3()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test dontManipulateStateVariablesPart4()
        let finder = ApproximateStorageChangeFinder::from(&context, func4);
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariablesPart4()\n{:?}",
            finder
        );
        let no_changes_found = !finder.state_variables_have_been_manipulated();
        assert!(no_changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert!(finder.directly_manipulated_state_variables.is_empty());

        // Test dontManipulateStateVariablesPart4()
        let finder = ApproximateStorageChangeFinder::from(&context, func5);
        println!(
            "NoStructPlusFixedArrayAssignmentExample::dontManipulateStateVariablesPart5()\n{:?}",
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
        let finder = ApproximateStorageChangeFinder::from(&context, func);
        println!("DynamicArraysPushExample::manipulateDirectly()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);

        // Test manipulateViaIndexAccess()
        let finder = ApproximateStorageChangeFinder::from(&context, func2);
        println!("DynamicArraysPushExample::manipulateViaIndexAccess()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 3);

        // Test manipulateViaMemberAccess()
        let finder = ApproximateStorageChangeFinder::from(&context, func3);
        println!("DynamicArraysPushExample::manipulateViaMemberAccess()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert!(finder.manipulated_storage_pointers.is_empty());
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);

        // Test manipulateViaMemberAccess2()
        let finder = ApproximateStorageChangeFinder::from(&context, func4);
        println!("DynamicArraysPushExample::manipulateViaMemberAccess2()\n{:?}", finder);
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
        let finder = ApproximateStorageChangeFinder::from(&context, func);
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
        let finder = ApproximateStorageChangeFinder::from(&context, func);
        println!("FixedSizeArraysDeletionExample::manipulateDirectly()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 1);
        assert!(finder.manipulated_storage_pointers.is_empty());

        // Test func2()
        let finder = ApproximateStorageChangeFinder::from(&context, func2);
        println!("FixedSizeArraysDeletionExample::manipulateViaIndexAccess()\n{:?}", finder);
        let changes_found = finder.state_variables_have_been_manipulated();
        assert!(changes_found);
        assert_eq!(finder.directly_manipulated_state_variables.len(), 2);
        assert!(finder.manipulated_storage_pointers.is_empty());
    }
}
