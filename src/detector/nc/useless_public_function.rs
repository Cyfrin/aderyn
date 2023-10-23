use std::error::Error;

use crate::{
    ast::{Identifier, Visibility},
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Default)]
pub struct UselessPublicFunctionDetector {
    found_useless_public_function: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for UselessPublicFunctionDetector {
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        // If there are any items in the found_useless_public_function vector that have an ID
        // equal to node.referenced_declaration, remove that item from the vector
        self.found_useless_public_function.retain(|f| {
            match f {
                Some(ASTNode::FunctionDefinition(func_def)) => {
                    func_def.id != node.referenced_declaration
                }
                _ => true, // retain the item if it's not a FunctionDefinition
            }
        });
        Ok(true)
    }
}

impl Detector for UselessPublicFunctionDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // Get all FunctionDefinitions that are `public`, and store them in a vector
        let binding = loader.get_function_definitions();
        let new_nodes = binding
            .iter()
            .filter(|f| f.visibility == Visibility::Public)
            .map(|f| Some(ASTNode::FunctionDefinition((**f).clone())));

        self.found_useless_public_function.extend(new_nodes);

        // Visit all Identifiers and check if the function_call.expression.referenced_declaration is
        loader.get_identifiers().iter().for_each(|i| {
            i.accept(self).unwrap();
        });

        Ok(self.found_useless_public_function.len() > 0)
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
    use crate::detector::detector::{detector_test_helpers::load_contract, Detector};

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
            crate::detector::detector::IssueSeverity::NC
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
