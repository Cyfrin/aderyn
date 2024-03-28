use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{browser::ExtractIdentifiers, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnprotectedInitializerDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnprotectedInitializerDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for function in context.function_definitions() {
            if function.name.to_lowercase().contains("init") {
                let has_modifiers = !function.modifiers.is_empty();
                if !has_modifiers {
                    let identifiers = ExtractIdentifiers::from(function).extracted;
                    if !identifiers
                        .iter()
                        .any(|x| x.name == "revert" || x.name == "require")
                    {
                        capture!(self, context, function);
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unprotected initializer")
    }

    fn description(&self) -> String {
        String::from("Consider protecting the initializer functions with modifiers.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnprotectedInitializer)
    }
}

#[cfg(test)]
mod unprotected_initializer {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::UnprotectedInitializerDetector;

    #[test]
    fn test_unprotected_initializer() {
        let context = load_contract(
            "../tests/contract-playground/out/UnprotectedInitialize.sol/InitializedContract.json",
        );

        let mut detector = UnprotectedInitializerDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // println!("{:?}", detector.instances());
        assert_eq!(detector.instances().len(), 1);
    }
}
