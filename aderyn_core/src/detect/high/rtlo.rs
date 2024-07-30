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

#[derive(Default)]
pub struct RTLODetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for RTLODetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);

        for source_unit in context.source_units() {
            if let Some(content) = &source_unit.source {
                if content.contains('\u{202e}') {
                    capture!(self, context, source_unit);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("RTLO character detected in file. \\u{202e}")
    }

    fn description(&self) -> String {
        String::from("Right to left override character may be misledaing and cause potential attacks by visually misordering method arguments!")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::RTLO.to_string()
    }
}

#[cfg(test)]
mod rtlo_detector_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, high::rtlo::RTLODetector};

    #[test]
    #[serial]
    fn c() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/RTLO.sol",
        );

        let mut detector = RTLODetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("RTLO character detected in file. \\u{202e}")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Right to left override character may be misledaing and cause potential attacks by visually misordering method arguments!")
        );
    }
}
