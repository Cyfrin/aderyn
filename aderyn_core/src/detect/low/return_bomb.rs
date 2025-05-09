use std::{collections::BTreeMap, error::Error};

use crate::ast::{ASTNode, MemberAccess, NodeID};

use crate::{
    ast::NodeType,
    capture,
    context::{
        browser::GetClosestAncestorOfTypeX,
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
pub struct ReturnBombDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ReturnBombDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // Look for calls on addresses that are unprotected. (non state variable address that has
        // not undergone any binary checks)

        // Capture the ones where no gas limit  is explicitly set *and* there is a `returndatacopy`
        // operation Basially you are checking for the 2nd element in the tuple - (bool
        // success, bytes memory ret) which invokes the above operation.

        for func in helpers::get_implemented_external_and_public_functions(context) {
            let callgraphs =
                CallGraphConsumer::get(context, &[&(func.into())], CallGraphDirection::Inward)?;
            for callgraph in callgraphs {
                let mut tracker = CallNoAddressChecksTracker {
                    has_address_checks: false,
                    calls_on_non_state_variable_addresses: vec![], /* collection of all
                                                                    * `address.call` Member
                                                                    * Accesses
                                                                    * where address is not a
                                                                    * state
                                                                    * variable */
                    context,
                };
                callgraph.accept(context, &mut tracker)?;

                if !tracker.has_address_checks {
                    // Now we assume that in this region all addresses are unprotected (because they
                    // are not involved in any binary ops/checks)
                    for member_access in tracker.calls_on_non_state_variable_addresses {
                        // Now we need to see if address.call{gas: xxx}() has been called with
                        // options and if so, scan to see if the gaslimit is
                        // set. If it is, then it is not a vulnerability
                        // because OOG is likely not possible when there is
                        // defined gas limit Therefore, continue the for
                        // loop and investigate other instances

                        if let Some(ASTNode::FunctionCallOptions(function_call_ops)) = member_access
                            .closest_ancestor_of_type(context, NodeType::FunctionCallOptions)
                        {
                            if function_call_ops.names.contains(&String::from("gas")) {
                                continue;
                            }
                        }

                        // Here, we know that there is no gas limit set for the call. So we need to
                        // only check for the cases where `returndatacopy`
                        // happens and then capture it.

                        if let Some(ASTNode::FunctionCall(function_call)) =
                            member_access.closest_ancestor_of_type(context, NodeType::FunctionCall)
                        {
                            // In this case there are no options like gas, etc, passed to the
                            // `address.call()` So we need to check if
                            // `returndatacopy` is triggered. If yes, then it is a problem

                            if let Some(ASTNode::Assignment(assignment)) = function_call
                                .closest_ancestor_of_type(context, NodeType::Assignment)
                            {
                                // The following check will ensure that the last parameter which is
                                // `bytes memory retData` is not unpacked.
                                // (there is nothing after comma)
                                if !assignment.left_hand_side.type_descriptions().is_some_and(
                                    |type_desc| {
                                        type_desc
                                            .type_string
                                            .as_ref()
                                            .is_some_and(|type_string| type_string.ends_with(",)"))
                                    },
                                ) {
                                    capture!(self, context, assignment);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Return Bomb")
    }

    fn description(&self) -> String {
        String::from("A low level callee may consume all callers gas unexpectedly. Avoid unlimited implicit decoding of returndata on \
            calls to unchecked addresses. You can limit the gas by passing a gas limit as an option to the call. For example, \
            `unknownAddress.call{gas: gasLimitHere}(\"calldata\")` That would act as a safety net from OOG errors.
        ")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ReturnBomb)
    }
}

struct CallNoAddressChecksTracker<'a> {
    has_address_checks: bool,
    calls_on_non_state_variable_addresses: Vec<MemberAccess>,
    context: &'a WorkspaceContext,
}

impl CallGraphVisitor for CallNoAddressChecksTracker<'_> {
    fn visit_any(&mut self, node: &crate::context::workspace::ASTNode) -> eyre::Result<()> {
        if !self.has_address_checks && helpers::has_binary_checks_on_some_address(node) {
            self.has_address_checks = true;
        }
        self.calls_on_non_state_variable_addresses.extend(
            helpers::get_low_level_calls_on_non_state_variable_addresses(node, self.context),
        );
        self.calls_on_non_state_variable_addresses.dedup();
        eyre::Ok(())
    }
}

#[cfg(test)]
mod return_bomb_detector_tests {

    use crate::detect::{detector::IssueDetector, low::return_bomb::ReturnBombDetector};

    #[test]

    fn test_return_bomb_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ReturnBomb.sol",
        );

        let mut detector = ReturnBombDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        assert_eq!(detector.instances().len(), 1);
    }
}
