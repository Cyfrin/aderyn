use std::error::Error;

use crate::{
    context::{ast_node::ASTNode, loader::ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DeprecatedOZFunctionsDetector {
    found_deprecated_oz_functions: Vec<Option<ASTNode>>,
}

impl Detector for DeprecatedOZFunctionsDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for identifier in loader.get_identifiers() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call identifier.accept(self)
            let source_unit = loader
                .get_source_unit_from_child_node(&ASTNode::Identifier(identifier.clone()))
                .unwrap();

            let import_directives = source_unit.import_directives();
            if import_directives.iter().any(|directive| {
                directive
                    .absolute_path
                    .as_ref()
                    .map_or(false, |path| path.contains("openzeppelin"))
            }) && identifier.name == "_setupRole"
            {
                self.found_deprecated_oz_functions
                    .push(Some(ASTNode::Identifier(identifier.clone())));
            }
        }
        for member_access in loader.get_member_accesses() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call member_access.accept(self)
            let source_unit = loader
                .get_source_unit_from_child_node(&ASTNode::MemberAccess(member_access.clone()))
                .unwrap();
            let import_directives = source_unit.import_directives();
            if import_directives.iter().any(|directive| {
                directive
                    .absolute_path
                    .as_ref()
                    .map_or(false, |path| path.contains("openzeppelin"))
            }) && member_access.member_name == "safeApprove"
            {
                self.found_deprecated_oz_functions
                    .push(Some(ASTNode::MemberAccess(member_access.clone())));
            }
        }
        Ok(!self.found_deprecated_oz_functions.is_empty())
    }

    fn title(&self) -> String {
        String::from("Deprecated OpenZeppelin functions should not be used")
    }

    fn description(&self) -> String {
        String::from("Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_deprecated_oz_functions.clone()
    }
}

#[cfg(test)]
mod deprecated_oz_functions_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::DeprecatedOZFunctionsDetector;

    #[test]
    fn test_deprecated_oz_functions_detector() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/DeprecatedOZFunctions.sol/DeprecatedOZFunctions.json",
        );
        let mut detector = DeprecatedOZFunctionsDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Deprecated OpenZeppelin functions should not be used")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/"
            )
        );
    }
}
