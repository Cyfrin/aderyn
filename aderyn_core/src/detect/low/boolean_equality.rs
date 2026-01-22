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
pub struct BooleanEqualityDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for BooleanEqualityDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for binary_operation in context.binary_operations() {
            if binary_operation.operator == "=="
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
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Boolean equality is not required")
    }

    fn description(&self) -> String {
        String::from(
            "If `x` is a boolean, use `if(x)` and `if(!x)` instead of `if(x == true)` or `if(x == false)`.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::BooleanEquality)
    }
}

#[cfg(test)]
mod boolean_equality_tests {

    use crate::detect::{detector::IssueDetector, low::boolean_equality::BooleanEqualityDetector};

    #[test]
    fn test_boolean_equality_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/BooleanEquality.sol",
        );

        let mut detector = BooleanEqualityDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 4);
    }
}
