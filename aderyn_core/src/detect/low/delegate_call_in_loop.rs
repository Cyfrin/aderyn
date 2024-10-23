use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{ASTNode, NodeID},
    capture,
    context::{
        browser::ExtractMemberAccesses,
        graph::{CallGraph, CallGraphDirection, CallGraphVisitor},
        workspace_context::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::get_explore_centers_of_loops,
    },
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
        // PLAN
        // Explore inward from loops and track all the `delegatecall` that you come across

        let loop_explore_centers = get_explore_centers_of_loops(context);

        for explore_center in loop_explore_centers {
            // Setup
            // Later when https://github.com/Cyfrin/aderyn/pull/650 is merged, we can make it so that it
            // tracks the whole path to the actual delegate call site to display in the report.
            let mut delegate_call_tracker = DelegateCallTracker::default();

            // All the ASTNodes that are potentially run in a loop
            let callgraph = CallGraph::new(context, &[explore_center], CallGraphDirection::Inward)?;

            // Kick-off
            callgraph.accept(context, &mut delegate_call_tracker)?;

            if delegate_call_tracker.has_delegate_call {
                capture!(self, context, explore_center);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Using `delegatecall` in loop may consume excessive gas")
    }

    fn description(&self) -> String {
        String::from("Using `delegatecall` in loop may consume excessive gas")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::DelegateCallInLoop)
    }
}

#[derive(Default)]
struct DelegateCallTracker {
    has_delegate_call: bool,
}

impl CallGraphVisitor for DelegateCallTracker {
    fn visit_any(&mut self, node: &ASTNode) -> eyre::Result<()> {
        if self.has_delegate_call {
            return Ok(());
        }

        let dcalls = ExtractMemberAccesses::from(node)
            .extracted
            .into_iter()
            .filter(|ma| ma.member_name == "delegatecall")
            .count();

        self.has_delegate_call = dcalls > 0;

        Ok(())
    }
}

#[cfg(test)]
mod delegate_call_in_loop_detector_tests {
    use serial_test::serial;

    use super::DelegateCallInLoopDetector;
    use crate::detect::detector::IssueDetector;

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
        assert_eq!(detector.severity(), crate::detect::detector::IssueSeverity::Low);
    }
}
