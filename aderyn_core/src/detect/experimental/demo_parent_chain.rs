use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::{get_all_parents_in_no_specific_order, get_parent_chain_of},
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ParentChainDemonstrator {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
    pub found_nodes: Vec<ASTNode>,
}

impl IssueDetector for ParentChainDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments.keys() {
            capture!(self, context, assignment.clone());

            let parent_chain = get_all_parents_in_no_specific_order(&assignment.into(), context);
            if let Some(chain) = parent_chain {
                for element in chain {
                    if let ASTNode::FunctionDefinition(def) = element.clone() {
                        capture!(self, context, def);
                    }
                    self.found_nodes.push(element);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Centralization Risk for trusted owners")
    }

    fn description(&self) -> String {
        String::from("Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.")
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod parent_chain_demo_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        experimental::demo_parent_chain::ParentChainDemonstrator,
    };

    #[test]
    fn test_parent_chain_demo() {
        let context = load_contract(
            "../tests/contract-playground/out/ParentChainContract.sol/ParentChainContract.json",
        );

        let mut detector = ParentChainDemonstrator::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found a centralization risk
        assert!(found);

        println!("{:?}", detector.instances());

        println!("FOUND NODES");
        println!("{:?}", detector.found_nodes);

        // assert that the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Medium
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Centralization Risk for trusted owners")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds."
            )
        );
    }
}
