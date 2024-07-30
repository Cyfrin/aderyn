use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{NodeID, YulExpression};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct IncorrectShiftOrderDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for IncorrectShiftOrderDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let yul_function_calls = context.yul_function_calls();
        for yul_function_call in yul_function_calls {
            if (yul_function_call.function_name.name == "shl"
                || yul_function_call.function_name.name == "shr")
                && yul_function_call
                    .arguments
                    .get(1)
                    .is_some_and(|n| matches!(n, YulExpression::YulLiteral(_)))
            {
                capture!(self, context, yul_function_call);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Incorrect Assembly Shift Parameter Order")
    }

    fn description(&self) -> String {
        String::from("Example: `shl(shifted, 4)` will shift the right constant `4` by `a` bits. The correct order is `shl(4, shifted)`.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::IncorrectShiftOrder.to_string()
    }
}

#[cfg(test)]
mod incorrect_shift_order_detector_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, high::IncorrectShiftOrderDetector};

    #[test]
    #[serial]
    fn test_incorrect_shift_order_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/IncorrectShift.sol",
        );

        let mut detector = IncorrectShiftOrderDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Incorrect Assembly Shift Parameter Order")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Example: `shl(shifted, 4)` will shift the right constant `4` by `a` bits. The correct order is `shl(4, shifted)`.")
        );
    }
}
