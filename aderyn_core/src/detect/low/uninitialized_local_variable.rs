use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::ast::{ASTNode, NodeID};

use crate::{
    capture,
    context::{browser::ExtractReferencedDeclarations, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UninitializedLocalVariableDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UninitializedLocalVariableDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Assumption:
        // VariableDeclarationStatements consists of statements that look like `uint x;` `uint y,
        // z;`, `uint p = 12;` but are not declared at the contract level (state level) but
        // rather within functions and modifiers

        let mut potentially_uninitialized_local_variables = HashSet::new();

        for variable_declaration_statement in context
            .variable_declaration_statements()
            .into_iter()
            .filter(|s| s.initial_value.is_none())
        {
            potentially_uninitialized_local_variables.extend(
                variable_declaration_statement.declarations.iter().flat_map(|s| {
                    if let Some(ref s) = s {
                        return Some(s.id);
                    }
                    None
                }),
            );
        }

        // We can filter out the initialized variables by looking at LHS of assignments.
        // This trick works for local variables because it's not possible to have structs, mappings,
        // dynamic arrays declared local to the function.
        for assignment in context.assignments() {
            let references =
                ExtractReferencedDeclarations::from(assignment.left_hand_side.as_ref()).extracted;
            potentially_uninitialized_local_variables.retain(|v| !references.contains(v));
        }

        // Blacklist variables assigned via Yul Assignments
        let mut blacklist_variable_names = HashSet::new();

        for yul_assignment in context.yul_assignments() {
            blacklist_variable_names
                .extend(yul_assignment.variable_names.iter().map(|v| v.name.clone()))
        }

        for id in potentially_uninitialized_local_variables {
            if let Some(ASTNode::VariableDeclaration(v)) = context.nodes.get(&id) {
                if !blacklist_variable_names.contains(&v.name) {
                    // Ignore memory structs because they can have an initializeMethod of their own.
                    // So not covered under the assignment operator
                    if v.type_descriptions
                        .type_string
                        .as_ref()
                        .is_some_and(|type_string| !type_string.contains("struct "))
                    {
                        capture!(self, context, v);
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Uninitialized Local Variable")
    }

    fn description(&self) -> String {
        String::from("Initialize all the variables. If a variable is meant to be initialized to zero, explicitly set it to zero to improve code readability.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UninitializedLocalVariable)
    }
}

#[cfg(test)]
mod uninitialized_local_variables_detector_tests {

    use crate::detect::{
        detector::IssueDetector,
        low::uninitialized_local_variable::UninitializedLocalVariableDetector,
    };

    #[test]

    fn test_uninitialized_local_variables() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UninitializedLocalVariables.sol",
        );

        let mut detector = UninitializedLocalVariableDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 12);
    }
}
