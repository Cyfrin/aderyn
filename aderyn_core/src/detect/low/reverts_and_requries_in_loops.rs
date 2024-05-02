use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{NodeID, NodeType};

use crate::capture;
use crate::context::browser::GetClosestAncestorOfTypeX;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct RevertsAndRequiresInLoopsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for RevertsAndRequiresInLoopsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Collect all require statements
        let requires_and_reverts = context
            .identifiers()
            .into_iter()
            .filter(|&id| id.name == "revert" || id.name == "require")
            .collect::<Vec<_>>();

        for item in requires_and_reverts {
            if let Some(for_loop) = item.closest_ancestor_of_type(context, NodeType::ForStatement) {
                capture!(self, context, for_loop);
            }
            if let Some(while_loop) =
                item.closest_ancestor_of_type(context, NodeType::WhileStatement)
            {
                capture!(self, context, while_loop);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Loop contains `require`/`revert` statements")
    }

    fn description(&self) -> String {
        String::from("Avoid `require` / `revert` statements in a loop because a single bad item can cause the whole transaction to fail. It's better to forgive on fail and return failed elements post processing of the loop")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::RevertsAndRequiresInLoops)
    }
}

#[cfg(test)]
mod reevrts_and_requires_in_loops {
    use crate::detect::{
        detector::IssueDetector,
        low::reverts_and_requries_in_loops::RevertsAndRequiresInLoopsDetector,
    };

    #[test]
    fn test_reverts_and_requires_in_loops() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/RevertsAndRequriesInLoops.sol",
        );

        let mut detector = RevertsAndRequiresInLoopsDetector::default();
        let found = detector.detect(&context).unwrap();

        // println!("{:?}", detector.instances());

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Loop contains `require`/`revert` statements")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Avoid `require` / `revert` statements in a loop because a single bad item can cause the whole transaction to fail. It's better to forgive on fail and return failed elements post processing of the loop")
        );
    }
}
