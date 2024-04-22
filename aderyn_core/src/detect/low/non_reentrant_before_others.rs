use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
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
                    if modifier
                        .modifier_name
                        .name
                        .to_lowercase()
                        .contains("nonreentrant")
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
        String::from("The `nonReentrant` `modifier` should occur before all other modifiers")
    }

    fn description(&self) -> String {
        String::from("This is a best-practice to protect against reentrancy in other modifiers.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::NonReentrantBeforeOthers)
    }
}

#[cfg(test)]
mod non_reentrant_before_others_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, low::NonReentrantBeforeOthersDetector};

    #[test]
    #[serial]
    fn test_non_reentrant_before_others_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/AdminContract.sol",
        );

        let mut detector = NonReentrantBeforeOthersDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found something
        assert!(found);
        // assert that the detector found the correct number
        assert_eq!(detector.instances().len(), 1);

        // assert that the line number is 10
        let (_, line_number, _) = detector.instances().keys().next().unwrap().clone();
        assert_eq!(line_number, 10);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            "The `nonReentrant` `modifier` should occur before all other modifiers"
        );
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            "This is a best-practice to protect against reentrancy in other modifiers."
        );
    }
}
