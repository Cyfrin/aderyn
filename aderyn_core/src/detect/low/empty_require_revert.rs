use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct EmptyRequireRevertDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for EmptyRequireRevertDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Collect all require statements without a string literal.
        let requires_and_reverts = context
            .identifiers()
            .into_iter()
            .filter(|&id| id.name == "revert" || id.name == "require");

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
        String::from("Empty `require()` / `revert()` Statement")
    }

    fn description(&self) -> String {
        String::from("Use descriptive reason strings or custom errors for revert paths.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::EmptyRequireRevert)
    }
}

#[cfg(test)]
mod require_with_string_tests {

    use crate::detect::detector::IssueDetector;

    use super::EmptyRequireRevertDetector;

    #[test]
    fn test_require_with_string_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DeprecatedOZFunctions.sol",
        );

        let mut detector = EmptyRequireRevertDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }

    #[test]
    fn test_require_with_custom_error_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnusedError.sol",
        );

        let mut detector = EmptyRequireRevertDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
    }
}
