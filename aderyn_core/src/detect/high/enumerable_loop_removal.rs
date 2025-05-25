use std::{collections::BTreeMap, error::Error};

use crate::ast::{NodeID, NodeType};

use crate::{
    capture,
    context::{
        browser::{ExtractMemberAccesses, GetClosestAncestorOfTypeX},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct EnumerableLoopRemovalDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for EnumerableLoopRemovalDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Find MemberAccesses with name `remove` and
        // typeDescriptions.typeString.contains(EnumerableSet) for each one
        // Find the closest ancestor of a loop
        // if it exists, extract all `at` member accesses on the enumerableset
        // If an `at` memberaccess also exists in the loop, add the remove to found_instances

        context
            .member_accesses()
            .into_iter()
            .filter(|member_access| {
                member_access.member_name == "remove"
                    && member_access
                        .type_descriptions
                        .type_string
                        .as_ref()
                        .is_some_and(|type_string| type_string.contains("EnumerableSet"))
            })
            .for_each(|member_access| {
                let parent_loops = [
                    member_access.closest_ancestor_of_type(context, NodeType::ForStatement),
                    member_access.closest_ancestor_of_type(context, NodeType::WhileStatement),
                    member_access.closest_ancestor_of_type(context, NodeType::DoWhileStatement),
                ];
                for parent_loop in parent_loops.into_iter().flatten() {
                    ExtractMemberAccesses::from(parent_loop).extracted.into_iter().for_each(
                        |at_member_access| {
                            if at_member_access.member_name == "at" {
                                capture!(self, context, member_access);
                            }
                        },
                    );
                }
            });

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("EnumerableSet.remove Corrupts Order")
    }

    fn description(&self) -> String {
        String::from("If the order of an EnumerableSet is required, removing items in a loop using `at` and `remove` corrupts this order.
Consider using a different data structure or removing items by collecting them during the loop, then removing after the loop.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::EnumerableLoopRemoval)
    }
}

#[cfg(test)]
mod enuemrable_loop_removal_tests {

    use crate::detect::{detector::IssueDetector, high::EnumerableLoopRemovalDetector};

    #[test]

    fn test_enumerable_loop_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/EnumerableSetIteration.sol",
        );

        let mut detector = EnumerableLoopRemovalDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 5);
    }
}
