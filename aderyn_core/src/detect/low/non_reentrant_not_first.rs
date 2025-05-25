use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct NonReentrantBeforeOthersDetector {
    // Keys are source file name, line number and source location
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for NonReentrantBeforeOthersDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let function_definitions = context.function_definitions();
        for definition in function_definitions {
            if definition.modifiers.len() > 1 {
                for (index, modifier) in definition.modifiers.iter().enumerate() {
                    if modifier.modifier_name.name().to_lowercase().contains("nonreentrant")
                        && index != 0
                    {
                        capture!(self, context, modifier);
                    }
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`nonReentrant` is Not the First Modifier")
    }

    fn description(&self) -> String {
        String::from("To protect against reentrancy in other modifiers, the `nonReentrant` modifier should be the first modifier in the list of modifiers.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::NonReentrantNotFirst)
    }
}

#[cfg(test)]
mod non_reentrant_before_others_tests {

    use crate::detect::{detector::IssueDetector, low::NonReentrantBeforeOthersDetector};

    #[test]

    fn test_non_reentrant_before_others_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/AdminContract.sol",
        );

        let mut detector = NonReentrantBeforeOthersDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);

        // assert that the line number is 10
        let (_, line_number, _) = detector.instances().keys().next().unwrap().clone();
        assert_eq!(line_number, 10);
    }
}
