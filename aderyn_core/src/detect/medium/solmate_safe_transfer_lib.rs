use std::{collections::BTreeMap, error::Error};

use crate::{
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SolmateSafeTransferLibDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for SolmateSafeTransferLibDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for import_directive in loader.import_directives.keys() {
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
                self.found_instances.insert(
                    loader.get_node_sort_key(&ASTNode::ImportDirective(import_directive.clone())),
                    import_directive.src.clone(),
                );
            }
        }

        Ok(!self.found_instances.is_empty())
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
        // assert that the detector found
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

    #[test]
    fn test_solmate_safe_transfer_lib_no_issue() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/ArbitraryTransferFrom.sol/ArbitraryTransferFrom.json",
        );

        let mut detector = SolmateSafeTransferLibDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found
        assert!(!found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 0);
    }
}
