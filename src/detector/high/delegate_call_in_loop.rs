use std::error::Error;

use crate::visitor::ast_visitor::Node;
use crate::{
    ast::MemberAccess,
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::ASTConstVisitor,
};
use eyre::Result;

#[derive(Default)]
pub struct DelegateCallInLoopDetector {
    found_delegate_call_in_loop: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for DelegateCallInLoopDetector {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if node.member_name == "delegatecall" {
            self.found_delegate_call_in_loop
                .push(Some(ASTNode::MemberAccess(node.clone())));
        }
        Ok(true)
    }
}

impl Detector for DelegateCallInLoopDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for for_statement in loader.get_for_statements() {
            for_statement.accept(self)?;
        }
        for while_statement in loader.get_while_statements() {
            while_statement.accept(self)?;
        }

        Ok(self.found_delegate_call_in_loop.len() > 0)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Using `delegatecall` in loop")
    }

    fn description(&self) -> String {
        String::from("When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.")
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_delegate_call_in_loop.clone()
    }
}

#[cfg(test)]
mod delegate_call_in_loop_detector_tests {
    use crate::detector::detector::{detector_test_helpers::load_contract, Detector};

    use super::DelegateCallInLoopDetector;

    #[test]
    fn test_delegate_call_in_loop_detector() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        );
        let mut detector = DelegateCallInLoopDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found a delegate call in a loop
        assert!(found);
        // assert that the detector found the correct number of instances (1)
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detector::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Using `delegatecall` in loop")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "When calling `delegatecall` the same `msg.value` amount will be accredited multiple times."
            )
        );
    }
}
