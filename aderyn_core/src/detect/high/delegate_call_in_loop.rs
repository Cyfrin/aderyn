use std::collections::BTreeMap;
use std::error::Error;

use crate::{
    ast::{MemberAccess, NodeID},
    capture,
    context::{browser::ExtractMemberAccesses, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DelegateCallInLoopDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DelegateCallInLoopDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut member_accesses: Vec<MemberAccess> = vec![];

        // Get all delegatecall member accesses inside for statements
        member_accesses.extend(context.for_statements().iter().flat_map(|&statement| {
            ExtractMemberAccesses::from(statement)
                .extracted
                .into_iter()
                .filter(|ma| ma.member_name == "delegatecall")
        }));

        // Get all delegatecall member accsesses inside while statements
        member_accesses.extend(context.while_statements().iter().flat_map(|&statement| {
            ExtractMemberAccesses::from(statement)
                .extracted
                .into_iter()
                .filter(|ma| ma.member_name == "delegatecall")
        }));

        // For each member access found, add them to found_instances
        for member_access in member_accesses {
            capture!(self, context, member_access);
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

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::DelegateCallInLoop)
    }
}

#[cfg(test)]
mod delegate_call_in_loop_detector_tests {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::DelegateCallInLoopDetector;

    #[test]
    #[serial]
    fn test_delegate_call_in_loop_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/inheritance/ExtendedInheritance.sol",
        );

        let mut detector = DelegateCallInLoopDetector::default();
        let found = detector.detect(&context).unwrap();
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
