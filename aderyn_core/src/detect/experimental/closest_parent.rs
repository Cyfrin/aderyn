#![allow(clippy::collapsible_match)]
use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{
        browser::GetClosestParentOfTypeX,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ClosestParentDemonstrator {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/*

In ParentChainContract.sol, there is only 1 assignment done. The goal is to capture it first, second and third parent
*/

impl IssueDetector for ClosestParentDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments() {
            capture!(self, context, assignment);

            if let Some(ASTNode::Block(block)) =
                assignment.closest_parent_of_type(context, NodeType::Block)
            {
                capture!(self, context, block);
            }

            if let Some(ASTNode::ForStatement(for_statement)) =
                assignment.closest_parent_of_type(context, NodeType::ForStatement)
            {
                capture!(self, context, for_statement);

                if let Some(ASTNode::Block(block)) =
                    for_statement.closest_parent_of_type(context, NodeType::Block)
                {
                    capture!(self, context, block);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
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
mod parent_chain_demo_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        experimental::closest_parent::ClosestParentDemonstrator,
    };

    #[test]
    fn test_closest_parent() {
        let context = load_contract(
            "../tests/contract-playground/out/ParentChainContract.sol/ParentChainContract.json",
        );

        let mut detector = ClosestParentDemonstrator::default();
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
