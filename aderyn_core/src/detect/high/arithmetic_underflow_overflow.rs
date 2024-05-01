use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::{ExtractAssignments, ExtractBinaryOperations, ExtractPragmaDirectives},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use semver::{Op, VersionReq};

#[derive(Default)]
pub struct ArithmeticUnderflowOverflowDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

fn version_req_below_0_8(version_req: &VersionReq) -> bool {
    // Simplified logic to check if version_req is below 0.8
    // Note: This might not cover all complex semver cases.
    if version_req.comparators.len() == 1 {
        let comparator = &version_req.comparators[0];
        match comparator.op {
            Op::Tilde | Op::Caret | Op::LessEq | Op::Greater | Op::GreaterEq | Op::Exact => {
                return comparator.major == 0 && comparator.minor < Some(8);
            }
            Op::Less => {
                return comparator.major == 0 && comparator.minor <= Some(8);
            }
            _ => {}
        }
    } else if version_req.comparators.len() == 2 {
        let comparator_2 = &version_req.comparators[1];
        match comparator_2.op {
            Op::Less => {
                return comparator_2.major == 0 && comparator_2.minor <= Some(8);
            }
            Op::LessEq | Op::Exact => {
                return comparator_2.major == 0 && comparator_2.minor < Some(8);
            }
            _ => {}
        }
    }

    false
}

impl IssueDetector for ArithmeticUnderflowOverflowDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for source_unit in context.source_units() {
            let pragma_directives = ExtractPragmaDirectives::from(source_unit).extracted;
            for pragma_directive in pragma_directives.iter() {
                let mut version_string = String::new();
                for literal in &pragma_directive.literals {
                    if literal == "solidity" {
                        continue;
                    }
                    if version_string.is_empty() && literal.contains("0.") {
                        version_string.push('=');
                    }
                    if version_string.len() > 5 && (literal == "<" || literal == "=") {
                        version_string.push(',');
                    }
                    version_string.push_str(literal);
                }

                let req = VersionReq::parse(&version_string)?;
                if version_req_below_0_8(&req) {
                    let assignments = ExtractAssignments::from(source_unit).extracted;
                    let binary_operations = ExtractBinaryOperations::from(source_unit).extracted;

                    assignments
                        .iter()
                        .filter(|assignment| {
                            assignment.operator == "+=" || assignment.operator == "-="
                        })
                        .for_each(|assignment| capture!(self, context, assignment));
                    binary_operations
                        .iter()
                        .filter(|binary_op| binary_op.operator == "+" || binary_op.operator == "-")
                        .for_each(|op| capture!(self, context, op));
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`SafeMath` library should be imported and used to prevent underflow and overflow in solidity compilers below `0.8.0`")
    }

    fn description(&self) -> String {
        String::from("Use `SafeMath` library for arithmetic operations to avoid underflow and/or overflow in solidity compilers below `0.8.0`")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ArithmeticUnderflowOverflow)
    }
}

#[cfg(test)]
mod arithmetic_underflow_overflow_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::ArithmeticUnderflowOverflowDetector;

    #[test]
    fn test_arithmetic_underflow_overflow() {
        let context = load_contract(
            "../tests/contract-playground/out/ArithmeticUnderflowOverflow.sol/ArithmeticUnderflowOverflow.json",
        );

        let mut detector = ArithmeticUnderflowOverflowDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 8);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("`SafeMath` library should be imported and used to prevent underflow and overflow in solidity compilers below `0.8.0`")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from("Use `SafeMath` library for arithmetic operations to avoid underflow and/or overflow in solidity compilers below `0.8.0`"));
    }
}
