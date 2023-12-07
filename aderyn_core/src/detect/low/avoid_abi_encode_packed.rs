use std::{collections::BTreeMap, error::Error};

use crate::{
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct AvoidAbiEncodePackedDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for AvoidAbiEncodePackedDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for member_access in loader.get_member_accesses() {
            // If the member_access's member_name = "encodePacked", loop through the argument_types and count how many of them contain any of the following in type_strings:
            // ["bytes ", "[]", "string"]
            // If the count is greater than 1, add the member_access to the found_abi_encode_packed vector
            if member_access.member_name == "encodePacked" {
                let mut count = 0;
                let argument_types = member_access.argument_types.as_ref().unwrap();
                for argument_type in argument_types {
                    if argument_type
                        .type_string
                        .as_ref()
                        .unwrap()
                        .contains("bytes ")
                        || argument_type.type_string.as_ref().unwrap().contains("[]")
                        || argument_type
                            .type_string
                            .as_ref()
                            .unwrap()
                            .contains("string")
                    {
                        count += 1;
                    }
                }
                if count > 1 {
                    self.found_instances.insert(
                        loader.get_node_sort_key(&ASTNode::MemberAccess(member_access.clone())),
                        member_access.src.clone(),
                    );
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`")
    }

    fn description(&self) -> String {
        String::from(
            "Use `abi.encode()` instead which will pad items to 32 bytes, which will [prevent hash collisions](https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode) (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). Unless there is a compelling reason, `abi.encode` should be preferred. If there is only one argument to `abi.encodePacked()` it can often be cast to `bytes()` or `bytes32()` [instead](https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739).\nIf all arguments are strings and or bytes, `bytes.concat()` should be used instead.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod avoid_abi_encode_packed_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::AvoidAbiEncodePackedDetector;

    #[test]
    fn test_avoid_abi_encode_packed_detector() {
        let context_loader =
            load_contract("./tests/contract-playground/out/KeccakContract.sol/KeccakContract.json");
        let mut detector = AvoidAbiEncodePackedDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct abi encode packed
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 3);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from(
                "`abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`"
            )
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Use `abi.encode()` instead which will pad items to 32 bytes, which will [prevent hash collisions](https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode) (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). Unless there is a compelling reason, `abi.encode` should be preferred. If there is only one argument to `abi.encodePacked()` it can often be cast to `bytes()` or `bytes32()` [instead](https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739).\nIf all arguments are strings and or bytes, `bytes.concat()` should be used instead.",
            )
        );
    }
}
