use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{ASTNode, NodeID},
    capture,
    context::{
        browser::ExtractMemberAccesses,
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::get_explore_centers_of_loops,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct DelegatecallInLoopDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DelegatecallInLoopDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // Explore inward from loops and track all the `delegatecall` that you come across

        let loop_explore_centers = get_explore_centers_of_loops(context);

        for explore_center in loop_explore_centers {
            // TODO: capture hints!

            // All the ASTNodes that are potentially run in a loop
            let callgraphs =
                CallGraphConsumer::get(context, &[explore_center], CallGraphDirection::Inward)?;

            for callgraph in callgraphs {
                let mut delegate_call_tracker = DelegateCallTracker::default();
                callgraph.accept(context, &mut delegate_call_tracker)?;

                if delegate_call_tracker.has_delegate_call {
                    capture!(self, context, explore_center);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("`delegatecall` in loop")
    }

    fn description(&self) -> String {
        String::from("Using `delegatecall` in loop may consume excessive gas, or worse, lead to more severe issues.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::DelegatecallInLoop)
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

    use super::DelegatecallInLoopDetector;
    use crate::detect::detector::IssueDetector;

    #[test]

    fn test_delegate_call_in_loop_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/inheritance/ExtendedInheritance.sol",
        );

        let mut detector = DelegatecallInLoopDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
