use std::error::Error;

use crate::{
    ast::Identifier,
    context::{ast_node::ASTNode, loader::ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct RequireWithStringDetector {
    found_require_without_string: Vec<Option<ASTNode>>,
}

impl Detector for RequireWithStringDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // Collect all require statements without a string literal.
        let requires_and_reverts: Vec<&Identifier> = loader
            .get_identifiers()
            .iter()
            .filter(|id| id.name == "revert" || id.name == "require")
            .cloned()
            .collect();

        for id in requires_and_reverts {
            if (id.name == "revert" && id.argument_types.as_ref().unwrap().is_empty())
                || (id.name == "require" && id.argument_types.as_ref().unwrap().len() == 1)
            {
                self.found_require_without_string
                    .push(Some(ASTNode::Identifier(id.clone())));
            }
        }

        Ok(!self.found_require_without_string.is_empty())
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

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_require_without_string.clone()
    }
}

#[cfg(test)]
mod require_with_string_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::RequireWithStringDetector;

    #[test]
    fn test_require_with_string() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/DeprecatedOZFunctions.sol/DeprecatedOZFunctions.json",
        );
        let mut detector = RequireWithStringDetector::default();
        // assert that the detector finds something
        let found = detector.detect(&context_loader).unwrap();
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
