use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct CentralizationRiskDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for CentralizationRiskDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract_definition in context.contract_definitions().iter() {
            for bc in contract_definition.base_contracts.iter() {
                if matches!(
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
                ) {
                    capture!(self, context, bc);
                }
            }
        }

        for modifier_invocation in context.modifier_invocations().iter().filter(|&&mi| {
            mi.modifier_name.name == "onlyOwner"
                || mi.modifier_name.name == "requiresAuth"
                || mi.modifier_name.name.contains("onlyRole")
        }) {
            capture!(self, context, modifier_invocation);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Centralization Risk for trusted owners")
    }

    fn description(&self) -> String {
        String::from("Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod centralization_risk_detector_tests {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::CentralizationRiskDetector;

    #[test]
    #[serial]
    fn test_centralization_risk_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/AdminContract.sol",
        );

        let mut detector = CentralizationRiskDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found a centralization risk
        assert!(found);
        // assert that the number of instances found is 3
        assert_eq!(detector.instances().len(), 3);
        // assert that the severity is Low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
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
