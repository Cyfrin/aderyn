use std::{collections::BTreeMap, error::Error};

use crate::{
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct CentralizationRiskDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for CentralizationRiskDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for contract_definition in loader.contract_definitions.keys().filter(|cd| {
            cd.base_contracts.iter().any(|bc| {
                matches!(
                    bc.base_name.name.as_str(),
                    "Owned"
                        | "Ownable"
                        | "Ownable2Step"
                        | "AccessControl"
                        | "AccessControlCrossChain"
                        | "AccessControlEnumerable"
                        | "Auth"
                        | "RolesAuthority"
                        | "MultiRolesAuthority"
                )
            })
        }) {
            self.found_instances.insert(
                loader.get_node_sort_key(&ASTNode::ContractDefinition(contract_definition.clone())),
                contract_definition.src.clone(),
            );
        }

        for modifier_invocation in loader.modifier_invocations.keys().filter(|mi| {
            mi.modifier_name.name == "onlyOwner"
                || mi.modifier_name.name == "requiresAuth"
                || mi.modifier_name.name.contains("onlyRole")
        }) {
            self.found_instances.insert(
                loader.get_node_sort_key(&ASTNode::ModifierInvocation(modifier_invocation.clone())),
                modifier_invocation.src.clone(),
            );
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Centralization Risk for trusted owners")
    }

    fn description(&self) -> String {
        String::from("Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.")
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod centralization_risk_detector_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::CentralizationRiskDetector;

    #[test]
    fn test_centralization_risk_detector() {
        let context_loader =
            load_contract("../tests/contract-playground/out/AdminContract.sol/AdminContract.json");

        let mut detector = CentralizationRiskDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found a centralization risk
        assert!(found);
        // assert that the number of instances found is 3
        assert_eq!(detector.instances().len(), 3);
        // assert that the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Medium
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Centralization Risk for trusted owners")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds."
            )
        );
    }
}
