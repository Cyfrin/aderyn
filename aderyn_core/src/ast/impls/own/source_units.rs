use crate::ast::{ImportDirective, NodeID, SourceUnit, SourceUnitNode};
use eyre::{eyre, Result};
use std::io;

impl SourceUnitNode {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            SourceUnitNode::PragmaDirective(pragma_directive) => Some(pragma_directive.id),
            SourceUnitNode::ImportDirective(import_directive) => Some(import_directive.id),
            SourceUnitNode::ContractDefinition(contract_definition) => Some(contract_definition.id),
            SourceUnitNode::StructDefinition(struct_definition) => Some(struct_definition.id),
            SourceUnitNode::EnumDefinition(enum_definition) => Some(enum_definition.id),
            SourceUnitNode::ErrorDefinition(error_definition) => Some(error_definition.id),
            SourceUnitNode::VariableDeclaration(variable_declaration) => {
                Some(variable_declaration.id)
            }
            SourceUnitNode::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
                Some(user_defined_value_type_definition.id)
            }
            SourceUnitNode::FunctionDefinition(function_defn) => Some(function_defn.id),
            SourceUnitNode::UsingForDirective(using_for_directive) => Some(using_for_directive.id),
            SourceUnitNode::EventDefinition(event_definition) => Some(event_definition.id),
        }
    }
}

impl SourceUnit {
    pub fn source_line(&self, src: &str) -> Result<usize> {
        let source = match self.source.as_ref() {
            Some(source) => source.as_str(),
            _ => return Err(eyre!("not found")),
        };

        let values: Vec<Option<usize>> = src
            .split(':')
            .map(|token| {
                if token.is_empty() {
                    None
                } else {
                    token
                        .parse()
                        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
                        .ok()
                }
            })
            .collect();

        let index = values.first().and_then(|&value| value).ok_or_else(|| eyre!("not found"))?;

        if index > source.len() {
            return Err(eyre!("index out of bounds"));
        }

        Ok(source[..index].chars().filter(|&c| c == '\n').count() + 1)
    }

    pub fn import_directives(&self) -> Vec<&ImportDirective> {
        self.nodes
            .iter()
            .filter_map(|n| {
                let SourceUnitNode::ImportDirective(node) = n else {
                    return None;
                };
                Some(node)
            })
            .collect()
    }
}
