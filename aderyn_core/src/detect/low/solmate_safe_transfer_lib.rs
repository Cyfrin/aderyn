use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SolmateSafeTransferLibDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for SolmateSafeTransferLibDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for import_directive in context.import_directives() {
            // If the import directive absolute_path contains the strings "solmate" and "SafeTransferLib", flip the found_solmate_import flag to true
            if import_directive
                .absolute_path
                .as_ref()
                .unwrap()
                .contains("solmate")
                && import_directive
                    .absolute_path
                    .as_ref()
                    .unwrap()
                    .contains("SafeTransferLib")
            {
                capture!(self, context, import_directive);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Solmate\'s SafeTransferLib does not check for token contract\'s existence")
    }

    fn description(&self) -> String {
        String::from("There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.\nhttps://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 \n`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`\n")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::SolmateSafeTransferLib)
    }
}

#[cfg(test)]
mod solmate_safe_transfer_lib_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, low::SolmateSafeTransferLibDetector};

    #[test]
    #[serial]
    fn test_solmate_safe_transfer_lib_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/T11sTranferer.sol",
        );

        let mut detector = SolmateSafeTransferLibDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found
        assert!(found);
        // assert that the detector found the correct number of instances (1)
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is Low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from(
                "Solmate\'s SafeTransferLib does not check for token contract\'s existence"
            )
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.\nhttps://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 \n`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`\n"
            )
        );
    }

    #[test]
    #[serial]
    fn test_solmate_safe_transfer_lib_no_issue_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ArbitraryTransferFrom.sol",
        );

        let mut detector = SolmateSafeTransferLibDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found
        assert!(!found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 0);
    }
}
