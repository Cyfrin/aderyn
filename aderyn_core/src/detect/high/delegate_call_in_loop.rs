use std::collections::BTreeMap;
use std::error::Error;

use crate::context::browser::MemberAccesses;
use crate::{
    ast::MemberAccess,
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DelegateCallInLoopDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for DelegateCallInLoopDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        let mut member_accesses: Vec<MemberAccess> = vec![];

        member_accesses.extend(loader.for_statements.keys().flat_map(|statement| {
            let accesses: MemberAccesses = statement.into();
            accesses.member_accesses
        }));

        member_accesses.extend(loader.while_statements.keys().flat_map(|statement| {
            let accesses: MemberAccesses = statement.into();
            accesses.member_accesses
        }));

        for member_access in member_accesses {
            if member_access.member_name == "delegatecall" {
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
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::DelegateCallInLoopDetector;

    #[test]
    fn test_delegate_call_in_loop_detector() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
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
