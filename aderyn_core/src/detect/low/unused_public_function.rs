use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{FunctionKind, NodeID, Visibility},
    capture,
    context::{
        browser::GetClosestAncestorOfTypeX,
        workspace::{ASTNode, WorkspaceContext},
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::count_identifiers_that_reference_an_id,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct UnusedPublicFunctionDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnusedPublicFunctionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let unreferenced_public_functions =
            context.function_definitions().into_iter().filter(|&function| {
                matches!(function.visibility, Visibility::Public)
                    && !matches!(function.kind(), &FunctionKind::Constructor)
                    && function.overrides.is_none()
                    && !function.is_virtual
                    && count_identifiers_that_reference_an_id(context, function.id) == 0
            });

        for unreferenced_public_function in unreferenced_public_functions {
            if let Some(ASTNode::ContractDefinition(parent_contract)) = unreferenced_public_function
                .closest_ancestor_of_type(context, crate::ast::NodeType::ContractDefinition)
            {
                if parent_contract.is_abstract {
                    continue;
                }
            }
            capture!(self, context, unreferenced_public_function);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Public Function Not Used Internally")
    }

    fn description(&self) -> String {
        String::from("If a function is marked public but is not used internally, consider marking it as `external`.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnusedPublicFunction)
    }
}

#[cfg(test)]
mod useless_public_function_tests {
    use crate::detect::detector::IssueDetector;

    use super::UnusedPublicFunctionDetector;

    #[test]

    fn test_useless_public_functions_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/Counter.sol",
        );

        let mut detector = UnusedPublicFunctionDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]

    fn test_useless_public_functions_does_not_capture_abstract_contract_functions() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/AbstractContract.sol",
        );

        let mut detector = UnusedPublicFunctionDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
        assert_eq!(detector.instances().len(), 0);
    }

    #[test]

    fn test_useless_public_functions_does_not_capture_virtual_or_overriding_functions() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/PublicFunction.sol",
        );

        let mut detector = UnusedPublicFunctionDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
        assert_eq!(detector.instances().len(), 0);
    }
}
