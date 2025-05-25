use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC20OperationDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnsafeERC20OperationDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in context.member_accesses() {
            if member_access.expression.as_ref().type_descriptions().is_some_and(|desc| {
                desc.type_string.as_ref().is_some_and(|type_string| type_string.contains("ERC20"))
            }) && member_access.member_name == "transferFrom"
                || member_access.member_name == "approve"
                || member_access.member_name == "transfer"
            {
                capture!(self, context, member_access);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unsafe ERC20 Operation")
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
        format!("{}", IssueDetectorNamePool::UnsafeERC20Operation)
    }
}

#[cfg(test)]
mod unsafe_erc20_functions_tests {
    use crate::detect::detector::IssueDetector;

    use super::UnsafeERC20OperationDetector;

    #[test]

    fn test_unsafe_erc20_functions_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DeprecatedOZFunctions.sol",
        );

        let mut detector = UnsafeERC20OperationDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 5);
    }
}
