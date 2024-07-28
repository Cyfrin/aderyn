use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{NodeID, StorageLocation};

use crate::capture;
use crate::context::workspace_context::ASTNode;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct StorageArrayEditWithMemoryDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StorageArrayEditWithMemoryDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // get all Identifiers with argumentTypes
        // If any of them are of the type storage,
        //  grab the index of that param in the array of argument types
        //  get the refereddeclaration node of the identifier (a function)
        //  get parameter at that index and check if the storageLocation is not storage
        //  if not, capture it.

        for identifier in context
            .identifiers()
            .into_iter()
            .filter(|identifier| identifier.argument_types.is_some())
        {
            for (index, argument_type) in identifier
                .argument_types
                .as_ref()
                .unwrap()
                .iter()
                .enumerate()
            {
                if let Some(type_string) = &argument_type.type_string {
                    if type_string.contains("storage ref") {
                        let definition_ast = context
                            .nodes
                            .get(&identifier.referenced_declaration.unwrap());
                        if let Some(ASTNode::FunctionDefinition(definition)) = definition_ast {
                            let parameter = definition
                                .parameters
                                .parameters
                                .get(index)
                                .ok_or_else(|| eyre::eyre!("Parameter not found"))?;
                            if parameter.storage_location != StorageLocation::Storage {
                                capture!(self, context, identifier);
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Storage Array Edited with Memory")
    }

    fn description(&self) -> String {
        String::from("Storage reference is passed to a function with a memory parameter. This will not update the storage variable as expected. Consider using storage parameters instead.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::StorageArrayEditWithMemory.to_string()
    }
}

#[cfg(test)]
mod storage_array_edit_with_memory_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        high::storage_array_edit_with_memory::StorageArrayEditWithMemoryDetector,
    };

    #[test]
    #[serial]
    fn test_storage_array_edit_with_memory() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StorageParameters.sol",
        );

        let mut detector = StorageArrayEditWithMemoryDetector::default();
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
            String::from("Storage Array Edited with Memory")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Storage reference is passed to a function with a memory parameter. This will not update the storage variable as expected. Consider using storage parameters instead.")
        );
    }
}
