use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, FunctionCallKind, NodeID};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;
use phf::phf_map;

#[derive(Default)]
pub struct UnsafeCastingDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnsafeCastingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        context
            .function_calls()
            .into_iter()
            .for_each(|function_call| {
                if function_call.kind == FunctionCallKind::TypeConversion {
                    let casting_to_type = function_call
                        .type_descriptions
                        .type_string
                        .as_ref()
                        .unwrap();

                    if let Expression::ElementaryTypeNameExpression(to_expression) =
                        &*function_call.expression
                    {
                        if let Some(argument_types) = &to_expression.argument_types {
                            let casting_from_type = argument_types[0].type_string.as_ref().unwrap();

                            if casting_from_type.contains("uint") {
                                if let Some(casting_from_type_index) =
                                    UINT_CASTING_MAP.get(casting_from_type)
                                {
                                    // if casting from a larger uint to a smaller uint
                                    if casting_to_type.contains("uint")
                                        && casting_from_type_index
                                            > UINT_CASTING_MAP.get(casting_to_type).unwrap()
                                    {
                                        capture!(self, context, function_call);
                                    }
                                }
                            } else if casting_from_type.contains("int")
                                && !casting_from_type.contains("uint")
                            {
                                if let Some(casting_from_type_index) =
                                    INT_CASTING_MAP.get(casting_from_type)
                                {
                                    // if casting from a larger int to a smaller int
                                    if casting_to_type.contains("int")
                                        && !casting_to_type.contains("uint")
                                        && casting_from_type_index
                                            > INT_CASTING_MAP.get(casting_to_type).unwrap()
                                    {
                                        capture!(self, context, function_call);
                                    }
                                }
                            } else if casting_from_type.contains("bytes")
                                && !casting_from_type.contains("bytes ")
                            {
                                if let Some(casting_from_type_index) =
                                    BYTES32_CASTING_MAP.get(casting_from_type)
                                {
                                    // if casting from a larger bytes32 to a smaller bytes32
                                    if casting_to_type.contains("bytes")
                                        && !casting_to_type.contains("bytes ")
                                        && casting_from_type_index
                                            > BYTES32_CASTING_MAP.get(casting_to_type).unwrap()
                                    {
                                        capture!(self, context, function_call);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Unsafe Casting")
    }

    fn description(&self) -> String {
        String::from("Downcasting int/uints in Solidity can be unsafe due to the potential for data loss and unintended behavior.\
        When downcasting a larger integer type to a smaller one (e.g., uint256 to uint128), the value may exceed the range of the target type,\
        leading to truncation and loss of significant digits. Use OpenZeppelin's SafeCast library to safely downcast integers.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnsafeCastingDetector)
    }
}

static UINT_CASTING_MAP: phf::Map<&'static str, usize> = phf_map! {
    "uint8" => 0,
    "uint16" => 1,
    "uint24" => 2,
    "uint32" => 3,
    "uint40" => 4,
    "uint48" => 5,
    "uint56" => 6,
    "uint64" => 7,
    "uint72" => 8,
    "uint80" => 9,
    "uint88" => 10,
    "uint96" => 11,
    "uint104" => 12,
    "uint112" => 13,
    "uint120" => 14,
    "uint128" => 15,
    "uint136" => 16,
    "uint144" => 17,
    "uint152" => 18,
    "uint160" => 19,
    "uint168" => 20,
    "uint176" => 21,
    "uint184" => 22,
    "uint192" => 23,
    "uint200" => 24,
    "uint208" => 25,
    "uint216" => 26,
    "uint224" => 27,
    "uint232" => 28,
    "uint240" => 29,
    "uint248" => 30,
    "uint256" => 31,
};

static INT_CASTING_MAP: phf::Map<&'static str, usize> = phf_map! {
    "int8" => 0,
    "int16" => 1,
    "int24" => 2,
    "int32" => 3,
    "int40" => 4,
    "int48" => 5,
    "int56" => 6,
    "int64" => 7,
    "int72" => 8,
    "int80" => 9,
    "int88" => 10,
    "int96" => 11,
    "int104" => 12,
    "int112" => 13,
    "int120" => 14,
    "int128" => 15,
    "int136" => 16,
    "int144" => 17,
    "int152" => 18,
    "int160" => 19,
    "int168" => 20,
    "int176" => 21,
    "int184" => 22,
    "int192" => 23,
    "int200" => 24,
    "int208" => 25,
    "int216" => 26,
    "int224" => 27,
    "int232" => 28,
    "int240" => 29,
    "int248" => 30,
    "int256" => 31,
};

static BYTES32_CASTING_MAP: phf::Map<&'static str, usize> = phf_map! {
    "bytes1" => 0,
    "bytes2" => 1,
    "bytes3" => 2,
    "bytes4" => 3,
    "bytes5" => 4,
    "bytes6" => 5,
    "bytes7" => 6,
    "bytes8" => 7,
    "bytes9" => 8,
    "bytes10" => 9,
    "bytes11" => 10,
    "bytes12" => 11,
    "bytes13" => 12,
    "bytes14" => 13,
    "bytes15" => 14,
    "bytes16" => 15,
    "bytes17" => 16,
    "bytes18" => 17,
    "bytes19" => 18,
    "bytes20" => 19,
    "bytes21" => 20,
    "bytes22" => 21,
    "bytes23" => 22,
    "bytes24" => 23,
    "bytes25" => 24,
    "bytes26" => 25,
    "bytes27" => 26,
    "bytes28" => 27,
    "bytes29" => 28,
    "bytes30" => 29,
    "bytes31" => 30,
    "bytes32" => 31,
};

#[cfg(test)]
mod unsafe_casting_detector_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        high::UnsafeCastingDetector,
    };

    #[test]
    fn test_unsafe_casting_detector() {
        let context = load_contract("../tests/contract-playground/out/Casting.sol/Casting.json");

        let mut detector = UnsafeCastingDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 93);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
    }
}
