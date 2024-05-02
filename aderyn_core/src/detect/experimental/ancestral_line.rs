#![allow(clippy::collapsible_match)]
use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::{GetAncestralLine, SortNodeReferencesToSequence},
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct AncestralLineDemonstrator {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/*

In ParentChainContract.sol, there is only 1 assignment done. The goal is to capture it first, second and third parent
*/

impl IssueDetector for AncestralLineDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments() {
            capture!(self, context, assignment);

            if let Some(parent_chain) = assignment.ancestral_line(context) {
                if let ASTNode::Block(_) = parent_chain[1] {
                    capture!(self, context, parent_chain[1]);
                }
                if let ASTNode::ForStatement(_) = parent_chain[2] {
                    capture!(self, context, parent_chain[2]);
                }
                if let ASTNode::Block(block) = parent_chain[3] {
                    capture!(self, context, block);
                }
            }

            if let Some(mut parent_chain) = assignment.ancestral_line(context) {
                let sorted_chain = parent_chain.sort_by_src_position(context).unwrap();
                parent_chain.reverse();
                assert_eq!(sorted_chain, parent_chain);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Parent Chain Demonstration")
    }

    fn description(&self) -> String {
        String::from("Parent Chain Demonstration")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod ancestral_line_demo_tests {
    use crate::detect::{
        detector::IssueDetector, experimental::ancestral_line::AncestralLineDemonstrator,
    };

    use serial_test::serial;

    #[test]
    #[serial]
    fn test_ancestral_line_demo() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/parent_chain/ParentChainContract.sol",
        );

        let mut detector = AncestralLineDemonstrator::default();
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
