use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{ContractKind, NodeID};

use crate::capture;
use crate::context::browser::ExtractContractDefinitions;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct LibrarySharesFileWithContractDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for LibrarySharesFileWithContractDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);
        // capture!(self, context, item, "hint");

        for source_unit in context.source_units() {
            let all_contracts = ExtractContractDefinitions::from(source_unit).extracted;
            let number_of_non_library_contracts = all_contracts
                .iter()
                .filter(|c| c.kind != ContractKind::Library)
                .count();
            if number_of_non_library_contracts > 0 {
                for library_contract in all_contracts
                    .iter()
                    .filter(|c| c.kind == ContractKind::Library)
                {
                    capture!(self, context, library_contract);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Library shares file with contracts.")
    }

    fn description(&self) -> String {
        String::from("In Solidity, libraries are typically designed to be backward compatible, so they require flexible pragma versions to support a wide range of contracts. By placing libraries in separate files with floating pragmas, you ensure that they remain adaptable to different contract versions, while contracts in other files can stick to fixed pragma versions for stability.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::LibrarySharesFileWithContract.to_string()
    }
}

#[cfg(test)]
mod library_shares_file_with_contracts_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::library_shares_file_with_contracts::LibrarySharesFileWithContractDetector,
    };

    #[test]
    #[serial]
    fn test_library_shares_file_with_contracts_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/LibrarySharesFileWithContract.sol",
        );

        let mut detector = LibrarySharesFileWithContractDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
