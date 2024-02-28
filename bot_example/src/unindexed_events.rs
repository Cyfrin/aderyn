use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::workspace_context::WorkspaceContext;
use aderyn_driver::core_ast::NodeID;
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{IssueDetector, IssueSeverity};

#[derive(Default)]
pub struct UnindexedEventsDetector {
    // Keys are source file name and line number
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
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod unindexed_event_tests {

    use super::UnindexedEventsDetector;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::{IssueDetector, IssueSeverity};

    #[test]
    fn test_unindexed_events() {
        let context = load_contract(
            "../../aderyn/tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        );

        let mut detector = UnindexedEventsDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of unindexed events
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(detector.severity(), IssueSeverity::NC);
        // assert that the detector returns the correct title
        assert_eq!(detector.title(), "Event is missing `indexed` fields");
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            "Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed."
        );
    }
}
