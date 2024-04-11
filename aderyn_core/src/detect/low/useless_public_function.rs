use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{FunctionKind, NodeID, Visibility},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::count_identifiers_that_reference_an_id,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct UselessPublicFunctionDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UselessPublicFunctionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let unreferenced_public_functions =
            context
                .function_definitions()
                .into_iter()
                .filter(|&function| {
                    matches!(function.visibility, Visibility::Public)
                        && !matches!(function.kind, FunctionKind::Constructor)
                        && count_identifiers_that_reference_an_id(context, function.id) == 0
                });

        for unreferenced_public_function in unreferenced_public_functions {
            capture!(self, context, unreferenced_public_function);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`public` Functions not used internally could be marked `external`")
    }

    fn description(&self) -> String {
        String::from("Instead of marking a Function as `public`, consider marking it as `external` if it is not used internally.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UselessPublicFunction)
    }
}

#[cfg(test)]
mod useless_public_function_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::UselessPublicFunctionDetector;

    #[test]
    fn test_useless_public_functions() {
        let context =
            load_contract("../tests/contract-playground/out/Counter.sol/Counter.0.8.25.json");

        let mut detector = UselessPublicFunctionDetector::default();
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
            String::from("`public` Functions not used internally could be marked `external`")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from("Instead of marking a Function as `public`, consider marking it as `external` if it is not used internally."));
    }
}
