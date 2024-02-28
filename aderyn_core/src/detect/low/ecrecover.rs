use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct EcrecoverDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for EcrecoverDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers() {
            if identifier.name == "ecrecover" {
                capture!(self, context, identifier);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`ecrecover` is susceptible to signature malleability")
    }

    fn description(&self) -> String {
        String::from(
            "The `ecrecover` function is susceptible to signature malleability. \
            This means that the same message can be signed in multiple ways, \
            allowing an attacker to change the message signature without invalidating it. \
            This can lead to unexpected behavior in smart contracts, \
            such as the loss of funds or the ability to bypass access control. \
            Consider using OpenZeppelin's ECDSA library instead of the built-in function.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::Ecrecover)
    }
}

#[cfg(test)]
mod ecrecover_tests {

    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::EcrecoverDetector;

    #[test]
    fn test_ecrecover_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        );

        let mut detector = EcrecoverDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an ecrecover
        assert!(found);
        // assert that the detector found the correct ecrecover
        assert_eq!(detector.instances().len(), 1);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("`ecrecover` is susceptible to signature malleability")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "The `ecrecover` function is susceptible to signature malleability. \
                This means that the same message can be signed in multiple ways, \
                allowing an attacker to change the message signature without invalidating it. \
                This can lead to unexpected behavior in smart contracts, \
                such as the loss of funds or the ability to bypass access control. \
                Consider using OpenZeppelin's ECDSA library instead of the built-in function.",
            )
        );
    }
}
