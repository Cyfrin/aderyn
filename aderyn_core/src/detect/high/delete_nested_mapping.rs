use std::{collections::BTreeMap, error::Error};

use crate::ast::{
    ASTNode, Expression, Identifier, IndexAccess, Mapping, NodeID, TypeName, UserDefinedTypeName,
    VariableDeclaration,
};

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DeletionNestedMappingDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DeletionNestedMappingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for delete_operation in
            context.unary_operations().into_iter().filter(|op| op.operator == "delete")
        {
            if let Expression::IndexAccess(IndexAccess { base_expression, .. }) =
                delete_operation.sub_expression.as_ref()
            {
                if let Expression::Identifier(Identifier {
                    referenced_declaration: Some(referenced_id),
                    type_descriptions,
                    ..
                }) = base_expression.as_ref()
                {
                    // Check if we're deleting a value from mapping
                    if type_descriptions
                        .type_string
                        .as_ref()
                        .is_some_and(|type_string| type_string.starts_with("mapping"))
                    {
                        // Check if the value in the mapping is of type struct that has a member
                        // which is also a mapping
                        if let Some(ASTNode::VariableDeclaration(VariableDeclaration {
                            type_name: Some(TypeName::Mapping(Mapping { value_type, .. })),
                            ..
                        })) = context.nodes.get(referenced_id)
                        {
                            if let TypeName::UserDefinedTypeName(UserDefinedTypeName {
                                referenced_declaration,
                                ..
                            }) = value_type.as_ref()
                            {
                                if let Some(ASTNode::StructDefinition(structure)) =
                                    context.nodes.get(referenced_declaration)
                                {
                                    // Check that a member of a struct is of type mapping
                                    if structure.members.iter().any(|member| {
                                        member.type_descriptions.type_string.as_ref().is_some_and(
                                            |type_string| type_string.starts_with("mapping"),
                                        )
                                    }) {
                                        capture!(self, context, delete_operation);
                                    }
                                }
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
        String::from("Deletion from a nested mapping")
    }

    fn description(&self) -> String {
        String::from("A deletion in a structure containing a mapping will not delete the mapping. The remaining data may be used to compromise the contract.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::DeleteNestedMapping.to_string()
    }
}

#[cfg(test)]
mod deletion_nested_mapping_tests {

    use crate::detect::{
        detector::IssueDetector, high::delete_nested_mapping::DeletionNestedMappingDetector,
    };

    #[test]

    fn test_deletion_nested_mapping() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DeletionNestedMappingStructureContract.sol",
        );

        let mut detector = DeletionNestedMappingDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
