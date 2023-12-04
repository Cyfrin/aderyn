use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::*;
use crate::{
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DelegateCallInLoopDetector {
    found_member_access: Vec<Option<ASTNode>>,

    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl AstBaseVisitor for DelegateCallInLoopDetector {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if node.member_name == "delegatecall" {
            self.found_member_access
                .push(Some(ASTNode::MemberAccess(node.clone())));
        }
        Ok(true)
    }
}

impl Detector for DelegateCallInLoopDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for for_statement in loader.for_statements.iter() {
            for_statement.accept(self)?;
        }
        for while_statement in loader.while_statements.iter() {
            while_statement.accept(self)?;
        }
        for member_access in self.found_member_access.clone().into_iter().flatten() {
            if let ASTNode::MemberAccess(member_access) = member_access {
                self.found_instances.insert(
                    loader.get_node_sort_key(&ASTNode::MemberAccess(member_access.clone())),
                    member_access.src.clone(),
                );
            }
        }

        Ok(!self.found_instances.is_empty())
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

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod delegate_call_in_loop_detector_tests {
    use std::path::PathBuf;

    use crate::detect::detector::{detector_test_helpers::load_contract_from_source, Detector};

    use super::DelegateCallInLoopDetector;

    #[test]
    fn test_delegate_call_in_loop_detector() {
        let context_loader = load_contract_from_source(&PathBuf::from(
            "./tests/contract-playground/src/inheritance/ExtendedInheritance.sol",
        ));
        let mut detector = DelegateCallInLoopDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found a delegate call in a loop
        assert!(found);
        // assert that the detector found the correct number of instances (1)
        assert_eq!(detector.instances().len(), 1);
        println!("{:#?}", detector.instances());
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
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
