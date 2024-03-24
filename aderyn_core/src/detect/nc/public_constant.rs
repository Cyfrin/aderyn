use std::collections::BTreeMap;

use crate::{
    ast::{NodeID, Visibility},
    capture,
    context::{browser::ExtractVariableDeclarations, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};

#[derive(Default)]
pub struct PublicConstantDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for PublicConstantDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn std::error::Error>> {
        for contract in context.contract_definitions() {
            let num_public_constant_variables = ExtractVariableDeclarations::from(contract)
                .extracted
                .into_iter()
                .filter(|x| x.constant && matches!(x.visibility, Visibility::Public))
                .count();
            if num_public_constant_variables > 1 {
                capture!(self, context, contract);
            }
        }

        Ok(!self.found_instances.is_empty())
    }
    fn title(&self) -> String {
        String::from("Public constants can be replaced by single getter function")
    }

    fn description(&self) -> String {
        String::from("If there is more than 1 public constant in a contract, they all can be marked private and exposed via a single getter function to save deployment cost.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::PublicConstant)
    }
}

#[cfg(test)]
mod public_constant_detector {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::PublicConstantDetector;

    #[test]
    fn test_public_constants_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/ConstantsLiterals.sol/ConstantsLiterals.json",
        );

        let mut detector = PublicConstantDetector::default();
        // assert that the detector finds something
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Public constants can be replaced by single getter function")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from("If there is more than 1 public constant in a contract, they all can be marked private and exposed via a single getter function to save deployment cost."));
    }
}
