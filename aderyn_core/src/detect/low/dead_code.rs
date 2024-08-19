use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{ASTNode, ContractKind, NodeID, NodeType, Visibility};

use crate::capture;
use crate::context::browser::GetClosestAncestorOfTypeX;
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DeadCodeDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DeadCodeDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Heuristic:
        // Internal non overriding functions inside of non abstract contracts that have a body (implemented) and are not used
        // If an internal function is marked override then, it may still be used even if it doesn't have a direct referencedDeclaration
        // pointing to it.

        for func in context
            .function_definitions()
            .into_iter()
            .filter(|&f| {
                f.overrides.is_none()
                    && f.implemented
                    && f.visibility == Visibility::Internal
                    && !f.is_constructor
            })
            .filter(|&f| {
                if let Some(ASTNode::ContractDefinition(contract)) =
                    f.closest_ancestor_of_type(context, NodeType::ContractDefinition)
                {
                    if contract.kind == ContractKind::Contract {
                        match contract.is_abstract {
                            Some(false) | None => {
                                return true;
                            }
                            _ => (),
                        }
                    }
                }
                false
            })
        {
            if helpers::count_identifiers_that_reference_an_id(context, func.id) == 0 {
                capture!(self, context, func);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Dead Code")
    }

    fn description(&self) -> String {
        String::from("Functions that are not used. Consider removing them.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::DeadCode)
    }
}

#[cfg(test)]
mod dead_code_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, low::dead_code::DeadCodeDetector};

    #[test]
    #[serial]
    fn test_dead_code() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DeadCode.sol",
        );

        let mut detector = DeadCodeDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
