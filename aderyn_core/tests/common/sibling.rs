use std::{collections::BTreeMap, error::Error};

use aderyn_core::{
    ast::NodeID,
    capture,
    context::{
        browser::{
            GetImmediateChildren, GetNextSibling, GetPreviousSibling, SortNodeReferencesToSequence,
        },
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SiblingDemonstrator {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for SiblingDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.contract_definitions() {
            if let Some(children) = contract.children(context) {
                if let Some(sorted) = children.sort_by_src_position(context) {
                    assert!(sorted.len() >= 2);
                    assert!(sorted[1].previous_sibling(context).unwrap() == sorted[0]);
                    assert!(sorted[0].next_sibling(context).unwrap() == sorted[1]);
                    capture!(self, context, sorted[1]);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Sibling Demonstration")
    }

    fn description(&self) -> String {
        String::from("Sibling Demonstration")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}
