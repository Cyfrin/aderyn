use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::{
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SelfdestructDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for SelfdestructDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers() {
            if identifier.name == "selfdestruct" {
                capture!(self, context, identifier);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("`selfdestruct` is Deprecated")
    }

    fn description(&self) -> String {
        String::from("Remove the `selfdestruct` instruction from the code.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::Selfdestruct.to_string()
    }
}

#[cfg(test)]
mod selfdestruct_identifier_tests {

    use crate::detect::{detector::IssueDetector, high::SelfdestructDetector};

    #[test]

    fn test_selfdestruct_identifier_tests() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UsingSelfdestruct.sol",
        );

        let mut detector = SelfdestructDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(detector.severity(), crate::detect::detector::IssueSeverity::High);
        // assert the title is correct
        assert_eq!(detector.title(), String::from("`selfdestruct` is Deprecated"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Remove the `selfdestruct` instruction from the code.")
        );
    }
}
