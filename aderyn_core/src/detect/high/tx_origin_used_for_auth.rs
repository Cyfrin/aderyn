use std::{collections::BTreeMap, error::Error};

use crate::ast::{ASTNode, Expression, Identifier, NodeID};

use crate::{
    capture,
    context::{
        browser::ExtractMemberAccesses,
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct TxOriginUsedForAuthDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for TxOriginUsedForAuthDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for if_statement in context.if_statements() {
            // Check within the condition block only
            let ast_node: ASTNode = if_statement.condition.clone().into();
            self.check_eligibility_and_capture(context, &[&ast_node], &(if_statement.into()))?;
        }

        for function_call in context.function_calls() {
            if let Expression::Identifier(Identifier { name, .. }) =
                function_call.expression.as_ref()
            {
                if name != "require" {
                    continue;
                }

                // Now, check for arguments of the `require(..., "message")` function call
                let arguments = function_call
                    .arguments
                    .clone()
                    .into_iter()
                    .map(|n| n.into())
                    .collect::<Vec<ASTNode>>();

                let ast_nodes: &[&ASTNode] = &(arguments.iter().collect::<Vec<_>>());
                self.check_eligibility_and_capture(context, ast_nodes, &(function_call.into()))?;
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Use of `tx.origin` for authentication")
    }

    fn description(&self) -> String {
        String::from("Using `tx.origin` may lead to problems when users are interacting via smart contract with your \
            protocol. It is recommended to use `msg.sender` for authentication.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::TxOriginUsedForAuth)
    }
}

impl TxOriginUsedForAuthDetector {
    fn check_eligibility_and_capture(
        &mut self,
        context: &WorkspaceContext,
        check_nodes: &[&ASTNode],
        capture_node: &ASTNode,
    ) -> Result<(), Box<dyn Error>> {
        // Boilerplate
        let callgraphs = CallGraphConsumer::get(context, check_nodes, CallGraphDirection::Inward)?;
        for callgraph in callgraphs {
            let mut tracker = MsgSenderAndTxOriginTracker::default();
            callgraph.accept(context, &mut tracker)?;

            if tracker.satisfied() {
                capture!(self, context, capture_node);
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct MsgSenderAndTxOriginTracker {
    reads_msg_sender: bool,
    reads_tx_origin: bool,
}

impl MsgSenderAndTxOriginTracker {
    /// To avoid FP (msg.sender == tx.origin) we require that tx.origin is present and msg.sender is
    /// absent for it to be considered satisfied
    fn satisfied(&self) -> bool {
        self.reads_tx_origin && !self.reads_msg_sender
    }
}

impl CallGraphVisitor for MsgSenderAndTxOriginTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        let member_accesses = ExtractMemberAccesses::from(node).extracted;

        let has_msg_sender = member_accesses.iter().any(|member_access| {
            member_access.member_name == "sender"
                && if let Expression::Identifier(identifier) = member_access.expression.as_ref() {
                    identifier.name == "msg"
                } else {
                    false
                }
        });
        self.reads_msg_sender = self.reads_msg_sender || has_msg_sender;

        let has_tx_origin = member_accesses.iter().any(|member_access| {
            member_access.member_name == "origin"
                && if let Expression::Identifier(identifier) = member_access.expression.as_ref() {
                    identifier.name == "tx"
                } else {
                    false
                }
        });
        self.reads_tx_origin = self.reads_tx_origin || has_tx_origin;

        Ok(())
    }
}

#[cfg(test)]
mod tx_origin_used_for_auth_detector {

    use crate::detect::{
        detector::IssueDetector, high::tx_origin_used_for_auth::TxOriginUsedForAuthDetector,
    };

    #[test]

    fn test_tx_origin_used_for_auth() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/TxOriginUsedForAuth.sol",
        );

        let mut detector = TxOriginUsedForAuthDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 3);
    }
}
