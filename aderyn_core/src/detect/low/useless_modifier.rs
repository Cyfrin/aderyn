use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UselessModifierDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UselessModifierDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut invocations: HashMap<i64, usize> = HashMap::new();

        for inv in context.modifier_invocations() {
            if let Some(id) = inv.modifier_name.referenced_declaration {
                match invocations.entry(id) {
                    std::collections::hash_map::Entry::Occupied(mut o) => *o.get_mut() += 1,
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(1);
                    }
                };
            }
        }

        for modifier in context.modifier_definitions() {
            let count = *invocations.get(&modifier.id).unwrap_or(&0);
            if count == 1 {
                capture!(self, context, modifier);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Modifiers invoked only once can be shoe-horned into the function")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UselessModifier)
    }
}

#[cfg(test)]
mod useless_modifier_tests {
    use crate::detect::detector::IssueDetector;

    use super::UselessModifierDetector;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_useless_modifier_tests_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/OnceModifierExample.sol",
        );

        let mut detector = UselessModifierDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Modifiers invoked only once can be shoe-horned into the function")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
