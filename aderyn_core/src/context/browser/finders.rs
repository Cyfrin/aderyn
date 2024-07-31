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
                if let Some(node) = self.context.nodes.get(&id) {
                    let loc_info = self.context.get_node_sort_key(node);
                    write!(f, "Line {:?}\n", (loc_info.1, loc_info.2))?;
                } else {
                    write!(f, "<uknown_node_id_{}>\n", id)?;
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
                if let Some(node) = self.context.nodes.get(&id) {
                    let loc_info = self.context.get_node_sort_key(node);
                    write!(f, "Line {:?}\n", (loc_info.1, loc_info.2))?;
                } else {
                    write!(f, "<uknown_node_id_{}>\n", id)?;
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

    pub fn has_variable_declaration_been_manipulated(&self, var: &VariableDeclaration) -> bool {
        self.directly_manipulated_state_variables.contains(&var.id)
    }
}

impl<'a> ASTConstVisitor for LightWeightStateVariableManipulationFinder<'a> {
    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        // Catch delete operations
        if node.operator == "delete" {
            if let Some(id) = find_referenced_declaration_for_identifier_or_indexed_identifier(
                node.sub_expression.as_ref(),
            ) {
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
        if let Some(id) = find_referenced_declaration_for_identifier_or_indexed_identifier(
            member.expression.as_ref(),
        ) {
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
        if let Some(id) = find_referenced_declaration_for_identifier_or_indexed_identifier(
            assignment.left_hand_side.as_ref(),
        ) {
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

fn find_referenced_declaration_for_identifier_or_indexed_identifier(
    expr: &Expression,
) -> Option<NodeID> {
    match expr {
        Expression::Identifier(Identifier {
            referenced_declaration: Some(id),
            ..
        }) => {
            return Some(*id);
        }
        // Handle mappings assignment
        Expression::IndexAccess(IndexAccess {
            base_expression, ..
        }) => {
            return find_referenced_declaration_for_identifier_or_indexed_identifier(
                base_expression.as_ref(),
            );
        }
        // Handle struct member assignment
        Expression::MemberAccess(MemberAccess { expression, .. }) => {
            return find_referenced_declaration_for_identifier_or_indexed_identifier(
                expression.as_ref(),
            );
        }
        _ => (),
    };
    None
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
