use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{ContractKind, NodeID, Visibility},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct LibraryHasPublicOrExternalFunctionsDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for LibraryHasPublicOrExternalFunctionsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context
            .contract_definitions()
            .iter()
            .filter(|x| x.kind == ContractKind::Library)
        {
            for function in contract.function_definitions() {
                if function.visibility == Visibility::Public
                    || function.visibility == Visibility::External
                {
                    capture!(self, context, function);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Library has public or external function")
    }

    fn description(&self) -> String {
        String::from("Public / external functions are not ideal to have in libraries as they are not designed to hold state and this can be misleading.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::LargeNumericLiteral)
    }
}

#[cfg(test)]
mod library_has_public_or_external_functions {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::LibraryHasPublicOrExternalFunctionsDetector;

    #[test]
    fn test_library_has_public_or_external_functions() {
        let context =
            load_contract("../tests/contract-playground/out/LibraryContract.sol/Math.json");

        let mut detector = LibraryHasPublicOrExternalFunctionsDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context).unwrap();
        assert!(found);

        println!("{:?}", detector.instances());

        // assert that the detector finds the correct number of instances
        assert_eq!(detector.instances().len(), 1);
    }
}
