use std::{collections::BTreeMap, error::Error};

use crate::ast::{FunctionKind, NodeID};

use crate::{
    capture,
    context::{browser::ExtractFunctionDefinitions, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

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
                    .filter(|function| function.kind() == &FunctionKind::Constructor)
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
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]

    fn test_multiple_constructors_detector_no_issue() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ArbitraryTransferFrom.sol",
        );

        let mut detector = MultipleConstructorsDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
        assert_eq!(detector.instances().len(), 0);
    }
}
