use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{Expression, NodeID},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct NonTypesafeAbiEncodeDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for NonTypesafeAbiEncodeDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in context.member_accesses() {
            if let Expression::Identifier(identifier) = &member_access.expression.as_ref() {
                if identifier.name == "abi" {
                    if matches!(
                        member_access.member_name.as_ref(),
                        "encodeWithSignature" | "encodeWithSelector"
                    ) {
                        capture!(self, context, member_access);
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from(
            "Use abi.encodeCall for typesafety instead of abi.encodeWithSignature (or) abi.encodeWithSelector ",
        )
    }

    fn description(&self) -> String {
        String::from("Solidity compiler can type check for functions at compile time when abi.encodeCall is used. That doesn't happen in cases where abi.encodeWithSignature (or) abi.encodeWithSelector is used.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::NonTypesafeAbiEncode)
    }
}

#[cfg(test)]
mod non_typesafe_abi_encode_detector {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    #[test]
    fn test_non_typesafe_abi_encode_detector() {
        let context =
            load_contract("../tests/contract-playground/out/ABIEncode.sol/AbiEncode.json");

        let mut detector = super::NonTypesafeAbiEncodeDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that it found something
        assert!(found);
        // assert that the number of instances is correct
        assert_eq!(detector.instances().len(), 2);

        println!("{:?}", detector.instances());
    }
}
