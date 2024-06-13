use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the TemplateDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct LtInsteadOfLeDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for LtInsteadOfLeDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for binary_operation in context.binary_operations() {
            if binary_operation.operator == "<=" {
                capture!(self, context, binary_operation);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Use < instead of <=")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::LtInsteadOfLeDetector)
    }
}

#[cfg(test)]
mod template_detector_tests {
    use crate::detect::{detector::IssueDetector, low::lt_instead_of_le::LtInsteadOfLeDetector};

    #[test]
    fn test_template_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/LtInsteadOfLe.sol",
        );
        let mut detector = LtInsteadOfLeDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found the issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from("Use < instead of <="));
        // assert the description is correct
        assert_eq!(detector.description(), String::from(""));
    }
}
