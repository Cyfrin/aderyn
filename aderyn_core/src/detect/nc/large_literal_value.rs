use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{LiteralKind, NodeID},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct LargeLiteralValueDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for LargeLiteralValueDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for numeric_literal in context
            .literals()
            .iter()
            .filter(|x| x.kind == LiteralKind::Number)
        {
            if let Some(value) = numeric_literal.value.clone() {
                if value.ends_with("0000") {
                    capture!(self, context, numeric_literal);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from(
            "Large literal values multiples of 10000 can be replaced with scientific notation",
        )
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::LargeNumericLiteral)
    }
}

#[cfg(test)]
mod large_literal_values {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::LargeLiteralValueDetector;

    #[test]
    fn test_large_literal_values_multiples_of_10000() {
        let context =
            load_contract("../tests/contract-playground/out/HugeConstants.sol/HugeConstants.json");

        let mut detector = LargeLiteralValueDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of instances
        assert_eq!(detector.instances().len(), 20);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from(
                "Large literal values multiples of 10000 can be replaced with scientific notation"
            )
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
