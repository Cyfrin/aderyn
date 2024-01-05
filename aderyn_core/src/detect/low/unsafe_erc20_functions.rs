use std::{collections::BTreeMap, error::Error};

use crate::{
    context::{
        browser::ContextBrowser,
        loader::{ASTNode, ContextLoader},
    },
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC20FunctionsDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for UnsafeERC20FunctionsDetector {
    fn detect(
        &mut self,
        loader: &ContextLoader,
        _browser: &mut ContextBrowser,
    ) -> Result<bool, Box<dyn Error>> {
        for member_access in loader.member_accesses.keys() {
            if member_access.member_name == "transferFrom"
                || member_access.member_name == "approve"
                || member_access.member_name == "transfer"
            {
                self.found_instances.insert(
                    loader.get_node_sort_key(&ASTNode::MemberAccess(member_access.clone())),
                    member_access.src.clone(),
                );
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unsafe ERC20 Operations should not be used")
    }

    fn description(&self) -> String {
        String::from("ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod unsafe_erc20_functions_tests {
    use crate::{
        context::browser::ContextBrowser,
        detect::detector::{detector_test_helpers::load_contract, Detector},
    };

    use super::UnsafeERC20FunctionsDetector;

    #[test]
    fn test_unsafe_erc20_functions() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/DeprecatedOZFunctions.sol/DeprecatedOZFunctions.json",
        );
        let mut context_browser = ContextBrowser::default_from(&context_loader);
        context_browser.build_parallel();
        let mut detector = UnsafeERC20FunctionsDetector::default();
        let found = detector
            .detect(&context_loader, &mut context_browser)
            .unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct abi encode packed
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 5);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Unsafe ERC20 Operations should not be used")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library."
            )
        );
    }
}
