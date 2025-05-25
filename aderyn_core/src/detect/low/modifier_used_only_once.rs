use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::{
    ast::NodeID,
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ModifierUsedOnlyOnceDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ModifierUsedOnlyOnceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut invocations: HashMap<i64, usize> = HashMap::new();

        for inv in context.modifier_invocations() {
            if let Some(id) = inv.modifier_name.referenced_declaration() {
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
        String::from("Modifier Invoked Only Once")
    }

    fn description(&self) -> String {
        String::from(
            "Consider removing the modifier or inlining the logic into the calling function.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ModifierUsedOnlyOnce)
    }
}

#[cfg(test)]
mod useless_modifier_tests {
    use crate::detect::detector::IssueDetector;

    use super::ModifierUsedOnlyOnceDetector;

    #[test]

    fn test_useless_modifier_tests_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/OnceModifierExample.sol",
        );

        let mut detector = ModifierUsedOnlyOnceDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
