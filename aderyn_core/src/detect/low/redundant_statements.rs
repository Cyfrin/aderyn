use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, NodeID, NodeType};

use crate::capture;
use crate::context::browser::GetImmediateParent;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the RedundantStatementsDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct RedundantStatementsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for RedundantStatementsDetector {
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
        String::from("Redundant statements have no effect.")
    }

    fn description(&self) -> String {
        String::from("Remove the redundant statements because no code will be generated and it just congests the codebase.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::RedundantStatements.to_string()
    }
}

#[cfg(test)]
mod redundant_statements_detector {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, low::redundant_statements::RedundantStatementsDetector,
    };

    #[test]
    #[serial]
    fn test_redundant_statements() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/RedundantStatements.sol",
        );

        let mut detector = RedundantStatementsDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);

        println!("{:#?}", detector.instances());

        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 6);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Redundant statements have no effect.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Remove the redundant statements because no code will be generated and it just congests the codebase.")
        );
    }
}
