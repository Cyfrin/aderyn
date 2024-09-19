use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{ASTNode, NodeID, NodeType};

use crate::capture;
use crate::context::browser::GetImmediateParent;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UncheckedSendDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UncheckedSendDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in context.member_accesses() {
            if member_access.member_name == "send"
                && member_access
                    .expression
                    .type_descriptions()
                    .is_some_and(|type_desc| {
                        type_desc.type_string.as_ref().is_some_and(|type_string| {
                            type_string == "address" || type_string == "address payable"
                        })
                    })
            {
                if let Some(ASTNode::FunctionCall(func_call)) = member_access.parent(context) {
                    if let Some(ASTNode::ExpressionStatement(expr_stmnt)) =
                        func_call.parent(context)
                    {
                        if expr_stmnt
                            .parent(context)
                            .is_some_and(|node| node.node_type() == NodeType::Block)
                        {
                            capture!(self, context, func_call);
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
        String::from("Unchecked `bool success` value for send call.")
    }

    fn description(&self) -> String {
        String::from("The transaction `address(payable?).send(address)` may fail because of reasons like out-of-gas, \
        invalid receipient address or revert from the recipient. Therefore, the boolean returned by this function call must be checked \
        to be `true` in order to verify that the transaction was successful")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::UncheckedSend.to_string()
    }
}

#[cfg(test)]
mod unchecked_send_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, high::unchecked_send::UncheckedSendDetector};

    #[test]
    #[serial]
    fn test_unchecked_send() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UncheckedSend.sol",
        );

        let mut detector = UncheckedSendDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("Instances {:#?}", detector.instances());

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Unchecked `bool success` value for send call.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("The transaction `address(payable?).send(address)` may fail because of reasons like out-of-gas, \
            invalid receipient address or revert from the recipient. Therefore, the boolean returned by this function call must be checked \
            to be `true` in order to verify that the transaction was successful")
        );
    }
}
