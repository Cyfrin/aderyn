use std::{collections::BTreeMap, error::Error};

use crate::ast::{Expression, NodeID, NodeType};

use crate::{
    capture,
    context::{browser::GetImmediateParent, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the RedundantStatementDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct RedundantStatementDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for RedundantStatementDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for expression_statement in context.expression_statements() {
            if let Some(parent) = expression_statement.parent(context) {
                if parent.node_type() != NodeType::Block {
                    continue;
                }

                match &expression_statement.expression {
                    Expression::Identifier(identifier) => {
                        capture!(self, context, identifier);
                    }
                    Expression::ElementaryTypeNameExpression(elementary_type_expression) => {
                        capture!(self, context, elementary_type_expression);
                    }
                    _ => (),
                };
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Redundant Statement")
    }

    fn description(&self) -> String {
        String::from("Remove the redundant statement.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::RedundantStatement.to_string()
    }
}

#[cfg(test)]
mod redundant_statements_detector {

    use crate::detect::{
        detector::IssueDetector, low::redundant_statement::RedundantStatementDetector,
    };

    #[test]

    fn test_redundant_statements() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/RedundantStatements.sol",
        );

        let mut detector = RedundantStatementDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 6);
    }
}
