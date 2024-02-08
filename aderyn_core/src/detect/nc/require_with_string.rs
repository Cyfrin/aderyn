use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{DetectorNamePool, IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct RequireWithStringDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for RequireWithStringDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Collect all require statements without a string literal.
        let requires_and_reverts = context
            .identifiers
            .keys()
            .filter(|id| id.name == "revert" || id.name == "require");

        for id in requires_and_reverts {
            if (id.name == "revert" && id.argument_types.as_ref().unwrap().is_empty())
                || (id.name == "require" && id.argument_types.as_ref().unwrap().len() == 1)
            {
                capture!(self, context, id);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`require()` / `revert()` statements should have descriptive reason strings or custom errors")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", DetectorNamePool::RequireWithString)
    }
}

#[cfg(test)]
mod require_with_string_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::RequireWithStringDetector;

    #[test]
    fn test_require_with_string() {
        let context = load_contract(
            "../tests/contract-playground/out/DeprecatedOZFunctions.sol/DeprecatedOZFunctions.json",
        );

        let mut detector = RequireWithStringDetector::default();
        // assert that the detector finds something
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("`require()` / `revert()` statements should have descriptive reason strings or custom errors")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
