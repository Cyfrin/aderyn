use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::context::investigator::{
    StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
};
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
            let mut tracker = MsgSenderAndCallWithValueTracker::default();
            let investigator = StandardInvestigator::new(
                context,
                &[&(func.into())],
                StandardInvestigationStyle::Downstream,
            )?;
            investigator.investigate(context, &mut tracker)?;

            if tracker.sends_native_eth && !tracker.has_msg_sender_checks {
                capture!(self, context, func);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Sending native Eth is not protected from these functions.")
    }

    fn description(&self) -> String {
        String::from("Introduce checks for `msg.sender` in the function")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::SendEtherNoChecks.to_string()
    }
}

#[cfg(test)]
mod send_ether_no_checks_detector_tests {
    use crate::detect::{
        detector::IssueDetector, high::send_ether_no_checks::SendEtherNoChecksDetector,
    };

    #[test]
    fn test_send_ether_no_checks() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/SendEtherNoChecks.sol",
        );

        let mut detector = SendEtherNoChecksDetector::default();
        let found = detector.detect(&context).unwrap();
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
            String::from("Sending native Eth is not protected from these functions.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Introduce checks for `msg.sender` in the function")
        );
    }
}

#[derive(Default)]
pub struct MsgSenderAndCallWithValueTracker {
    pub has_msg_sender_checks: bool,
    pub sends_native_eth: bool,
}

impl StandardInvestigatorVisitor for MsgSenderAndCallWithValueTracker {
    fn visit_fallback(&mut self, node: &ASTNode) -> eyre::Result<()> {
        if !self.has_msg_sender_checks && helpers::has_msg_sender_binary_operation(node) {
            self.has_msg_sender_checks = true;
        }
        if !self.sends_native_eth && helpers::has_calls_that_sends_native_eth(node) {
            self.sends_native_eth = true;
        }
        eyre::Ok(())
    }
}
