use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::context::browser::Peek;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
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
        String::from("Dangerous unary operator found in assignment.")
    }

    fn description(&self) -> String {
        String::from("Potentially mistakened `=+` for `+=` or `=-` for `-=`. Please include a space in between.")
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
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::dangerous_unary_operator::DangerousUnaryOperatorDetector,
    };

    #[test]
    #[serial]
    fn test_dangerous_unary_operator() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DangerousUnaryOperator.sol",
        );

        let mut detector = DangerousUnaryOperatorDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue

        println!("{:#?}", detector.instances());

        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Dangerous unary operator found in assignment.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Potentially mistakened `=+` for `+=` or `=-` for `-=`. Please include a space in between.")
        );
    }
}
