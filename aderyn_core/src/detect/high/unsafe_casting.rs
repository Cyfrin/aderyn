use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, FunctionCall, FunctionCallKind, NodeID, NodeType};

use crate::capture;
use crate::context::browser::{
    ExtractBinaryOperations, ExtractIdentifiers, GetClosestAncestorOfTypeX,
};
use crate::context::workspace_context::ASTNode;
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
        for function_call in context.function_calls() {
            if function_call.kind == FunctionCallKind::TypeConversion {
                let casting_to_type = match function_call.type_descriptions.type_string.as_ref() {
                    Some(t) => t,
                    None => continue,
                };

                let first_arg = function_call.arguments.first();
                let identifier_id = match first_arg {
                    Some(Expression::Identifier(identifier)) => identifier.referenced_declaration,
                    _ => continue,
                };

                if let Expression::ElementaryTypeNameExpression(to_expression) =
                    &*function_call.expression
                {
                    if let Some(argument_types) = &to_expression.argument_types {
                        let casting_from_type = match argument_types
                            .first()
                            .and_then(|arg| arg.type_string.as_ref())
                        {
                            Some(t) => t,
                            None => continue,
                        };

                        let casting_map = if casting_from_type.contains("uint") {
                            &UINT_CASTING_MAP
                        } else if casting_from_type.contains("int")
                            && !casting_from_type.contains("uint")
                        {
                            &INT_CASTING_MAP
                        } else if casting_from_type.contains("bytes")
                            && !casting_from_type.contains("bytes ")
                        {
                            &BYTES32_CASTING_MAP
                        } else {
                            continue;
                        };

                        handle_casting(
                            self,
                            context,
                            function_call,
                            casting_from_type,
                            casting_to_type,
                            casting_map,
                            identifier_id,
                        );
                    }
                }
            }
        }
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

fn handle_casting(
    detector: &mut UnsafeCastingDetector,
    context: &WorkspaceContext,
    function_call: &FunctionCall,
    casting_from_type: &str,
    casting_to_type: &str,
    casting_map: &phf::Map<&'static str, usize>,
    identifier_id: NodeID,
) {
    if let Some(casting_from_type_index) = casting_map.get(casting_from_type) {
        // More precise checks for type casting
        let is_valid_cast = match casting_map.get(casting_to_type) {
            Some(casting_to_type_index) => casting_from_type_index > casting_to_type_index,
            None => false,
        };

        if is_valid_cast {
            // Check if there are any binary operations that involve the identifier
            if !has_binary_operation_checks(
                function_call.closest_ancestor_of_type(context, NodeType::ContractDefinition),
                &identifier_id,
            ) {
                capture!(detector, context, function_call);
            }
        }
    }
}

fn has_binary_operation_checks(
    contract: Option<&ASTNode>,
    identifier_reference_declaration_id: &NodeID,
) -> bool {
    if let Some(ASTNode::ContractDefinition(contract)) = contract {
        return ExtractBinaryOperations::from(contract)
            .extracted
            .iter()
            .any(|binary_operation| {
                ExtractIdentifiers::from(binary_operation)
                    .extracted
                    .into_iter()
                    .any(|identifier| {
                        identifier.referenced_declaration == *identifier_reference_declaration_id
                    })
            });
    }
    false
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
    use crate::detect::{detector::IssueDetector, high::UnsafeCastingDetector};

    #[test]
    fn test_unsafe_casting_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/Casting.sol",
        );

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
