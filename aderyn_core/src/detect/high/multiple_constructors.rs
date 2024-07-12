use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::context::browser::ExtractFunctionDefinitions;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the MultipleConstructorsDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct MultipleConstructorsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for MultipleConstructorsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let contracts_with_multiple_constructors = context
            .contract_definitions()
            .into_iter()
            .filter(|&contract| {
                ExtractFunctionDefinitions::from(contract)
                    .extracted
                    .iter()
                    .filter(|function| function.is_constructor)
                    .count()
                    > 1
            })
            .collect::<Vec<_>>();

        for contract in contracts_with_multiple_constructors {
            capture!(self, context, contract);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Contract Has Multiple Constructors")
    }

    fn description(&self) -> String {
        String::from("In some versions of Solidity, contracts compile with multiple constructors. The first constructor takes precedence. This can lead to unexpected behavior.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::MultipleConstructors.to_string()
    }
}

#[cfg(test)]
mod multiple_constructors_detector_tests {
    use crate::detect::{detector::IssueDetector, high::MultipleConstructorsDetector};

    #[test]
    fn test_multiple_constructors_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MultipleConstructorSchemes.sol",
        );

        let mut detector = MultipleConstructorsDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Contract Has Multiple Constructors")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("In some versions of Solidity, contracts compile with multiple constructors. The first constructor takes precedence. This can lead to unexpected behavior.")
        );
    }

    #[test]
    fn test_multiple_constructors_detector_no_issue() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ArbitraryTransferFrom.sol",
        );

        let mut detector = MultipleConstructorsDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector did not find an issue
        assert!(!found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 0);
    }
}
