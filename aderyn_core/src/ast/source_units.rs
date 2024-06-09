use super::*;
use crate::visitor::ast_visitor::*;
use eyre::eyre;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};

#[derive(Clone, Debug, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum SourceUnitNode {
    FunctionDefinition(FunctionDefinition),
    StructDefinition(StructDefinition),
    ErrorDefinition(ErrorDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    ImportDirective(ImportDirective),
    PragmaDirective(PragmaDirective),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    ContractDefinition(ContractDefinition),
}

impl<'de> Deserialize<'de> for SourceUnitNode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType").unwrap().as_str().unwrap();
        match node_type {
            "FunctionDefinition" => Ok(SourceUnitNode::FunctionDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "ContractDefinition" => Ok(SourceUnitNode::ContractDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "StructDefinition" => Ok(SourceUnitNode::StructDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "ErrorDefinition" => Ok(SourceUnitNode::ErrorDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "EnumDefinition" => Ok(SourceUnitNode::EnumDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "VariableDeclaration" => Ok(SourceUnitNode::VariableDeclaration(
                serde_json::from_value(json).unwrap(),
            )),
            "ImportDirective" => Ok(SourceUnitNode::ImportDirective(
                serde_json::from_value(json).unwrap(),
            )),
            "PragmaDirective" => Ok(SourceUnitNode::PragmaDirective(
                serde_json::from_value(json).unwrap(),
            )),
            "UserDefinedValueTypeDefinition" => Ok(SourceUnitNode::UserDefinedValueTypeDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "UsingForDirective" => Ok(SourceUnitNode::UsingForDirective(
                serde_json::from_value(json).unwrap(),
            )),
            _ => panic!("Invalid source unit node type: {node_type}"),
        }
    }
}

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
        }
    }
}

impl Node for SourceUnitNode {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            SourceUnitNode::PragmaDirective(pragma_directive) => pragma_directive.accept(visitor),
            SourceUnitNode::ImportDirective(import_directive) => import_directive.accept(visitor),
            SourceUnitNode::ContractDefinition(contract_definition) => {
                contract_definition.accept(visitor)
            }
            SourceUnitNode::StructDefinition(struct_definition) => {
                struct_definition.accept(visitor)
            }
            SourceUnitNode::EnumDefinition(enum_definition) => enum_definition.accept(visitor),
            SourceUnitNode::ErrorDefinition(error_definition) => error_definition.accept(visitor),
            SourceUnitNode::VariableDeclaration(variable_declaration) => {
                variable_declaration.accept(visitor)
            }
            SourceUnitNode::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
                user_defined_value_type_definition.accept(visitor)
            }
            SourceUnitNode::FunctionDefinition(function_defn) => function_defn.accept(visitor),
            SourceUnitNode::UsingForDirective(using_for_directive) => {
                using_for_directive.accept(visitor)
            }
        }
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub license: Option<String>,
    pub nodes: Vec<SourceUnitNode>,
    pub exported_symbols: Option<HashMap<String, Vec<NodeID>>>,
    pub absolute_path: Option<String>,
    pub id: NodeID,

    #[serde(skip_serializing)]
    pub source: Option<String>,
}

impl Node for SourceUnit {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_source_unit(self)? {
            list_accept(&self.nodes, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_source_unit(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let node_ids = &self
            .nodes
            .iter()
            .flat_map(|x| x.get_node_id())
            .collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, node_ids.clone())?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
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

        let index = values
            .first()
            .and_then(|&value| value)
            .ok_or_else(|| eyre!("not found"))?;

        if index > source.len() {
            return Err(eyre!("index out of bounds"));
        }

        Ok(source[..index].chars().filter(|&c| c == '\n').count() + 1)
    }

    pub fn pragma_directives(&self) -> Vec<&PragmaDirective> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let SourceUnitNode::PragmaDirective(pragma_directive) = node {
                result.push(pragma_directive);
            }
        }

        result
    }

    pub fn import_directives(&self) -> Vec<&ImportDirective> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let SourceUnitNode::ImportDirective(import_directive) = node {
                result.push(import_directive);
            }
        }

        result
    }

    pub fn contract_definitions(&self) -> Vec<&ContractDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                result.push(contract_definition);
            }
        }

        result
    }

    pub fn contract_definition(&self, id: NodeID) -> Option<&ContractDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                if id == contract_definition.id {
                    return Some(contract_definition);
                }
            }
        }

        None
    }

    pub fn struct_definition(&self, id: NodeID) -> Option<&StructDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::StructDefinition(struct_definition) = node {
                if id == struct_definition.id {
                    return Some(struct_definition);
                }
            } else if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if let ContractDefinitionNode::StructDefinition(struct_definition) = node {
                        if id == struct_definition.id {
                            return Some(struct_definition);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn enum_definition(&self, id: NodeID) -> Option<&EnumDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::EnumDefinition(enum_definition) = node {
                if id == enum_definition.id {
                    return Some(enum_definition);
                }
            } else if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if let ContractDefinitionNode::EnumDefinition(enum_definition) = node {
                        if id == enum_definition.id {
                            return Some(enum_definition);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn user_defined_value_type_definition(
        &self,
        id: NodeID,
    ) -> Option<&UserDefinedValueTypeDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::UserDefinedValueTypeDefinition(
                user_defined_value_type_definition,
            ) = node
            {
                if id == user_defined_value_type_definition.id {
                    return Some(user_defined_value_type_definition);
                }
            } else if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if let ContractDefinitionNode::UserDefinedValueTypeDefinition(
                        user_defined_value_type_definition,
                    ) = node
                    {
                        if id == user_defined_value_type_definition.id {
                            return Some(user_defined_value_type_definition);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn function_definition(&self, id: NodeID) -> Option<&FunctionDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                        if function_definition.id == id {
                            return Some(function_definition);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn function_and_contract_definition(
        &self,
        id: NodeID,
    ) -> Option<(&ContractDefinition, &FunctionDefinition)> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                        if function_definition.id == id {
                            return Some((contract_definition, function_definition));
                        }
                    }
                }
            }
        }

        None
    }

    pub fn find_contract_definition_node(
        &self,
        id: NodeID,
    ) -> Option<(&ContractDefinition, &ContractDefinitionNode)> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if id
                        == match node {
                            ContractDefinitionNode::UsingForDirective(node) => node.id,
                            ContractDefinitionNode::StructDefinition(node) => node.id,
                            ContractDefinitionNode::EnumDefinition(node) => node.id,
                            ContractDefinitionNode::VariableDeclaration(node) => node.id,
                            ContractDefinitionNode::EventDefinition(node) => node.id,
                            ContractDefinitionNode::FunctionDefinition(node) => node.id,
                            ContractDefinitionNode::ModifierDefinition(node) => node.id,
                            ContractDefinitionNode::ErrorDefinition(node) => node.id,
                            ContractDefinitionNode::UserDefinedValueTypeDefinition(node) => node.id,
                        }
                    {
                        return Some((contract_definition, node));
                    }
                }
            }
        }

        None
    }
}
