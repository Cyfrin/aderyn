use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct YulReturnDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for YulReturnDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for function_call in context.yul_function_calls() {
            if function_call.function_name.name == "return" {
                capture!(self, context, function_call);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Yul block contains `return`")
    }

    fn description(&self) -> String {
        String::from("This causes the transaction execution to halt, and nothing after that call will execute including code following the assembly block.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::YulReturn.to_string()
    }
}

#[cfg(test)]
mod yul_return_detector_tests {

    use crate::detect::{detector::IssueDetector, high::yul_return::YulReturnDetector};

    #[test]

    fn test_yul_return() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/YulReturn.sol",
        );

        let mut detector = YulReturnDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        assert_eq!(detector.instances().len(), 1);
    }
}
