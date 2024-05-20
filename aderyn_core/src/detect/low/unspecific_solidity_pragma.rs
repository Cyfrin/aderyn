use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
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
            for literal in &pragma_directive.literals {
                if literal.contains('^') || literal.contains('>') {
                    capture!(self, context, pragma_directive);
                    break;
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Solidity pragma should be specific, not wide")
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
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_unspecific_solidity_pragma_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/inheritance/IContractInheritance.sol",
        );

        let mut detector = UnspecificSolidityPragmaDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct abi encode packed
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 1);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Solidity pragma should be specific, not wide")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`"
            )
        );
    }
}
