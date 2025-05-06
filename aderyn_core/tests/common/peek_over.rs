use std::{collections::BTreeMap, error::Error};

use aderyn_core::{
    ast::NodeID,
    capture,
    context::{browser::PeekOver, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct PeekOverDemonstrator {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for PeekOverDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for event in context.event_definitions() {
            if let Some(text) = event.peek_over(context) {
                if text.contains("// SAME CONDITIONALS")
                    || text.contains("// DIFFERENT CONDITIONALS")
                {
                    capture!(self, context, event);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Peek Over Demonstration")
    }

    fn description(&self) -> String {
        String::from("Peek Over Demonstration")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}
