use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::GetImmediateParent,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ParentDemonstrator {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

/*

In ParentChainContract.sol, there is only 1 assignment done. The goal is to capture it's parent

*/

impl IssueDetector for ParentDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments.keys() {
            // retrieve immediate parent
            if let Some(ASTNode::Block(block)) = assignment.parent(context) {
                capture!(self, context, block);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Parent Demonstration")
    }

    fn description(&self) -> String {
        String::from("Parent Demonstration")
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod parent_demo_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        experimental::demo_parent::ParentDemonstrator,
    };

    #[test]
    fn test_parent_demo() {
        let context = load_contract(
            "../tests/contract-playground/out/ParentChainContract.sol/ParentChainContract.json",
        );

        let mut detector = ParentDemonstrator::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        println!("{:?}", detector.instances());
        assert!(detector.instances().len() == 1);
        // Points to line 17 which is where the block starts
    }
}
