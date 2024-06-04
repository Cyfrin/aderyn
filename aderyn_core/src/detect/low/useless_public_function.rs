use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{FunctionKind, NodeID, Visibility},
    capture,
    context::{
        browser::GetClosestAncestorOfTypeX,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::count_identifiers_that_reference_an_id,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct UselessPublicFunctionDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UselessPublicFunctionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let unreferenced_public_functions =
            context
                .function_definitions()
                .into_iter()
                .filter(|&function| {
                    matches!(function.visibility, Visibility::Public)
                        && !matches!(function.kind, FunctionKind::Constructor)
                        && count_identifiers_that_reference_an_id(context, function.id) == 0
                });

        for unreferenced_public_function in unreferenced_public_functions {
            if let Some(ASTNode::ContractDefinition(parent_contract)) = unreferenced_public_function
                .closest_ancestor_of_type(context, crate::ast::NodeType::ContractDefinition)
            {
                if let Some(is_abstract) = parent_contract.is_abstract {
                    if is_abstract {
                        continue;
                    }
                }
            }
            capture!(self, context, unreferenced_public_function);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("`public` functions not used internally could be marked `external`")
    }

    fn description(&self) -> String {
        String::from("Instead of marking a function as `public`, consider marking it as `external` if it is not used internally.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UselessPublicFunction)
    }
}

#[cfg(test)]
mod useless_public_function_tests {
    use crate::detect::detector::IssueDetector;

    use super::UselessPublicFunctionDetector;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_useless_public_functions_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/Counter.sol",
        );

        let mut detector = UselessPublicFunctionDetector::default();
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
            String::from("`public` functions not used internally could be marked `external`")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from("Instead of marking a function as `public`, consider marking it as `external` if it is not used internally."));
    }

    #[test]
    #[serial]
    fn test_useless_public_functions_does_not_capture_abstract_contract_functions() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/AbstractContract.sol",
        );

        let mut detector = UselessPublicFunctionDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(!found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 0);
    }
}
