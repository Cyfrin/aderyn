#![allow(clippy::collapsible_match)]
use std::{collections::BTreeMap, error::Error};

use aderyn_core::{
    ast::{NodeID, NodeType},
    capture,
    context::{browser::GetImmediateChildren, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ImmediateChildrenDemonstrator {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
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
        IssueSeverity::High
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
