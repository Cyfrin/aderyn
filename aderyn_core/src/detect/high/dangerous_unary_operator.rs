use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::{
    capture,
    context::{browser::Peek, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DangerousUnaryOperatorDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DangerousUnaryOperatorDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments() {
            if let Some(content) = assignment.peek(context) {
                if content.contains("=-") || content.contains("=+") {
                    capture!(self, context, assignment);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Dangerous unary operator")
    }

    fn description(&self) -> String {
        String::from("Potentially mistaken `=+` for `+=` or `=-` for `-=`. This acts as an assignment instead of an increment or decrement.\
        Use the correct operator to increment or decrement a variable.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::DangerousUnaryOperator.to_string()
    }
}

#[cfg(test)]
mod dangerous_unary_expression_tests {

    use crate::detect::{
        detector::IssueDetector, high::dangerous_unary_operator::DangerousUnaryOperatorDetector,
    };

    #[test]
    fn test_dangerous_unary_operator() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DangerousUnaryOperator.sol",
        );

        let mut detector = DangerousUnaryOperatorDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
