use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{NodeID, TypeName};

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
pub struct TemplateDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for TemplateDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // capture!(self, context, item);

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Low Issue Title")
    }

    fn description(&self) -> String {
        String::from("Description of the Low issue.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("Low-issue-template")
    }
}

#[cfg(test)]
mod arbitrary_transfer_from_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        Low::arbitrary_transfer_from::ArbitraryTransferFromDetector,
    };

    #[test]
    fn test_arbitrary_transfer_from_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/ArbitraryTransferFrom.sol/ArbitraryTransferFrom.json",
        );

        let mut detector = ArbitraryTransferFromDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is Low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from("Low Issue Title"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Description of the Low issue.")
        );
    }
}
