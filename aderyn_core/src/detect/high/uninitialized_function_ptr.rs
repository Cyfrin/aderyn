use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::{ASTNode, Expression, Identifier, NodeID, NodeType, VariableDeclarationStatement};

use crate::capture;
use crate::context::browser::GetClosestAncestorOfTypeX;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UninitializedFunctionPointerDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UninitializedFunctionPointerDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let function_ptrs =
            context
                .variable_declarations()
                .into_iter()
                .filter(|variable_declaration| {
                    variable_declaration
                        .type_descriptions
                        .type_string
                        .as_ref()
                        .is_some_and(|type_string| type_string.starts_with("function"))
                });

        let ids_of_assigned_identifiers = context
            .assignments()
            .into_iter()
            .flat_map(|assignment| {
                if let Expression::Identifier(Identifier {
                    referenced_declaration: Some(id),
                    ..
                }) = assignment.left_hand_side.as_ref()
                {
                    return Some(*id);
                }
                None
            })
            .collect::<HashSet<NodeID>>();

        for function_ptr in function_ptrs {
            let mut has_initial_value = false;
            let mut has_been_assigned_value = false;

            // Check if it's initialized with a value already
            if let Some(ASTNode::VariableDeclarationStatement(VariableDeclarationStatement {
                initial_value: Some(_),
                ..
            })) = function_ptr
                .closest_ancestor_of_type(context, NodeType::VariableDeclarationStatement)
            {
                has_initial_value = true;
            }

            // Check if it's been assigned a value
            if ids_of_assigned_identifiers.contains(&function_ptr.id) {
                has_been_assigned_value = true;
            }

            // If neither of them is true, then it is likely an uninitialized function pointer
            if !has_been_assigned_value && !has_initial_value {
                capture!(self, context, function_ptr);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Uninitialized function pointer.")
    }

    fn description(&self) -> String {
        String::from("Initialize the function pointer")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::UninitializedFunctionPointer.to_string()
    }
}

#[cfg(test)]
mod uninitialized_function_ptr_tests {
    use crate::detect::{
        detector::IssueDetector,
        high::uninitialized_function_ptr::UninitializedFunctionPointerDetector,
    };

    #[test]
    fn test_uninitialized_func_ptr() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UninitializedFunctionPointer.sol",
        );

        let mut detector = UninitializedFunctionPointerDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:#?}", detector.instances());

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
            String::from("Uninitialized function pointer.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(String::from("Initialize the function pointer"))
        );
    }
}
