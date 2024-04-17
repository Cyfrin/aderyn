use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{browser::PeekOver, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct PeekOverDemonstrator {
    // Keys are source file name and line number
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

#[cfg(test)]
mod peek_over_demonstrator_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        experimental::peek_over::PeekOverDemonstrator,
    };

    #[test]
    fn test_peek_over() {
        let context = load_contract(
            "../tests/contract-playground/out/StorageConditionals.sol/StorageConditionals.json",
        );

        let mut detector = PeekOverDemonstrator::default();
        let _ = detector.detect(&context).unwrap();

        let instances = detector.instances();
        println!("{:?}", instances);

        assert!(instances.len() == 2);
    }
}
