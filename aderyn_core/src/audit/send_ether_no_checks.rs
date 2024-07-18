use prettytable::{row, Row};

use super::{auditor::AuditorDetector, investigators::SimpleInvestigator};
use crate::{
    context::workspace_context::WorkspaceContext, detect::helpers,
    visitor::ast_visitor::ASTConstVisitor,
};
use std::error::Error;

#[derive(Debug)]
struct SendEtherNoChecksInstance {
    pub filename: String,
    pub line_no: usize,
    pub func_name: String,
}

impl SendEtherNoChecksInstance {
    fn encode_from(func_name: String, node_key: (String, usize, String)) -> Self {
        let (filename, line_no, _) = node_key;
        Self {
            filename,
            line_no,
            func_name,
        }
    }
}

#[derive(Default)]
pub struct SendEtherWithoutMsgSenderChecksDetector {
    found_instances: Vec<SendEtherNoChecksInstance>,
}

impl AuditorDetector for SendEtherWithoutMsgSenderChecksDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for func in helpers::get_implemented_external_and_public_functions(context) {
            let mut tracker = MsgSenderAndCallWithValueTracker::default();
            let investigator = SimpleInvestigator::for_node(func, context)?;
            investigator.investigate(context, &mut tracker)?;

            if tracker.sends_native_eth && !tracker.has_msg_sender_checks {
                self.found_instances
                    .push(SendEtherNoChecksInstance::encode_from(
                        func.name.to_owned(),
                        context.get_node_sort_key_pure(&func.into()),
                    ));
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Sending native Eth is not protected from these functions.")
    }

    fn table_titles(&self) -> Row {
        row!["Filename", "Line No", "Function"]
    }

    fn table_rows(&self) -> Vec<Row> {
        self.found_instances
            .iter()
            .map(|instance| row![instance.filename, instance.line_no, instance.func_name])
            .collect()
    }

    fn skeletal_clone(&self) -> Box<dyn AuditorDetector> {
        Box::<SendEtherWithoutMsgSenderChecksDetector>::default()
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

#[cfg(test)]
mod send_ether_no_checks_detector {
    use crate::audit::{
        auditor::AuditorDetector, send_ether_no_checks::SendEtherWithoutMsgSenderChecksDetector,
    };

    #[test]
    fn test_send_ether_no_checks_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/auditor_mode/send_ether_no_checks/Tower.sol",
        );

        let mut detector = SendEtherWithoutMsgSenderChecksDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:#?}", detector.found_instances);

        assert!(found);
    }
}
