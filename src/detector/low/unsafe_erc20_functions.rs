use std::error::Error;

use crate::{
    ast::MemberAccess,
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC20FunctionsDetector {
    found_unsafe_erc20_functions: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for UnsafeERC20FunctionsDetector {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if node.member_name == "transferFrom"
            || node.member_name == "approve"
            || node.member_name == "transfer"
        {
            self.found_unsafe_erc20_functions
                .push(Some(ASTNode::MemberAccess(node.clone())));
        }
        Ok(true)
    }
}

impl Detector for UnsafeERC20FunctionsDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for member_access in loader.get_member_accesses() {
            member_access.accept(self)?;
        }
        Ok(self.found_unsafe_erc20_functions.len() > 0)
    }

    fn title(&self) -> String {
        String::from("Unsafe ERC20 Operations should not be used")
    }

    fn description(&self) -> String {
        String::from("ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_unsafe_erc20_functions.clone()
    }
}

#[cfg(test)]
mod unsafe_erc20_functions_tests {
    use crate::detector::detector::{detector_test_helpers::load_contract, Detector};

    use super::UnsafeERC20FunctionsDetector;

    #[test]
    fn test_unsafe_erc20_functions() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/DeprecatedOZFunctions.sol/DeprecatedOZFunctions.json",
        );
        let mut detector = UnsafeERC20FunctionsDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct abi encode packed
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 2);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detector::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Unsafe ERC20 Operations should not be used")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library."
            )
        );
    }
}
