use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct AvoidAbiEncodePackedDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for AvoidAbiEncodePackedDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in context.member_accesses() {
            // If the member_access's member_name = "encodePacked", loop through the argument_types
            // and count how many of them contain any of the following in type_strings:
            // ["bytes ", "[]", "string"]
            // If the count is greater than 1, add the member_access to the found_abi_encode_packed
            // vector
            if member_access.member_name == "encodePacked" {
                let mut count = 0;
                let argument_types = member_access.argument_types.as_ref().unwrap();
                for argument_type in argument_types {
                    if argument_type.type_string.as_ref().unwrap().contains("bytes ")
                        || argument_type.type_string.as_ref().unwrap().contains("[]")
                        || argument_type.type_string.as_ref().unwrap().contains("string")
                    {
                        count += 1;
                    }
                }
                if count > 1 {
                    capture!(self, context, member_access);
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`abi.encodePacked()` Hash Collision")
    }

    fn description(&self) -> String {
        String::from(
            "abi.encodePacked() should not be used with dynamic types when passing the result to a hash function such as `keccak256()`. \
        Use `abi.encode()` instead which will pad items to 32 bytes, preventing hash collisions: https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode. \
        (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). \
        Unless there is a compelling reason, `abi.encode` should be preferred. If there is only one argument to `abi.encodePacked()` \
        it can often be cast to `bytes()` or `bytes32()` instead: https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739. \
        If all arguments are strings and or bytes, `bytes.concat()` should be used instead.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::AbiEncodePackedHashCollision)
    }
}

#[cfg(test)]
mod avoid_abi_encode_packed_tests {

    use crate::detect::detector::IssueDetector;

    use super::AvoidAbiEncodePackedDetector;

    #[test]
    fn test_avoid_abi_encode_packed_detectorby_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/KeccakContract.sol",
        );

        let mut detector = AvoidAbiEncodePackedDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 3);
    }
}
