use std::{collections::HashSet, error::Error};

use crate::{
    ast::Visibility,
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UselessPublicFunctionDetector {
    found_useless_public_function: Vec<Option<ASTNode>>,
}

impl Detector for UselessPublicFunctionDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // Collect the ids of all functions referenced by identifiers.
        let referenced_functions: HashSet<_> = loader
            .get_identifiers()
            .iter()
            .map(|i| i.referenced_declaration)
            .collect();

        let function_definitions = loader.get_function_definitions();

        // Collect all public FunctionDefinitions which are not in the referenced set.
        let unreferenced_public_functions = function_definitions
            .iter()
            .filter(|f| f.visibility == Visibility::Public && !referenced_functions.contains(&f.id))
            .map(|f| Some(ASTNode::FunctionDefinition((*f).clone())));

        self.found_useless_public_function
            .extend(unreferenced_public_functions);

        Ok(!self.found_useless_public_function.is_empty())
    }

    fn title(&self) -> String {
        String::from("Functions not used internally could be marked external")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_useless_public_function.clone()
    }
}

#[cfg(test)]
mod useless_public_function_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::UselessPublicFunctionDetector;

    #[test]
    fn test_useless_public_functions() {
        let context_loader =
            load_contract("./tests/contract-playground/out/Counter.sol/Counter.json");
        let mut detector = UselessPublicFunctionDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context_loader).unwrap();
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
            String::from("Functions not used internally could be marked external")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
