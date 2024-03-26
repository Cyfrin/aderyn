#![allow(clippy::collapsible_match)]
use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{browser::GetImmediateChildren, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ImmediateChildrenDemonstrator {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ImmediateChildrenDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Step 1: Find the ParentChain contract

        let parent_chain_contract = context
            .contract_definitions()
            .into_iter()
            .filter(|x| x.name.contains("ParentChain"))
            .take(1)
            .next()
            .unwrap();

        // Step 2: Find the `increment` function

        let inc = parent_chain_contract
            .function_definitions()
            .into_iter()
            .filter(|x| x.name.contains("increment"))
            .take(1)
            .next()
            .unwrap();

        if let Some(children) = inc.body.as_ref().unwrap().children(context) {
            for child in children {
                assert!(
                    child.node_type() == NodeType::IfStatement,
                    "Only if statement should be caught in function body's immediate children!"
                );
                capture!(self, context, child);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("ImmediateChildrenDemonstrator")
    }

    fn description(&self) -> String {
        String::from("ImmediateChildrenDemonstrator")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod child_chain_demo_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        experimental::immediate_children::ImmediateChildrenDemonstrator,
    };

    #[test]
    fn test_immediate_child_demo() {
        let context = load_contract(
            "../tests/contract-playground/out/ParentChainContract.sol/ParentChainContract.json",
        );

        let mut detector = ImmediateChildrenDemonstrator::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        println!("{:?}", detector.instances());
        println!(
            "Total number of instances: {:?}",
            detector.instances().len()
        );
        assert!(detector.instances().len() == 1);
    }
}
