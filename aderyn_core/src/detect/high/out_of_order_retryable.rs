use std::{collections::BTreeMap, error::Error};

use crate::ast::{Expression, MemberAccess, NodeID};

use crate::{
    capture,
    context::{
        browser::ExtractFunctionCalls,
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct OutOfOrderRetryableDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for OutOfOrderRetryableDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for func in helpers::get_implemented_external_and_public_functions(context) {
            let callgraphs =
                CallGraphConsumer::get(context, &[&(func.into())], CallGraphDirection::Inward)?;
            for callgraph in callgraphs {
                let mut tracker = OutOfOrderRetryableTracker { number_of_retry_calls: 0 };
                callgraph.accept(context, &mut tracker)?;
                if tracker.number_of_retry_calls >= 2 {
                    capture!(self, context, func);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Out of Order Retryable Transaction")
    }

    fn description(&self) -> String {
        String::from("Do not rely on the order or successful execution of retryable tickets. Functions like \
            createRetryableTicket, outboundTransferCustomRefund, unsafeCreateRetryableTicket are free to be re-tried in any
            order if they fail in the first go. Since this operation happens off chain, the sequencer is in control of the
            order of these transactions. Therefore, restrict the use to at most 1 ticket call per function.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::OutOfOrderRetryable)
    }
}

struct OutOfOrderRetryableTracker {
    number_of_retry_calls: usize,
}

const SEQUENCER_FUNCTIONS: [&str; 3] =
    ["createRetryableTicket", "outboundTransferCustomRefund", "unsafeCreateRetryableTicket"];

impl CallGraphVisitor for OutOfOrderRetryableTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        if self.number_of_retry_calls >= 2 {
            return Ok(());
        }
        let function_calls = ExtractFunctionCalls::from(node).extracted;
        for func_call in function_calls {
            if let Expression::MemberAccess(MemberAccess { member_name, .. }) =
                func_call.expression.as_ref()
            {
                if SEQUENCER_FUNCTIONS.iter().any(|f| f == member_name) {
                    self.number_of_retry_calls += 1;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod out_of_order_retryable_tests {

    use crate::detect::{
        detector::IssueDetector, high::out_of_order_retryable::OutOfOrderRetryableDetector,
    };

    #[test]

    fn test_out_of_order_retryable() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/OutOfOrderRetryable.sol",
        );

        let mut detector = OutOfOrderRetryableDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
