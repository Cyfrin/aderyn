use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, NodeID};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct MisusedBooleanDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for MisusedBooleanDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN -
        // Check for
        // 1. if (… || true)`
        // 2. if(.. && false) (on either side of the || binary operations)

        for binary_operation in context.binary_operations() {
            if binary_operation.operator == "||" {
                if [
                    binary_operation.left_expression.as_ref(),
                    binary_operation.right_expression.as_ref(),
                ]
                .iter()
                .any(|&operand| {
                    if let Expression::Literal(literal) = operand {
                        if literal
                            .type_descriptions
                            .type_string
                            .as_ref()
                            .is_some_and(|type_string| type_string == "bool")
                        {
                            return literal.value.as_ref().is_some_and(|value| value == "true");
                        }
                    }
                    false
                }) {
                    capture!(self, context, binary_operation);
                }
            } else if binary_operation.operator == "&&" {
                if [
                    binary_operation.left_expression.as_ref(),
                    binary_operation.right_expression.as_ref(),
                ]
                .iter()
                .any(|&operand| {
                    if let Expression::Literal(literal) = operand {
                        if literal
                            .type_descriptions
                            .type_string
                            .as_ref()
                            .is_some_and(|type_string| type_string == "bool")
                        {
                            return literal.value.as_ref().is_some_and(|value| value == "false");
                        }
                    }
                    false
                }) {
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
        String::from("Misused boolean with logical operators")
    }

    fn description(&self) -> String {
        String::from("The patterns `if (… || true)` and `if (.. && false)` will always evaluate to true and false respectively.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::MisusedBoolean.to_string()
    }
}

#[cfg(test)]
mod misused_boolean_tests {
    use crate::detect::{detector::IssueDetector, high::misused_boolean::MisusedBooleanDetector};

    #[test]
    fn test_misused_boolean() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MisusedBoolean.sol",
        );

        let mut detector = MisusedBooleanDetector::default();
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
        assert_eq!(
            detector.title(),
            String::from("Misused boolean with logical operators")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("The patterns `if (… || true)` and `if (.. && false)` will always evaluate to true and false respectively.")
        );
    }
}
