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
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
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
                // Strip any underscore separators
                let value_no_underscores = value.replace('_', "");
                let is_huge = value_no_underscores.ends_with("0000");
                let is_hex = value_no_underscores.starts_with("0x");
                let is_exp = value_no_underscores.contains('e');
                if is_huge && !is_hex && !is_exp {
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
        String::from("Use `e` notation, for example: `1e18`, instead of its full numeric value.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
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
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::LargeLiteralValueDetector;

    #[test]
    #[serial]
    fn test_large_literal_values_multiples_of_10000_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/HugeConstants.sol",
        );

        let mut detector = LargeLiteralValueDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of instances
        assert_eq!(detector.instances().len(), 22);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from(
                "Large literal values multiples of 10000 can be replaced with scientific notation"
            )
        );
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            String::from(
                "Use `e` notation, for example: `1e18`, instead of its full numeric value."
            )
        );
    }
}
