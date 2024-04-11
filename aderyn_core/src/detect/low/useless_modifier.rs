use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UselessModifierDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UselessModifierDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for modifier in context.modifier_definitions() {
            let mut count = 0;
            for inv in context.modifier_invocations() {
                if let Some(id) = inv.modifier_name.referenced_declaration {
                    if id == modifier.id {
                        count += 1;
                    }
                }
            }
            if count == 1 {
                capture!(self, context, modifier);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Modifiers invoked only oLowe can be shoe-horned into the Function")
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
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::UselessModifierDetector;

    #[test]
    fn test_useless_modifier_tests() {
        let context = load_contract(
            "../tests/contract-playground/out/OnceModifierExample.sol/OnceModifierExample.json",
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
            String::from("Modifiers invoked only oLowe can be shoe-horned into the Function")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
