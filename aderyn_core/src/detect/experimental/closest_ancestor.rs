#![allow(clippy::collapsible_match)]
use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{
        browser::GetClosestAncestorOfTypeX,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ClosestAncestorDemonstrator {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/*

In ParentChainContract.sol, there is only 1 assignment done. The goal is to capture it first, second and third parent
*/

impl IssueDetector for ClosestAncestorDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments() {
            capture!(self, context, assignment);

            if let Some(ASTNode::Block(block)) =
                assignment.closest_ancestor_of_type(context, NodeType::Block)
            {
                capture!(self, context, block);
            }

            if let Some(for_statement) =
                assignment.closest_ancestor_of_type(context, NodeType::ForStatement)
            {
                capture!(self, context, for_statement);

                if let Some(ASTNode::Block(block)) =
                    for_statement.closest_ancestor_of_type(context, NodeType::Block)
                {
                    capture!(self, context, block);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Closest Parent Demonstrator")
    }

    fn description(&self) -> String {
        String::from("Closest Parent Demonstrator")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod closest_ancestor_demo_tests {
    use crate::detect::{
        detector::IssueDetector, experimental::closest_ancestor::ClosestAncestorDemonstrator,
    };

    use serial_test::serial;

    #[test]
    #[serial]
    fn test_closest_ancestor() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/parent_chain/ParentChainContract.sol",
        );

        let mut detector = ClosestAncestorDemonstrator::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        println!("{:?}", detector.instances());
        println!(
            "Total number of instances: {:?}",
            detector.instances().len()
        );
        assert!(detector.instances().len() == 4);
    }
}
