use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace::WorkspaceContext,
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::is_constant_boolean,
    },
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
        for binary_operation in context.binary_operations() {
            if (binary_operation.operator == "||" || binary_operation.operator == "&&")
                && [
                    binary_operation.left_expression.as_ref(),
                    binary_operation.right_expression.as_ref(),
                ]
                .iter()
                .any(|&operand| is_constant_boolean(context, operand))
            {
                capture!(self, context, binary_operation);
            }
        }

        for if_statement in context
            .if_statements()
            .iter()
            .filter(|statement| is_constant_boolean(context, &statement.condition))
        {
            capture!(self, context, if_statement);
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
        String::from(
            "The patterns `if (… || true)` and `if (.. && false)` will always evaluate to true and false respectively.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::MisusedBoolean)
    }
}

#[cfg(test)]
mod misused_boolean_tests {

    use crate::detect::{detector::IssueDetector, high::misused_boolean::MisusedBooleanDetector};

    #[test]
    fn test_misused_boolean_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MisusedBoolean.sol",
        );

        let mut detector = MisusedBooleanDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 10);
    }
}
