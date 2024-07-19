use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::audit::investigators::SimpleInvestigator;
use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::visitor::ast_visitor::ASTConstVisitor;
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
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);

        for func in helpers::get_implemented_external_and_public_functions(context) {
            let mut tracker = MsgSenderAndCallWithValueTracker::default();
            let investigator = SimpleInvestigator::for_node(func, context)?;
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
        String::from("Description of the high issue.")
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
    fn test_template_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/auditor_mode/send_ether_no_checks/Tower.sol",
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
            String::from("Description of the high issue.")
        );
    }
}

/// Our first "Tracker" here !
/// Given a `FunctionDefinition` as an entry point, this keeps track of whether there is possibility for
/// 1. Binary operations with `msg.sender`
/// 2. Sending of native ETH value with `(...).call{value: XXX}()`
///
/// It does so, by seeking help from investigators module.
///
/// NOTE - Conceptually, Tracker is very similar to Extractor. Trackers are more logical and less "dumb".
/// Trackers don't just store everything. They calculate certain values and tracks them while being visited.
#[derive(Default)]
pub struct MsgSenderAndCallWithValueTracker {
    pub has_msg_sender_checks: bool,
    pub sends_native_eth: bool,
}

impl ASTConstVisitor for MsgSenderAndCallWithValueTracker {
    fn visit_modifier_definition(
        &mut self,
        node: &crate::ast::ModifierDefinition,
    ) -> eyre::Result<bool> {
        if !self.has_msg_sender_checks && helpers::has_msg_sender_binary_operation(&node.into()) {
            self.has_msg_sender_checks = true;
        }
        if !self.sends_native_eth && helpers::has_calls_that_sends_native_eth(&node.into()) {
            self.sends_native_eth = true;
        }
        eyre::Ok(true)
    }
    fn visit_function_definition(
        &mut self,
        node: &crate::ast::FunctionDefinition,
    ) -> eyre::Result<bool> {
        if !self.has_msg_sender_checks && helpers::has_msg_sender_binary_operation(&node.into()) {
            self.has_msg_sender_checks = true;
        }
        if !self.sends_native_eth && helpers::has_calls_that_sends_native_eth(&node.into()) {
            self.sends_native_eth = true;
        }
        eyre::Ok(true)
    }
}
