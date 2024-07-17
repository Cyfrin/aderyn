use prettytable::{row, Row};

use super::auditor::AuditorDetector;
use crate::context::workspace_context::WorkspaceContext;
use std::error::Error;

struct SendEtherNoChecksInstance {
    ether_sending_call: String,
}

#[derive(Default)]
pub struct SendEtherWithoutMsgSenderChecksDetector {
    found_instances: Vec<SendEtherNoChecksInstance>,
}

impl AuditorDetector for SendEtherWithoutMsgSenderChecksDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Sending native Eth is not protected")
    }

    fn table_titles(&self) -> Row {
        row!["Code"]
    }

    fn table_rows(&self) -> Vec<Row> {
        self.found_instances
            .iter()
            .map(|instance| row![instance.ether_sending_call])
            .collect()
    }

    fn skeletal_clone(&self) -> Box<dyn AuditorDetector> {
        Box::<SendEtherWithoutMsgSenderChecksDetector>::default()
    }
}

#[cfg(test)]
mod send_ether_no_checks_detector {
    use crate::audit::{
        auditor::AuditorDetector, send_ether_no_checks::SendEtherWithoutMsgSenderChecksDetector,
    };

    #[test]
    fn test_attack_surface_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/auditor_mode/ExternalCalls.sol",
        );

        let mut detector = SendEtherWithoutMsgSenderChecksDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
    }
}
