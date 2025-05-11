use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::{
    capture,
    context::{
        browser::ExtractModifierInvocations,
        graph::CallGraphVisitor,
        workspace::{ASTNode, WorkspaceContext},
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct SendEtherNoChecksDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for SendEtherNoChecksDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for (func, callgraphs) in context.entrypoints_with_callgraphs() {
            for callgraph in callgraphs {
                let mut tracker = AddressChecksAndCallWithValueTracker::default();
                callgraph.accept(context, &mut tracker)?;

                // Hacky way to check if the modifier is a know msg.sender checking modifier
                // This is because our Callgraph doesn't navigate inside contracts that are outside
                // the scope, this includes imported contracts.
                let has_oz_modifier =
                    ExtractModifierInvocations::from(func).extracted.iter().any(|invocation| {
                        invocation.modifier_name.name().contains("onlyRole")
                            || invocation.modifier_name.name() == "onlyOwner"
                            || invocation.modifier_name.name() == "requiresAuth"
                    });

                if tracker.sends_native_eth
                    && !tracker.has_binary_checks_on_some_address
                    && !has_oz_modifier
                {
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
        String::from("ETH transferred without address checks")
    }

    fn description(&self) -> String {
        String::from("Consider introducing checks for `msg.sender` to ensure the recipient of the money is as intended.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::EthSendUncheckedAddress.to_string()
    }
}

#[derive(Default)]
pub struct AddressChecksAndCallWithValueTracker {
    pub has_binary_checks_on_some_address: bool,
    pub sends_native_eth: bool,
}

impl CallGraphVisitor for AddressChecksAndCallWithValueTracker {
    fn visit_any(&mut self, node: &ASTNode) -> eyre::Result<()> {
        if !self.has_binary_checks_on_some_address
            && helpers::has_binary_checks_on_some_address(node)
        {
            self.has_binary_checks_on_some_address = true;
        }
        if !self.sends_native_eth && helpers::has_calls_that_sends_native_eth(node) {
            self.sends_native_eth = true;
        }
        eyre::Ok(())
    }
}

#[cfg(test)]
mod send_ether_no_checks_detector_tests {

    use crate::detect::{
        detector::IssueDetector, high::eth_send_unchecked_address::SendEtherNoChecksDetector,
    };

    #[test]
    fn test_send_ether_no_checks_lib_import() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/SendEtherNoChecksLibImport.sol",
        );

        let mut detector = SendEtherNoChecksDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
        assert_eq!(detector.instances().len(), 0);
    }

    #[test]
    fn test_send_ether_no_checks() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/SendEtherNoChecks.sol",
        );

        let mut detector = SendEtherNoChecksDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 3);
    }
}
