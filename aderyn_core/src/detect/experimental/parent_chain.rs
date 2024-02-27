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

In ParentChainContract.sol, there is only 1 assignment done. The goal is to capture it first.
Then walk up it's parent tree and capture a) contract definition b) if statement c) for statement
We omit the function definition for the sake of example

*/

impl IssueDetector for ParentChainDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // println!("{:?}", context.parent_link);

        // for (k, v) in context.parent_link.iter() {
        //     let child = context.nodes.get(k);
        //     let parent = context.nodes.get(v);
        //     println!("{:?}\n{:?}\n\n", child, parent);
        // }

        for function_definition in context.function_definitions() {
            if let Some(first_parent) = context.get_parent(function_definition.id) {
                // println!("{:?}", first_parent);
                if let ASTNode::ContractDefinition(cd) = first_parent {
                    capture!(self, context, cd);
                }
            }
            // Retrieve Parent Chain (from closest to farthest)
            // let parents = GetParentChain::of(assignment, context);
            // {
            //     println!("Parent Chain (from closest to farthest)\n---------");
            //     for p in &parents {
            //         println!("{}", p)
            //     }
            //     println!("------------");
            /*
            ---------
            Assignment
            Block
            ForStatement
            Block
            IfStatement
            Block
            FunctionDefinition
            ContractDefinition
            ------------
             */
            // }
            // for p in &parents {
            //     if let ASTNode::ContractDefinition(f) = p {
            //         capture!(self, context, f);
            //     }
            //     if let ASTNode::Block(b) = p {
            //         for statement in &b.statements {
            //             match statement {
            //                 ast::statements::Statement::IfStatement(i) => {
            //                     println!("If statement captured !\n{}", i);
            //                     capture!(self, context, i);
            //                 }
            //                 ast::statements::Statement::ForStatement(f) => {
            //                     println!("For statement captured !\n{}", f);
            //                     capture!(self, context, f);
            //                 }
            //                 _ => (),
            //             };
            //         }
            //     }
            // }
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
            line 7, contract definition
                16, if statement
                17, for statement
                18, assignment
        */
        println!("{:?}", detector.instances());
        println!(
            "Total number of instances: {:?}",
            detector.instances().len()
        );
    }
}
