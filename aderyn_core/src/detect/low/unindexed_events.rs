use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnindexedEventsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnindexedEventsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // for each event definition, check if it has any indexed parameters
        // if it does not, then add it to the list of found unindexed events
        for event_definition in context.event_definitions() {
            let mut indexed_count = 0;
            let mut non_indexed = false;

            for param in &event_definition.parameters.parameters {
                if let Some(true) = param.indexed {
                    indexed_count += 1;
                } else {
                    non_indexed = true;
                }
            }

            if non_indexed && indexed_count < 3 {
                capture!(self, context, event_definition);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Event is missing `indexed` fields")
    }

    fn description(&self) -> String {
        String::from(
            "Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }
    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnindexedEvents)
    }
}

#[cfg(test)]
mod unindexed_event_tests {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::UnindexedEventsDetector;

    #[test]
    #[serial]
    fn test_unindexed_events_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/inheritance/ExtendedInheritance.sol",
        );

        let mut detector = UnindexedEventsDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of unindexed events
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(detector.title(), "Event is missing `indexed` fields");
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            "Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed."
        );
    }
}
