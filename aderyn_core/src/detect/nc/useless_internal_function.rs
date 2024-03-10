use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, Visibility},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity, ReusableDetector},
        reusable::IdentifiersThatReferenceAFunctionDetector,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct UselessInternalFunctionDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UselessInternalFunctionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let internal_functions = context
            .function_definitions()
            .into_iter()
            .filter(|&function| matches!(function.visibility, Visibility::Internal));

        for internal_function in internal_functions {
            if IdentifiersThatReferenceAFunctionDetector::default()
                .detect(context, &[internal_function.into()], &[])
                .map_or(false, |refs| refs.len() == 1)
            {
                capture!(self, context, internal_function);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Internal functions called only once can be inlined")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
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
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::UselessInternalFunctionDetector;

    #[test]
    fn test_useless_internal_functions() {
        let context = load_contract(
            "../tests/contract-playground/out/InternalFunctions.sol/InternalFunctionExample.json",
        );

        let mut detector = UselessInternalFunctionDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 1);

        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Internal functions called only once can be inlined")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
