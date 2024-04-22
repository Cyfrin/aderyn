use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC20FunctionsDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnsafeERC20FunctionsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in context.member_accesses() {
            if member_access.member_name == "transferFrom"
                || member_access.member_name == "approve"
                || member_access.member_name == "transfer"
            {
                capture!(self, context, member_access);
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

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnsafeERC20Functions)
    }
}

#[cfg(test)]
mod unsafe_erc20_functions_tests {
    use crate::detect::detector::IssueDetector;

    use super::UnsafeERC20FunctionsDetector;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_unsafe_erc20_functions_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_contract(
            "../tests/contract-playground/out/DeprecatedOZFunctions.sol/DeprecatedOZFunctions.json",
        );

        let mut detector = UnsafeERC20FunctionsDetector::default();
        let found = detector.detect(&context).unwrap();
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
