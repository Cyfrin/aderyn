use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct TimeUnitsNotUsedDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for TimeUnitsNotUsedDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let disallow = [
            "second", "seconds", "minute", "minutes", "hour", "hours", "day", "days", "week",
            "weeks",
        ];

        for variable in context.variable_declarations() {
            if disallow.iter().any(|&x| x == variable.name.to_lowercase()) {
                capture!(self, context, variable);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Native time units would be ideal instead of custom definitions")
    }

    fn description(&self) -> String {
        String::from("Suffixes like `seconds`, `minutes`, `hours`, `days` and `weeks` after literal numbers can be used to specify units of time where seconds are the base unit and units are considered naively.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::TimeUnitsNotUsed)
    }
}

#[cfg(test)]
mod time_units {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::TimeUnitsNotUsedDetector;

    #[test]
    fn test_time_units() {
        let context =
            load_contract("../tests/contract-playground/out/TimeUnits.sol/TimeUnits.json");

        let mut detector = TimeUnitsNotUsedDetector::default();
        // assert that the detector finds something
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 3);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
    }
}
