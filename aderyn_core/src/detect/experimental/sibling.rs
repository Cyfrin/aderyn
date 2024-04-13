use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    context::{
        browser::{
            GetImmediateChildren, GetNextSibling, GetPreviousSibling, SortNodeReferencesToSequence,
        },
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SiblingDemonstrator {
    // Keys are source file name and line number
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

#[cfg(test)]
mod sibling_demo_tests {
    use crate::detect::{
        detector::{
            detector_test_helpers::{load_contract, load_contract_directly},
            IssueDetector,
        },
        experimental::sibling::SiblingDemonstrator,
    };

    #[test]
    fn test_siblings() {
        let context = load_contract(
            "../tests/contract-playground/out/StorageConditionals.sol/StorageConditionals.json",
        );

        let mut detector = SiblingDemonstrator::default();
        let _ = detector.detect(&context).unwrap();
    }

    #[test]
    fn test_siblings_by_loading_contract_directly() {
        let context =
            load_contract_directly("../tests/contract-playground/src/StorageConditionals.sol");

        let mut detector = SiblingDemonstrator::default();
        let _ = detector.detect(&context).unwrap();
    }
}
