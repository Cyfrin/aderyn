use std::{collections::BTreeMap, error::Error};

use crate::ast::{ASTNode, NodeID, NodeType};

use crate::{
    capture,
    context::{
        browser::{GetClosestAncestorOfTypeX, GetImmediateParent},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UncheckedLowLevelCallDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UncheckedLowLevelCallDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let call_types = ["call", "staticcall", "delegatecall"];
        for member_access in context.member_accesses() {
            if call_types.iter().any(|&c| c == member_access.member_name)
                && member_access.expression.type_descriptions().is_some_and(|type_desc| {
                    type_desc.type_string.as_ref().is_some_and(|type_string| {
                        type_string == "address" || type_string == "address payable"
                    })
                })
            {
                if let Some(ASTNode::FunctionCall(func_call)) =
                    member_access.closest_ancestor_of_type(context, NodeType::FunctionCall)
                {
                    // In most cases, it's enough to check if the function call's parent is Block
                    // But to cover this case - dst.call.value(msg.value)("");
                    // We need to also check for the possibility where the function call's parent is
                    // another function call and that has a direct parent of type block
                    if let Some(ASTNode::ExpressionStatement(e)) = func_call.parent(context) {
                        if e.parent(context).is_some_and(|node| node.node_type() == NodeType::Block)
                        {
                            capture!(self, context, func_call);
                        }
                    }

                    if let Some(ASTNode::FunctionCall(outside_parent)) = func_call.parent(context) {
                        if let Some(ASTNode::ExpressionStatement(e)) =
                            outside_parent.parent(context)
                        {
                            if e.parent(context)
                                .is_some_and(|node| node.node_type() == NodeType::Block)
                            {
                                capture!(self, context, func_call);
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Unchecked Low level calls")
    }

    fn description(&self) -> String {
        String::from("The return value of the low-level call is not checked, so if the call fails, the Ether will be locked in the contract. If the low level is used to prevent blocking operations, consider logging failed calls. Ensure that the return value of a low-level call is checked or logged.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UncheckedLowLevelCall)
    }
}

#[cfg(test)]
mod unchecked_low_level_calls_tests {

    use crate::detect::{
        detector::IssueDetector, high::unchecked_low_level_call::UncheckedLowLevelCallDetector,
    };

    #[test]

    fn test_unchecked_low_level_calls() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UncheckedCalls.sol",
        );

        let mut detector = UncheckedLowLevelCallDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 9);
    }
}
