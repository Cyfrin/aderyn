use std::{collections::BTreeMap, convert::identity, error::Error};

use crate::ast::{ASTNode, Expression, NodeID};

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
pub struct MsgValueUsedInLoopDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for MsgValueUsedInLoopDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Investigate for loops to check for usage of `msg.value`
        for for_statement in context.for_statements() {
            if uses_msg_value(context, &(for_statement.into())).is_some_and(identity) {
                capture!(self, context, for_statement);
            }
        }

        // Investigate while loops to check for usage of `msg.value`
        for while_statement in context.while_statements() {
            if uses_msg_value(context, &(while_statement.into())).is_some_and(identity) {
                capture!(self, context, while_statement);
            }
        }

        // Investigate the do while loops to check for usage of `msg.value`
        for do_while_statement in context.do_while_statements() {
            if uses_msg_value(context, &(do_while_statement.into())).is_some_and(identity) {
                capture!(self, context, do_while_statement);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Loop contains `msg.value`")
    }

    fn description(&self) -> String {
        String::from("Provide an explicit array of amounts alongside the receivers array, and check that the sum of all amounts matches `msg.value`.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::MsgValueInLoop.to_string()
    }
}

fn uses_msg_value(context: &WorkspaceContext, ast_node: &ASTNode) -> Option<bool> {
    let callgraphs =
        CallGraphConsumer::get(context, &[ast_node], CallGraphDirection::Inward).ok()?;

    for callgraph in callgraphs {
        let mut tracker = MsgValueTracker::default();
        callgraph.accept(context, &mut tracker).ok()?;
        if tracker.has_msg_value {
            return Some(true);
        }
    }
    Some(false)
}

#[derive(Default)]
struct MsgValueTracker {
    has_msg_value: bool,
}

impl CallGraphVisitor for MsgValueTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        if !self.has_msg_value
            && ExtractMemberAccesses::from(node).extracted.iter().any(|member_access| {
                member_access.member_name == "value"
                    && if let Expression::Identifier(identifier) = member_access.expression.as_ref()
                    {
                        identifier.name == "msg"
                    } else {
                        false
                    }
            })
        {
            self.has_msg_value = true;
        }

        Ok(())
    }
}

#[cfg(test)]
mod msg_value_in_loop_detector {

    use crate::detect::{
        detector::IssueDetector, high::msg_value_in_loops::MsgValueUsedInLoopDetector,
    };

    #[test]

    fn test_msg_value_in_loop() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MsgValueInLoop.sol",
        );

        let mut detector = MsgValueUsedInLoopDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 4);
    }
}
