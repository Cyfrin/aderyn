use std::{collections::BTreeMap, error::Error};

use crate::{
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SolmateSafeTransferLibDetector {
    found_solmate_import: bool,
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for SolmateSafeTransferLibDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for import_directive in loader.import_directives.keys() {
            if !self.found_solmate_import {
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
                    self.found_solmate_import = true;
                }
            }
        }

        for member_access in loader.member_accesses.keys() {
            // If the member access member_name is any of the following names, add it to the list of found
            // found_transfer_usage vector: ["safeTransfer", "safeTransferFrom", "safeApprove"]
            if member_access.member_name == "safeTransfer"
                || member_access.member_name == "safeTransferFrom"
                || member_access.member_name == "safeApprove"
            {
                self.found_instances.insert(
                    loader.get_node_sort_key(&ASTNode::MemberAccess(member_access.clone())),
                    member_access.src.clone(),
                );
            }
        }

        if self.found_solmate_import && !self.found_instances.is_empty() {
            return Ok(true);
        }

        Ok(false)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Solmate\'s SafeTransferLib does not check for token contract\'s existence")
    }

    fn description(&self) -> String {
        String::from("There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.\nhttps://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 \n`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`\n")
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod solmate_safe_transfer_lib_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, Detector},
        medium::solmate_safe_transfer_lib::SolmateSafeTransferLibDetector,
    };

    #[test]
    fn test_solmate_safe_transfer_lib() {
        let context_loader =
            load_contract("../tests/contract-playground/out/T11sTranferer.sol/T11sTranferer.json");
        let mut detector = SolmateSafeTransferLibDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found a delegate call in a loop
        assert!(found);
        // assert that the detector found the correct number of instances (1)
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Medium
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
}
