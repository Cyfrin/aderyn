use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::context::graph::{CallGraph, CallGraphDirection, CallGraphVisitor};
use crate::context::workspace_context::ASTNode;
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
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
        for func in helpers::get_implemented_external_and_public_functions(context) {
            let mut tracker = AddressChecksAndCallWithValueTracker::default();
            let callgraph = CallGraph::new(context, &[&(func.into())], CallGraphDirection::Inward)?;
            callgraph.accept(context, &mut tracker)?;

            if tracker.sends_native_eth && !tracker.has_binary_checks_on_some_address {
                capture!(self, context, func);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Functions send eth away from contract but performs no checks on any address.")
    }

    fn description(&self) -> String {
        String::from("Consider introducing checks for `msg.sender` to ensure the recipient of the money is as intended.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::SendsEtherAwayWithoutCheckingAddress.to_string()
    }
}

#[cfg(test)]
mod send_ether_no_checks_detector_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::send_ether_no_checks::SendEtherNoChecksDetector,
    };

    #[test]
    #[serial]
    fn test_send_ether_no_checks() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/SendEtherNoChecks.sol",
        );

        let mut detector = SendEtherNoChecksDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 3);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
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
