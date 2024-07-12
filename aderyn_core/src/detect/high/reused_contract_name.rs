use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::hash::Hash;

use crate::ast::{NodeID, TypeName};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ReusedContractNameDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ReusedContractNameDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut contract_names_seen: HashSet<&str> = HashSet::new();

        for contract in context.contract_definitions() {
            if contract_names_seen.contains(&contract.name.as_str()) {
                capture!(self, context, contract);
            } else {
                contract_names_seen.insert(&contract.name);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Contract Name Reused in Different Files")
    }

    fn description(&self) -> String {
        String::from("When compiling contracts with certain development frameworks (for example: Truffle), having contracts with the same name across different files can lead to one being overwritten.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("high-issue-template")
    }
}

#[cfg(test)]
mod reused_contract_name_detector_tests {
    use crate::detect::{
        detector::IssueDetector, high::ReusedContractNameDetector,
        test_utils::load_multiple_solidity_source_units_into_single_context,
    };

    #[test]
    fn test_reused_contract_name_detector() {
        let context = load_multiple_solidity_source_units_into_single_context(&[
            "../tests/contract-playground/src/reused_contract_name/ContractA.sol",
            "../tests/contract-playground/src/reused_contract_name/ContractB.sol",
        ]);

        let mut detector = ReusedContractNameDetector::default();
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
            String::from("Contract Name Reused in Different Files")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("When compiling contracts with certain development frameworks (for example: Truffle), having contracts with the same name across different files can lead to one being overwritten.")
        );
    }
}
