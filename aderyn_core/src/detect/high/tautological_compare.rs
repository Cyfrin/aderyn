use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, Identifier, MemberAccess, NodeID};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers::get_literal_value_or_constant_variable_value;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct TautologicalCompareDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for TautologicalCompareDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for binary_operation in context.binary_operations().into_iter().filter(|binary_op| {
            ["&&", "||", ">=", ">", "<=", "<"]
                .into_iter()
                .any(|op| op == binary_op.operator)
        }) {
            let orientations = [
                (
                    binary_operation.left_expression.as_ref(),
                    binary_operation.right_expression.as_ref(),
                ),
                (
                    binary_operation.right_expression.as_ref(),
                    binary_operation.left_expression.as_ref(),
                ),
            ];

            match (
                binary_operation.left_expression.as_ref(),
                binary_operation.right_expression.as_ref(),
            ) {
                (
                    Expression::Identifier(Identifier {
                        referenced_declaration: Some(id0),
                        ..
                    }),
                    Expression::Identifier(Identifier {
                        referenced_declaration: Some(id1),
                        ..
                    }),
                )
                | (
                    Expression::MemberAccess(MemberAccess {
                        referenced_declaration: Some(id0),
                        ..
                    }),
                    Expression::MemberAccess(MemberAccess {
                        referenced_declaration: Some(id1),
                        ..
                    }),
                ) => {
                    if *id0 == *id1 {
                        capture!(self, context, binary_operation);
                    } else {
                        let v0 = get_literal_value_or_constant_variable_value(*id0, context);
                        let v1 = get_literal_value_or_constant_variable_value(*id1, context);

                        let is_equal_in_value = match (v0, v1) {
                            (Some(ref s0), Some(ref s1)) => s0 == s1,
                            _ => false,
                        };

                        if is_equal_in_value {
                            capture!(self, context, binary_operation);
                        }
                    }
                }
                _ => (),
            };

            for (lhs, rhs) in orientations {
                match (lhs, rhs) {
                    (
                        Expression::Identifier(Identifier {
                            referenced_declaration: Some(id0),
                            ..
                        }),
                        Expression::MemberAccess(MemberAccess {
                            referenced_declaration: Some(id1),
                            ..
                        }),
                    ) => {
                        if *id0 == *id1 {
                            capture!(self, context, binary_operation);
                        } else {
                            let v0 = get_literal_value_or_constant_variable_value(*id0, context);
                            let v1 = get_literal_value_or_constant_variable_value(*id1, context);

                            let is_equal_in_value = match (v0, v1) {
                                (Some(ref s0), Some(ref s1)) => s0 == s1,
                                _ => false,
                            };

                            if is_equal_in_value {
                                capture!(self, context, binary_operation);
                            }
                        }
                    }
                    (
                        Expression::Literal(literal),
                        Expression::MemberAccess(MemberAccess {
                            referenced_declaration: Some(id1),
                            ..
                        }),
                    )
                    | (
                        Expression::Literal(literal),
                        Expression::Identifier(Identifier {
                            referenced_declaration: Some(id1),
                            ..
                        }),
                    ) => {
                        let v0 = literal.value.to_owned();
                        let v1 = get_literal_value_or_constant_variable_value(*id1, context);

                        let is_equal_in_value = match (v0, v1) {
                            (Some(ref s0), Some(ref s1)) => s0 == s1,
                            _ => false,
                        };

                        if is_equal_in_value {
                            capture!(self, context, binary_operation);
                        }
                    }
                    _ => (),
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Tautological comparison.")
    }

    fn description(&self) -> String {
        String::from("The left hand side and the right hand side of the binary operation has the same value. This makes the condition always true or always false.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::TautologicalCompare.to_string()
    }
}

#[cfg(test)]
mod tautological_compare_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::tautological_compare::TautologicalCompareDetector,
    };

    #[test]
    #[serial]
    fn test_tatulogical_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/TautologicalCompare.sol",
        );

        let mut detector = TautologicalCompareDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from("Tautological comparison."));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("The left hand side and the right hand side of the binary operation has the same value. This makes the condition always true or always false.")
        );
    }
}
