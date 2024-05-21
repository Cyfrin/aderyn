use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, Visibility},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::count_identifiers_that_reference_an_id,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct UselessInternalFunctionDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UselessInternalFunctionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let internal_functions = context
            .function_definitions()
            .into_iter()
            .filter(|&function| {
                matches!(function.visibility, Visibility::Internal)
                    && !function.name.starts_with('_')
            });

        for internal_function in internal_functions {
            if count_identifiers_that_reference_an_id(context, internal_function.id) == 1 {
                capture!(self, context, internal_function);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Internal functions called only once can be inlined")
    }

    fn description(&self) -> String {
        String::from("Instead of separating the logic into a separate function, consider inlining the logic into the calling function. This can reduce the number of function calls and improve readability.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UselessInternalFunction)
    }
}

#[cfg(test)]
mod uselss_internal_function {
    use crate::detect::detector::IssueDetector;

    use super::UselessInternalFunctionDetector;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_useless_internal_functions_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/InternalFunctions.sol",
        );

        let mut detector = UselessInternalFunctionDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 1);

        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Internal functions called only once can be inlined")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from("Instead of separating the logic into a separate function, consider inlining the logic into the calling function. This can reduce the number of function calls and improve readability."));
    }
}
