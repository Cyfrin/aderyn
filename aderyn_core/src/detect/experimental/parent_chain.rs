#![allow(clippy::collapsible_match)]
use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::{ASTNode, WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ParentChainDemonstrator {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

/*

In ParentChainContract.sol, there is only 1 assignment done. The goal is to capture it first, second and third parent
*/

impl IssueDetector for ParentChainDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments() {
            println!("0 {}", assignment);
            capture!(self, context, assignment);
            if let Some(first_parent) = context.get_parent(assignment.id) {
                if let ASTNode::Block(block) = first_parent {
                    println!("1 {}", block);
                    capture!(self, context, block);
                    if let Some(second_parent) = context.get_parent(block.id) {
                        if let ASTNode::ForStatement(for_statement) = second_parent {
                            println!("2 {}", for_statement);
                            capture!(self, context, for_statement);
                            if let Some(third_parent) = context.get_parent(for_statement.id) {
                                if let ASTNode::Block(block) = third_parent {
                                    println!("3 {}", block);
                                    capture!(self, context, block);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Parent Chain Demonstration")
    }

    fn description(&self) -> String {
        String::from("Parent Chain Demonstration")
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
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
        experimental::parent_chain::ParentChainDemonstrator,
    };

    #[test]
    fn test_parent_chain_demo() {
        let context = load_contract(
            "../tests/contract-playground/out/ParentChainContract.sol/ParentChainContract.json",
        );

        let mut detector = ParentChainDemonstrator::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        // Instances
        /*
            Although we capture! 4 times, we will have only 3 instances
            because line 17 covers both the first and the second parent ! a.k.a block and the for statement
                16, block
                17, for statement, block
                18, assignment
        */
        println!("{:?}", detector.instances());
        println!(
            "Total number of instances: {:?}",
            detector.instances().len()
        );
        assert!(detector.instances().len() == 3);
    }
}
