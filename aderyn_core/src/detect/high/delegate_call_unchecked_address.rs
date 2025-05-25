use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::capture;

use crate::{
    context::{graph::CallGraphVisitor, workspace::WorkspaceContext},
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct DelegateCallUncheckedAddressDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DelegateCallUncheckedAddressDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        struct DelegateCallNoAddressChecksTracker<'a> {
            has_address_checks: bool,
            has_delegate_call_on_non_state_variable_address: bool,
            context: &'a WorkspaceContext,
        }

        impl CallGraphVisitor for DelegateCallNoAddressChecksTracker<'_> {
            fn visit_any(&mut self, node: &crate::context::workspace::ASTNode) -> eyre::Result<()> {
                if !self.has_address_checks && helpers::has_binary_checks_on_some_address(node) {
                    self.has_address_checks = true;
                }
                if !self.has_delegate_call_on_non_state_variable_address
                    && helpers::has_delegate_calls_on_non_state_variables(node, self.context)
                {
                    self.has_delegate_call_on_non_state_variable_address = true;
                }
                eyre::Ok(())
            }
        }

        for (func, callgraphs) in context.entrypoints_with_callgraphs() {
            for callgraph in callgraphs {
                let mut tracker = DelegateCallNoAddressChecksTracker {
                    has_address_checks: false,
                    has_delegate_call_on_non_state_variable_address: false,
                    context,
                };
                callgraph.accept(context, &mut tracker)?;

                if tracker.has_delegate_call_on_non_state_variable_address
                    && !tracker.has_address_checks
                {
                    capture!(self, context, func)
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("`delegatecall` to an Arbitrary Address")
    }

    fn description(&self) -> String {
        String::from("Making a `delegatecall` to an arbitrary address without any checks is dangerous. Consider adding requirements on the target address.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::DelegateCallUncheckedAddress.to_string()
    }
}

#[cfg(test)]
mod delegate_call_no_address_check_tests {

    use crate::detect::{
        detector::IssueDetector,
        high::delegate_call_unchecked_address::DelegateCallUncheckedAddressDetector,
    };

    #[test]
    fn test_delegate_call_without_checks() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DelegateCallWithoutAddressCheck.sol",
        );

        let mut detector = DelegateCallUncheckedAddressDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);

        assert_eq!(detector.instances().len(), 1);
    }
}
