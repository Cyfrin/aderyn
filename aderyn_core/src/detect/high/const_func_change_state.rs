use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{NodeID, StateMutability};

use crate::capture;
use crate::context::browser::ApproximateStorageChangeFinder;
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the ConstantFunctionChangingStateDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct ConstantFunctionChangingStateDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ConstantFunctionChangingStateDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for func in helpers::get_implemented_external_and_public_functions(context) {
            // Rule applies to only view functions, so ignore the rest
            if func.state_mutability() != &StateMutability::View {
                continue;
            }
            // Check for state variable changes
            let finder = ApproximateStorageChangeFinder::from(context, func);
            if finder.state_variables_have_been_manipulated() {
                capture!(self, context, func);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Constant functions changing state")
    }

    fn description(&self) -> String {
        String::from("Function is declared as a constant function but it changes state. Ensure that the attributes of contract compiled prior to 0.5 are correct.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::ConstantFunctionChangingState.to_string()
    }
}

#[cfg(test)]
mod constant_func_changing_state {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        high::const_func_change_state::ConstantFunctionChangingStateDetector,
    };

    #[test]
    #[serial]
    fn test_constant_function_changing_state() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ConstFuncChangeState.sol",
        );

        let mut detector = ConstantFunctionChangingStateDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
    }
}
