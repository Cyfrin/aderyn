use crate::{
    ast::NodeID,
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use std::{collections::BTreeMap, error::Error};

#[derive(Default)]
pub struct StateVariableInitOrder {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for StateVariableInitOrder {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        if context.via_ir {}

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State variable's initial value relies on constructor of another contract.")
    }

    fn description(&self) -> String {
        String::from(
            "With via_ir flag is enabled, there is different behavior in contracts where initial value \
            of a state variable relies on the result of the constructor in another contract.\
            \nhttps://docs.soliditylang.org/en/latest/ir-breaking-changes.html#semantic-only-changes",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::StateVariableInitOrder.to_string()
    }
}

#[cfg(test)]
mod state_variable_init_order {
    use super::*;

    #[test]
    fn test_state_variable_init_ordering() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/via-ir-enabled/src/SemanticOrdering.sol",
        );

        let mut detector = StateVariableInitOrder::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
