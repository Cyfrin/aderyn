use std::collections::BTreeMap;
use std::convert::identity;
use std::error::Error;

use crate::ast::{ASTNode, Expression, MemberAccess, NodeID};

use crate::capture;
use crate::context::browser::ExtractFunctionCalls;
use crate::context::investigator::{
    StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct HighLevelCallsInLoopDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/**
    This [`HighLevelCallsInLoopDetector`] detector will catch address.transfer as well as high level calls in loops
    (as they may potentially revert) and we don't specify "revert" explicitly.
    1. address.transfer
    2. Contract.xyz()

    Things what we don't capture - `address.send`, `address.call`, etc
    (Because we know for sure they don't revert unless we ask them to based on the return value of bool success)

    The solution is to perform this function call with low level
    call and then you can just break off the loop if the bool success returns false!

    In order to catch `requires` and `reverts` in the loops, there is another detector we have in `reverts_and_requires_in_loop.rs`
*/
impl IssueDetector for HighLevelCallsInLoopDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for for_statement in context.for_statements() {
            if has_high_level_calls_or_tranfers_eth_to_an_address(context, &(for_statement.into()))
                .is_some_and(identity)
            {
                capture!(self, context, for_statement);
            }
        }

        for while_statement in context.while_statements() {
            if has_high_level_calls_or_tranfers_eth_to_an_address(
                context,
                &(while_statement.into()),
            )
            .is_some_and(identity)
            {
                capture!(self, context, while_statement);
            }
        }

        for do_while_statement in context.do_while_statements() {
            if has_high_level_calls_or_tranfers_eth_to_an_address(
                context,
                &(do_while_statement.into()),
            )
            .is_some_and(identity)
            {
                capture!(self, context, do_while_statement);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Calls inside a loop might lead to a denial-of-service attack.")
    }

    fn description(&self) -> String {
        String::from("If one of the destinations has a fallback function that reverts, it will cause the whole transaction to revert. Favor pull over push for external calls.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::HighLevelCallsInLoop.to_string()
    }
}

fn has_high_level_calls_or_tranfers_eth_to_an_address(
    context: &WorkspaceContext,
    ast_node: &ASTNode,
) -> Option<bool> {
    let mut tracker = HighLevelCallsAndAddressTransferTracker::default();
    let investigator =
        StandardInvestigator::new(context, &[ast_node], StandardInvestigationStyle::Downstream)
            .ok()?;

    investigator.investigate(context, &mut tracker).ok()?;
    Some(tracker.satisfied())
}

#[derive(Default)]
struct HighLevelCallsAndAddressTransferTracker {
    has_address_transfer: bool,
    has_high_level_call: bool,
}

impl HighLevelCallsAndAddressTransferTracker {
    fn satisfied(&self) -> bool {
        self.has_high_level_call || self.has_address_transfer
    }
}

impl StandardInvestigatorVisitor for HighLevelCallsAndAddressTransferTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        if self.satisfied() {
            return Ok(());
        }

        let function_calls = ExtractFunctionCalls::from(node).extracted;

        for func in function_calls {
            if let Expression::MemberAccess(MemberAccess {
                member_name,
                expression,
                ..
            }) = func.expression.as_ref()
            {
                let is_called_on_address =
                    expression.type_descriptions().is_some_and(|type_desc| {
                        type_desc.type_string.as_ref().is_some_and(|type_string| {
                            type_string == "address" || type_string == "address payable"
                        })
                    });

                let is_called_on_contract =
                    expression.type_descriptions().is_some_and(|type_desc| {
                        type_desc
                            .type_string
                            .as_ref()
                            .is_some_and(|type_string| type_string.starts_with("contract "))
                    });

                if is_called_on_address && member_name == "transfer" {
                    self.has_address_transfer = true;
                }

                if is_called_on_contract {
                    self.has_high_level_call = true;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod high_level_calls_in_loop_detector {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::high_level_calls_in_loop::HighLevelCallsInLoopDetector,
    };

    #[test]
    #[serial]
    fn test_high_level_calls_in_loop() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/HighLevelCallsInsideLoop.sol",
        );

        let mut detector = HighLevelCallsInLoopDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
    }
}
