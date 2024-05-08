use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;

use crate::detect::helpers::get_calls_and_delegate_calls;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ExternalCallsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ExternalCallsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in get_calls_and_delegate_calls(context) {
            capture!(self, context, member_access);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("High Issue Title")
    }

    fn description(&self) -> String {
        String::from("Description of the high issue.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        "high-issue-template".to_string()
    }
}

#[cfg(test)]
mod external_calls_detector_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        experimental::get_external_calls::ExternalCallsDetector,
    };

    #[test]
    fn test_external_calls_detector() {
        let context =
            load_contract("../tests/contract-playground/out/ExternalCalls.sol/ExternalCalls.json");

        let mut detector = ExternalCallsDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 8);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from("High Issue Title"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Description of the high issue.")
        );
    }
}
