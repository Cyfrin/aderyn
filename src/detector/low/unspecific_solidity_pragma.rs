use std::error::Error;

use crate::{
    ast::PragmaDirective,
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Default)]
pub struct UnspecificSolidityPragmaDetector {
    pub found_unspecific_solidity_pragma: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for UnspecificSolidityPragmaDetector {
    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        for literal in &node.literals {
            if literal.contains("^") || literal.contains(">") {
                self.found_unspecific_solidity_pragma
                    .push(Some(ASTNode::PragmaDirective(node.clone())));
                break;
            }
        }
        Ok(true)
    }
}

impl Detector for UnspecificSolidityPragmaDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for pragma_directive in loader.get_pragma_directives() {
            pragma_directive.accept(self)?;
        }
        Ok(self.found_unspecific_solidity_pragma.len() > 0)
    }

    fn title(&self) -> String {
        String::from("Solidity pragma should be specific, not wide")
    }

    fn description(&self) -> String {
        String::from("Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_unspecific_solidity_pragma.clone()
    }
}

#[cfg(test)]
mod unspecific_solidity_pragma_tests {
    use crate::detector::{
        detector::{detector_test_helpers::load_contract, Detector},
        low::unspecific_solidity_pragma::UnspecificSolidityPragmaDetector,
    };

    #[test]
    fn test_deprecated_oz_functions_detector() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/IContractInheritance.sol/IContractInheritance.json",
        );
        let mut detector = UnspecificSolidityPragmaDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct abi encode packed
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 1);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detector::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Solidity pragma should be specific, not wide")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`"
            )
        );
    }
}
