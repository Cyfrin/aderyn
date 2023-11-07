use std::error::Error;

use crate::{
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct EcrecoverDetector {
    found_ecrecover: Vec<Option<ASTNode>>,
}

impl Detector for EcrecoverDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for identifier in loader.get_identifiers() {
            if identifier.name == "ecrecover" {
                self.found_ecrecover
                    .push(Some(ASTNode::Identifier(identifier.clone())));
            }
        }
        Ok(!self.found_ecrecover.is_empty())
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

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_ecrecover.clone()
    }
}

#[cfg(test)]
mod ecrecover_tests {

    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::EcrecoverDetector;

    #[test]
    fn test_ecrecover_detector() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        );
        let mut detector = EcrecoverDetector::default();
        let found = detector.detect(&context_loader).unwrap();
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
