use std::{collections::BTreeMap, error::Error};

use crate::ast::{NodeID, NodeType, TypeName};

use crate::{
    capture,
    context::{
        browser::ExtractPragmaDirectives,
        workspace::{ASTNode, WorkspaceContext},
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::pragma_directive_to_semver,
    },
};
use eyre::Result;
use semver::VersionReq;

#[derive(Default)]
pub struct NestedStructInMappingDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

fn version_req_allows_below_0_5_0(version_req: &VersionReq) -> bool {
    if version_req.comparators.is_empty() {
        return false; // Return false or handle as needed if there are no comparators
    }

    let comparator = &version_req.comparators[0];
    comparator.major == 0 && comparator.minor.is_some_and(|m| m < 5)
}

impl IssueDetector for NestedStructInMappingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);

        let mappings = context.variable_declarations().into_iter().filter(|vd| {
            if let Some(type_name) = &vd.type_name {
                if let TypeName::Mapping(_) = type_name {
                    return true;
                }
                return false;
            }
            false
        });

        for mapping in mappings {
            if let Some(TypeName::Mapping(mapping_type)) = &mapping.type_name {
                if let TypeName::UserDefinedTypeName(user_defined_type) = &*mapping_type.value_type
                {
                    let struct_definition_ast_node =
                        context.nodes.get(&user_defined_type.referenced_declaration);
                    if let Some(ASTNode::StructDefinition(struct_definition)) =
                        struct_definition_ast_node
                    {
                        for member in struct_definition.members.iter() {
                            if let Some(member_type_string) = &member.type_descriptions.type_string
                            {
                                if member_type_string.contains("struct") {
                                    // Check if the contract that this is in allows for solidity
                                    // pragma below 0.5.0
                                    let source_unit_ast_node = context
                                        .get_closest_ancestor(mapping.id, NodeType::SourceUnit);
                                    if let Some(source_unit_ast_node) = source_unit_ast_node {
                                        let pragma_directives =
                                            ExtractPragmaDirectives::from(source_unit_ast_node)
                                                .extracted;
                                        let version_req = pragma_directive_to_semver(
                                            pragma_directives.first().unwrap(),
                                        )?;
                                        if version_req_allows_below_0_5_0(&version_req) {
                                            capture!(self, context, mapping);
                                        }
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
        String::from("Nested Structs in Mappings pre-0.5.0")
    }

    fn description(&self) -> String {
        String::from("Prior to updates in Solidity 0.5.0, public mappings with nested structs compiled, but produced incorrect values. Refrain from using these, or update to a more recent version of Solidity.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::NestedStructInMapping.to_string()
    }
}

#[cfg(test)]
mod nested_struct_in_mapping_detector_tests {

    use crate::detect::{detector::IssueDetector, high::NestedStructInMappingDetector};

    #[test]

    fn test_nested_struct_in_mapping_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/nested_mappings/NestedMappings.sol",
        );

        let mut detector = NestedStructInMappingDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]

    fn test_nested_struct_in_mapping_detector_no_issue() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/nested_mappings/LaterVersion.sol",
        );

        let mut detector = NestedStructInMappingDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
        assert_eq!(detector.instances().len(), 0);
    }
}
