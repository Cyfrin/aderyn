use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{ContractKind, NodeID, NodeType},
    capture,
    context::{
        browser::{ExtractContractDefinitions, GetClosestAncestorOfTypeX},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnspecificSolidityPragmaDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnspecificSolidityPragmaDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for pragma_directive in context.pragma_directives() {
            let Some(source_unit) =
                pragma_directive.closest_ancestor_of_type(context, NodeType::SourceUnit)
            else {
                continue;
            };
            let contracts_in_source_unit = ExtractContractDefinitions::from(source_unit).extracted;
            if contracts_in_source_unit.iter().any(|c| c.kind == ContractKind::Library) {
                continue;
            }
            for literal in &pragma_directive.literals {
                if literal.contains('^') || literal.contains('>') || literal.contains('<') {
                    capture!(self, context, pragma_directive);
                    break;
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unspecific Solidity Pragma")
    }

    fn description(&self) -> String {
        String::from("Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnspecificSolidityPragma)
    }
}

#[cfg(test)]
mod unspecific_solidity_pragma_tests {
    use crate::detect::{
        detector::IssueDetector, low::unspecific_solidity_pragma::UnspecificSolidityPragmaDetector,
    };

    #[test]

    fn test_unspecific_solidity_pragma_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/inheritance/IContractInheritance.sol",
        );

        let mut detector = UnspecificSolidityPragmaDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]

    fn test_unspecific_solidity_pragma_detector_by_loading_contract_directly_on_library() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/OnlyLibrary.sol",
        );

        let mut detector = UnspecificSolidityPragmaDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
    }
}
