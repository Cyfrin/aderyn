use std::error::Error;

use crate::{
    ast::{Literal, LiteralKind},
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Default)]
pub struct ConstantsInsteadOfLiteralsDetector {
    found_literals: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for ConstantsInsteadOfLiteralsDetector {
    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        if node.kind == LiteralKind::Number
            || node.kind == LiteralKind::HexString
            || node.kind == LiteralKind::Address
        {
            self.found_literals
                .push(Some(ASTNode::Literal(node.clone())));
        }
        Ok(true)
    }
}

impl Detector for ConstantsInsteadOfLiteralsDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // get all function definitions.
        // for each function definition, find all Literal types
        // if the literal type is either a Number, HexString or Address, then add it to the list of found literals
        for function_definition in loader.get_function_definitions() {
            function_definition.accept(self)?;
        }

        Ok(!self.found_literals.is_empty())
    }

    fn title(&self) -> String {
        String::from("Constants should be defined and used instead of literals")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_literals.clone()
    }
}

#[cfg(test)]
mod constants_instead_of_literals_tests {
    use crate::detector::detector::{detector_test_helpers::load_contract, Detector};

    use super::ConstantsInsteadOfLiteralsDetector;

    #[test]
    fn test_constants_instead_of_literals() {
        let context_loader =
            load_contract("./tests/contract-playground/out/Counter.sol/Counter.json");
        let mut detector = ConstantsInsteadOfLiteralsDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context_loader).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        //
    }
}
