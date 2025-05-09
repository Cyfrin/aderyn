use std::{collections::BTreeMap, error::Error};

use crate::ast::{BinaryOperation, NodeID, TypeDescriptions};

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::get_literal_value_or_constant_variable_value,
    },
};
use eyre::Result;
use solidity_integer_helper::{
    does_operation_make_sense_with_lhs_value, does_operation_make_sense_with_rhs_value,
};

#[derive(Default)]
pub struct TautologyOrContraditionDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for TautologyOrContraditionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for binary_operation in context.binary_operations() {
            if let Some(is_tautlogy_or_contradiction) =
                binary_operation.is_tautology_or_contradiction(context)
            {
                if is_tautlogy_or_contradiction {
                    capture!(self, context, binary_operation);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Tautology or Contradiction in comparison")
    }

    fn description(&self) -> String {
        String::from("The condition has been determined to be either always true or always false due to the integer range in which we're operating.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::TautologyOrContradiction.to_string()
    }
}

#[cfg(test)]
mod tautology_or_contradiction_tests {

    use crate::detect::{
        detector::IssueDetector, high::tautology_or_contradiction::TautologyOrContraditionDetector,
    };

    #[test]

    fn test_tautology_or_contradiction_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/TautologyOrContradiction.sol",
        );

        let mut detector = TautologyOrContraditionDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}

pub trait OperationIsTautologyOrContradiction {
    fn is_tautology_or_contradiction(&self, context: &WorkspaceContext) -> Option<bool>;
}

impl OperationIsTautologyOrContradiction for BinaryOperation {
    fn is_tautology_or_contradiction(&self, context: &WorkspaceContext) -> Option<bool> {
        if let (
            Some(TypeDescriptions { type_string: Some(lhs_type_string), .. }),
            Some(TypeDescriptions { type_string: Some(rhs_type_string), .. }),
            operator,
        ) = (
            self.left_expression.as_ref().type_descriptions(),
            self.right_expression.as_ref().type_descriptions(),
            self.operator.clone(),
        ) {
            let supported_operators = [">", ">=", "<", "<="];
            if supported_operators.into_iter().all(|op| op != operator) {
                return None;
            }

            if let Some(lhs_value) = get_literal_value_or_constant_variable_value(
                self.left_expression.get_node_id()?,
                context,
            ) {
                if let Some(makes_sense) =
                    does_operation_make_sense_with_lhs_value(&lhs_value, &operator, rhs_type_string)
                {
                    if !makes_sense {
                        return Some(true);
                    }
                }
            }

            if let Some(rhs_value) = get_literal_value_or_constant_variable_value(
                self.right_expression.get_node_id()?,
                context,
            ) {
                if let Some(makes_sense) =
                    does_operation_make_sense_with_rhs_value(lhs_type_string, &operator, &rhs_value)
                {
                    if !makes_sense {
                        return Some(true);
                    }
                }
            }
        }

        None
    }
}

pub mod solidity_integer_helper {
    use num_bigint::BigInt;
    use num_traits::One;
    use std::{error::Error, ops::Neg};

    /// This data type is big enough to handle the extreme values of uint256 and int256
    /// (Tests below)
    #[derive(PartialEq, Debug, Clone)]
    pub struct SolidityNumberRange {
        min_val: BigInt,
        max_val: BigInt,
    }

    impl SolidityNumberRange {
        fn fully_contains(&self, other_solidity_number_range: &SolidityNumberRange) -> bool {
            (self.min_val <= other_solidity_number_range.min_val)
                && (self.max_val >= other_solidity_number_range.max_val)
        }

        fn fully_excludes(&self, other_solidity_number_range: &SolidityNumberRange) -> bool {
            (other_solidity_number_range.max_val < self.min_val)
                || (other_solidity_number_range.min_val > self.max_val)
        }
    }

    /// Does this make sense? (boolean answer)
    /// ```no_code
    ///     if(?uintX, operator, value)
    /// ```
    ///
    /// Example (ways to call this function)
    ///
    /// Say x is uint8:
    /// Then, when we come across a binary operation like follows:
    ///     x >= 300
    /// we can determine if it makes sense by calling this function
    ///     does_operation_make_sense_with_rhs_value("uint8", ">=", "300")
    ///
    /// This function checks for the range of integer values of uint8 and returns true if it is
    /// neither a tautology nor a contradiction.
    ///
    /// Here, I define tautology as the condition where the range Ex: (>=300) FULLY COVERS the Range
    /// of Uint8 Contradiction: When the range Ex:(>=300) fully excludes the Range of Uint8
    ///
    /// Notice how in the above example, the value is on the right hand side.
    /// Hence this function is called "does_...rhs_value".
    pub fn does_operation_make_sense_with_rhs_value(
        type_string: &str,
        operator: &str,
        value: &str,
    ) -> Option<bool> {
        let allowed_range = get_range_for_type_string(type_string).ok()?;
        let allowed_min_val = allowed_range.min_val.clone();
        let allowed_max_val = allowed_range.max_val.clone();

        let value_as_big_int = BigInt::parse_bytes(value.as_bytes(), 10)?;

        // First and foremost if the value is out of range it's 100% either a tautology or a
        // contradiction. Hence, return false.
        if value_as_big_int < allowed_min_val || value_as_big_int > allowed_max_val {
            return Some(false);
        }
        // At this point, we know that the value we are comparing to, is in the allowed range.
        // Now, we can get the represented range, and see if it fully contains the allowed range
        // (tatutology) or fully excludes the allowed range (contradiction)
        let represented_range = {
            match operator {
                ">=" => Some(SolidityNumberRange {
                    min_val: value_as_big_int.clone(),
                    max_val: allowed_max_val.clone(),
                }),
                ">" => Some(SolidityNumberRange {
                    min_val: value_as_big_int.clone() + BigInt::one(),
                    max_val: allowed_max_val.clone(),
                }),
                "<=" => Some(SolidityNumberRange {
                    min_val: allowed_min_val.clone(),
                    max_val: value_as_big_int.clone(),
                }),
                "<" => Some(SolidityNumberRange {
                    min_val: allowed_min_val.clone(),
                    max_val: value_as_big_int.clone() - BigInt::one(),
                }),
                &_ => None,
            }
        };

        if let Some(represented_range) = represented_range {
            return Some(
                !(represented_range.fully_contains(&allowed_range)
                    || represented_range.fully_excludes(&allowed_range)),
            );
        }

        None
    }

    /// Does this make sense? (boolean answer)
    /// ```no_code
    ///     if(value, operator, uint8?)
    /// ```
    ///
    /// Take advantage of the above method by reusing code
    ///
    /// Example (ways to call this function)
    ///
    /// Say x is uint8:
    /// Then, when we come across a binary operation like follows:
    ///     300 >= x
    /// we can determine if it makes sense by calling this function
    ///     does_operation_make_sense_with_lhs_value("300", ">=", "uint8")
    ///
    /// Notice, here the value 300 is on the left hand side.
    pub fn does_operation_make_sense_with_lhs_value(
        value: &str,
        operator: &str,
        type_string: &str,
    ) -> Option<bool> {
        let inverse_operator = {
            match operator {
                ">=" => Some("<="),
                "<=" => Some(">="),
                ">" => Some("<"),
                "<" => Some(">"),
                _ => None,
            }
        }?;
        does_operation_make_sense_with_rhs_value(type_string, inverse_operator, value)
    }

    /// Accept the type string to calculate the range.
    pub fn get_range_for_type_string(
        type_string: &str,
    ) -> Result<SolidityNumberRange, Box<dyn Error>> {
        if type_string.starts_with("uint") {
            if let Some((_, num_of_bits)) = &type_string.split_once("uint") {
                let num_of_bits = num_of_bits.parse::<u32>()?;
                return Ok(SolidityNumberRange {
                    min_val: find_uint_min(num_of_bits),
                    max_val: find_uint_max(num_of_bits),
                });
            }
        } else if type_string.starts_with("int") {
            if let Some((_, num_of_bits)) = &type_string.split_once("int") {
                let num_of_bits = num_of_bits.parse::<u32>()?;
                return Ok(SolidityNumberRange {
                    min_val: find_int_min(num_of_bits),
                    max_val: find_int_max(num_of_bits),
                });
            }
        }
        Err("Invalid type string provided!".into())
    }

    // Helpers to calculate min and max for uint types like uint8, uint16, uint24, and so on . . .

    fn find_uint_max(num_of_bits: u32) -> BigInt {
        BigInt::parse_bytes(b"2", 10).unwrap().pow(num_of_bits) - BigInt::one()
    }

    fn find_uint_min(_: u32) -> BigInt {
        BigInt::ZERO
    }

    // Helpers to calculate min and max for int types like int8, int16, int24, and so on . . .

    fn find_int_max(num_of_bits: u32) -> BigInt {
        BigInt::parse_bytes(b"2", 10).unwrap().pow(num_of_bits - 1) - BigInt::one()
    }

    fn find_int_min(num_of_bits: u32) -> BigInt {
        BigInt::parse_bytes(b"2", 10).unwrap().pow(num_of_bits - 1).neg()
    }

    #[cfg(test)]
    mod test_num_bigint_primitives {

        use std::ops::Neg;

        use num_bigint::BigInt;
        use num_traits::{FromPrimitive, One};

        use crate::detect::high::tautology_or_contradiction::solidity_integer_helper::{
            does_operation_make_sense_with_rhs_value, find_int_max, find_int_min, find_uint_max,
        };

        use super::{
            does_operation_make_sense_with_lhs_value, get_range_for_type_string,
            SolidityNumberRange,
        };

        /*
            Tests to ensure that num_bigint crate holds the capacity to work with numbers
            in the range that Solidity language operates in.
        */

        #[test]
        fn test_2_raised_to_3() {
            let two_raised_to_three = BigInt::parse_bytes(b"2", 10).unwrap().pow(3);
            assert_eq!(two_raised_to_three, BigInt::from_u8(8).unwrap());
        }

        #[test]
        fn can_find_max_of_uint256() {
            // This test shows that we can calculate the biggest possible number in Solidity for
            // uint which is 2^256 - 1.
            // hence we conclude that because we can represent 2^256 - 1, we can easily cover all
            // the smaller variants of uint that is uint8, uint16, .... all the ay upto uint256
            // because they are lesser than 2^256 - 1
            let uint256_max = BigInt::parse_bytes(b"2", 10).unwrap().pow(256) - BigInt::one();
            assert_eq!(
                uint256_max,
                BigInt::parse_bytes(
                    b"115792089237316195423570985008687907853269984665640564039457584007913129639935",
                    10
                )
                .unwrap()
            );
        }

        #[test]
        fn can_find_min_of_int256() {
            let int_256_min = BigInt::parse_bytes(b"2", 10).unwrap().pow(255).neg();
            assert_eq!(
                int_256_min,
                BigInt::parse_bytes(
                    b"-57896044618658097711785492504343953926634992332820282019728792003956564819968",
                    10
                )
                .unwrap()
            );
        }

        #[test]
        fn can_find_max_of_int256() {
            let int_256_max = BigInt::parse_bytes(b"2", 10).unwrap().pow(255) - BigInt::one();
            assert_eq!(
                int_256_max,
                BigInt::parse_bytes(
                    b"57896044618658097711785492504343953926634992332820282019728792003956564819967",
                    10
                )
                .unwrap()
            );
        }

        /*
            Tests that our helper methods work which will ensure that min and max value can be calculated
            for every bit range from 0 to 256 .
        */

        #[test]
        fn helper_method_can_find_max_of_uint256() {
            let uint256_max = find_uint_max(256);
            assert_eq!(
                uint256_max,
                BigInt::parse_bytes(
                    b"115792089237316195423570985008687907853269984665640564039457584007913129639935",
                    10
                )
                .unwrap()
            );
        }

        #[test]
        fn helper_method_can_find_min_of_int256() {
            let int_256_min = find_int_min(256);
            assert_eq!(
                int_256_min,
                BigInt::parse_bytes(
                    b"-57896044618658097711785492504343953926634992332820282019728792003956564819968",
                    10
                )
                .unwrap()
            );
        }

        #[test]
        fn helper_method_can_find_max_of_int256() {
            let int_256_max = find_int_max(256);
            assert_eq!(
                int_256_max,
                BigInt::parse_bytes(
                    b"57896044618658097711785492504343953926634992332820282019728792003956564819967",
                    10
                )
                .unwrap()
            );
        }

        #[test]
        fn helper_method_can_find_range_for_int176() {
            let actual_range = get_range_for_type_string("int176").unwrap();
            let expected_range = SolidityNumberRange {
                min_val: BigInt::parse_bytes(
                    b"-47890485652059026823698344598447161988085597568237568",
                    10,
                )
                .unwrap(),
                max_val: BigInt::parse_bytes(
                    b"47890485652059026823698344598447161988085597568237567",
                    10,
                )
                .unwrap(),
            };
            assert_eq!(actual_range, expected_range);
        }

        #[test]
        fn helper_method_can_find_range_for_int248() {
            let actual_range = get_range_for_type_string("int248").unwrap();
            let expected_range = SolidityNumberRange {
                min_val: BigInt::parse_bytes(
                    b"-226156424291633194186662080095093570025917938800079226639565593765455331328",
                    10,
                )
                .unwrap(),
                max_val: BigInt::parse_bytes(
                    b"226156424291633194186662080095093570025917938800079226639565593765455331327",
                    10,
                )
                .unwrap(),
            };
            assert_eq!(actual_range, expected_range);
        }

        #[test]
        fn helper_method_can_find_range_for_int24() {
            let actual_range = get_range_for_type_string("int24").unwrap();
            let expected_range = SolidityNumberRange {
                min_val: BigInt::parse_bytes(b"-8388608", 10).unwrap(),
                max_val: BigInt::parse_bytes(b"8388607", 10).unwrap(),
            };
            assert_eq!(actual_range, expected_range);
        }

        #[test]
        fn helper_method_can_find_range_for_uint144() {
            let actual_range = get_range_for_type_string("uint144").unwrap();
            let expected_range = SolidityNumberRange {
                min_val: BigInt::ZERO,
                max_val: BigInt::parse_bytes(b"22300745198530623141535718272648361505980415", 10)
                    .unwrap(),
            };
            assert_eq!(actual_range, expected_range);
        }

        #[test]
        fn helper_method_can_find_range_for_uint232() {
            let actual_range = get_range_for_type_string("uint232").unwrap();
            let expected_range = SolidityNumberRange {
                min_val: BigInt::ZERO,
                max_val: BigInt::parse_bytes(
                    b"6901746346790563787434755862277025452451108972170386555162524223799295",
                    10,
                )
                .unwrap(),
            };
            assert_eq!(actual_range, expected_range);
        }

        #[test]
        fn helper_method_can_find_range_for_uint256() {
            let actual_range = get_range_for_type_string("uint256").unwrap();
            let expected_range = SolidityNumberRange {
                min_val: BigInt::ZERO,
                max_val: BigInt::parse_bytes(
                    b"115792089237316195423570985008687907853269984665640564039457584007913129639935",
                    10,
                )
                .unwrap(),
            };
            assert_eq!(actual_range, expected_range);
        }

        #[test]
        fn helper_method_can_find_range_for_uint8() {
            let actual_range = get_range_for_type_string("uint8").unwrap();
            let expected_range = SolidityNumberRange {
                min_val: BigInt::ZERO,
                max_val: BigInt::parse_bytes(b"255", 10).unwrap(),
            };
            assert_eq!(actual_range, expected_range);
        }

        #[test]
        fn does_operation_make_sense_lhs_uint8_part1() {
            let does_not_make_sense =
                !does_operation_make_sense_with_lhs_value("256", ">=", "uint8").unwrap();
            assert!(does_not_make_sense);
        }

        #[test]
        fn does_operation_make_sense_rhs_uint8_part1() {
            let does_make_sense =
                does_operation_make_sense_with_rhs_value("uint8", "<", "255").unwrap();
            assert!(does_make_sense);
        }

        #[test]
        fn does_operation_make_sense_lhs_uint8_part2() {
            let does_not_make_sense =
                !does_operation_make_sense_with_lhs_value("255", ">=", "uint8").unwrap();
            assert!(does_not_make_sense);
        }

        #[test]
        fn does_operation_make_sense_lhs_uint8_part3() {
            let does_make_sense =
                does_operation_make_sense_with_lhs_value("245", ">=", "uint8").unwrap();
            assert!(does_make_sense);
        }

        #[test]
        fn does_operation_make_sense_rhs_uint8_part2() {
            let does_make_sense =
                does_operation_make_sense_with_rhs_value("uint8", "<", "89").unwrap();
            assert!(does_make_sense);
        }

        #[test]
        fn does_operation_make_sense_rhs_uint256_part3() {
            let does_make_sense =
                does_operation_make_sense_with_rhs_value("uint256", ">", "0").unwrap();
            assert!(does_make_sense);
        }

        #[test]
        fn does_operation_make_sense_rhs_uint256_part4() {
            let does_not_make_sense =
                !does_operation_make_sense_with_rhs_value("uint256", ">=", "0").unwrap();
            assert!(does_not_make_sense);
        }

        #[test]
        fn does_operation_make_sense_rhs_uint72_part5() {
            let does_not_make_sense =
                !does_operation_make_sense_with_rhs_value("uint72", "<", "0").unwrap();
            assert!(does_not_make_sense);
        }

        #[test]
        fn does_operation_make_sense_rhs_uint8_part6() {
            let does_not_make_sense =
                !does_operation_make_sense_with_rhs_value("uint8", ">", "258").unwrap();
            assert!(does_not_make_sense);
        }
    }
}
