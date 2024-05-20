use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    ast::Expression,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DivisionBeforeMultiplicationDetector {
    // Keys are source file name, line number, and description
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DivisionBeforeMultiplicationDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for op in context
            .binary_operations()
            .iter()
            .filter(|op| op.operator == "*")
        {
            if let Expression::BinaryOperation(left_op) = op.left_expression.as_ref() {
                if left_op.operator == "/" {
                    capture!(self, context, left_op)
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Incorrect Order of Division and Multiplication")
    }

    fn description(&self) -> String {
        String::from("Division operations followed directly by multiplication operations can lead to precision loss due to the way integer arithmetic is handled in Solidity.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::DivisionBeforeMultiplication)
    }
}

#[cfg(test)]
mod division_before_multiplication_detector_tests {
    use super::DivisionBeforeMultiplicationDetector;
    use crate::detect::detector::IssueDetector;

    #[test]
    fn test_template_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DivisionBeforeMultiplication.sol",
        );

        let mut detector = DivisionBeforeMultiplicationDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 4);
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        assert_eq!(
            detector.title(),
            String::from("Incorrect Order of Division and Multiplication")
        );
        assert_eq!(
            detector.description(),
            String::from("Division operations followed directly by multiplication operations can lead to precision loss due to the way integer arithmetic is handled in Solidity.")
        );
    }
}
