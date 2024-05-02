use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{
        browser::GetClosestAncestorOfTypeX,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DeprecatedOZFunctionsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DeprecatedOZFunctionsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call identifier.accept(self)
            if let Some(ASTNode::SourceUnit(source_unit)) =
                identifier.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                let import_directives = source_unit.import_directives();
                if import_directives.iter().any(|directive| {
                    directive
                        .absolute_path
                        .as_ref()
                        .map_or(false, |path| path.contains("openzeppelin"))
                }) && identifier.name == "_setupRole"
                {
                    capture!(self, context, identifier);
                }
            } else {
                // Optional: handle other cases, or do nothing
            }
        }
        for member_access in context.member_accesses() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call member_access.accept(self)
            if let Some(ASTNode::SourceUnit(source_unit)) =
                member_access.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                let import_directives = source_unit.import_directives();
                if import_directives.iter().any(|directive| {
                    directive
                        .absolute_path
                        .as_ref()
                        .map_or(false, |path| path.contains("openzeppelin"))
                }) && member_access.member_name == "safeApprove"
                {
                    capture!(self, context, member_access);
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Deprecated OpenZeppelin functions should not be used")
    }

    fn description(&self) -> String {
        String::from("Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::DeprecatedOzFunctions)
    }
}

#[cfg(test)]
mod deprecated_oz_functions_tests {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::DeprecatedOZFunctionsDetector;

    #[test]
    #[serial]
    fn test_deprecated_oz_functions_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DeprecatedOZFunctions.sol",
        );

        let mut detector = DeprecatedOZFunctionsDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Deprecated OpenZeppelin functions should not be used")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/"
            )
        );
    }
}
