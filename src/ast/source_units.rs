use super::*;
use super::{node::*, *};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};

#[derive(Clone, Debug, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum SourceUnitNode {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractDefinition(ContractDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    VariableDeclaration(VariableDeclaration),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
}

impl BaseNode for SourceUnitNode {
    fn accept(&self, visitor: &mut impl AstBaseVisitor) -> Result<()> {
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
        }
    }
}

impl<'de> Deserialize<'de> for SourceUnitNode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType").unwrap().as_str().unwrap();

        match node_type {
            "PragmaDirective" => Ok(SourceUnitNode::PragmaDirective(
                serde_json::from_value(json).unwrap(),
            )),
            "ImportDirective" => Ok(SourceUnitNode::ImportDirective(
                serde_json::from_value(json).unwrap(),
            )),
            "ContractDefinition" => Ok(SourceUnitNode::ContractDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "StructDefinition" => Ok(SourceUnitNode::StructDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "EnumDefinition" => Ok(SourceUnitNode::EnumDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "ErrorDefinition" => Ok(SourceUnitNode::ErrorDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "VariableDeclaration" => Ok(SourceUnitNode::VariableDeclaration(
                serde_json::from_value(json).unwrap(),
            )),
            "UserDefinedValueTypeDefinition" => Ok(SourceUnitNode::UserDefinedValueTypeDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            _ => panic!("Invalid source unit node type: {node_type}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub license: Option<String>,
    pub nodes: Vec<SourceUnitNode>,
    pub exported_symbols: Option<HashMap<String, Vec<NodeID>>>,
    pub experimental_solidity: Option<bool>,
    pub absolute_path: Option<String>,
    pub id: NodeID,

    #[serde(skip_serializing)]
    pub source: Option<String>,
}

impl BaseNode for SourceUnit {
    fn accept(&self, visitor: &mut impl AstBaseVisitor) -> Result<()> {
        if visitor.visit_source_unit(self)? {
            list_accept(&self.nodes, visitor)?;
        }
        visitor.end_visit_source_unit(self)
    }
}

impl SourceUnit {
    pub fn source_line(&self, src: &str) -> io::Result<usize> {
        let source = match self.source.as_ref() {
            Some(source) => source.as_str(),
            _ => return Err(io::Error::from(io::ErrorKind::NotFound)),
        };

        let mut values: Vec<Option<usize>> = vec![];

        for token in src.split(':') {
            values.push(if token.is_empty() {
                None
            } else {
                Some(
                    token
                        .parse()
                        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?,
                )
            });
        }

        Ok(source[..match values.first() {
            Some(&Some(value)) => value,
            _ => return Err(io::Error::from(io::ErrorKind::NotFound)),
        }]
            .chars()
            .filter(|&c| c == '\n')
            .count()
            + 1)
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

#[derive(Debug, PartialEq)]
pub struct SourceUnitContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
}

impl<'a> SourceUnitContext<'a> {
    pub fn create_pragma_directive_context(
        &self,
        pragma_directive: &'a PragmaDirective,
    ) -> PragmaDirectiveContext<'a> {
        PragmaDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            pragma_directive,
        }
    }

    pub fn create_import_directive_context(
        &self,
        import_directive: &'a ImportDirective,
    ) -> ImportDirectiveContext<'a> {
        ImportDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            import_directive,
        }
    }

    pub fn create_contract_definition_context(
        &self,
        contract_definition: &'a ContractDefinition,
    ) -> ContractDefinitionContext<'a> {
        ContractDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition,
        }
    }

    pub fn create_struct_definition_context(
        &self,
        struct_definition: &'a StructDefinition,
    ) -> StructDefinitionContext<'a> {
        StructDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            struct_definition,
        }
    }

    pub fn create_enum_definition_context(
        &self,
        enum_definition: &'a EnumDefinition,
    ) -> EnumDefinitionContext<'a> {
        EnumDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            enum_definition,
        }
    }

    pub fn create_error_definition_context(
        &self,
        error_definition: &'a ErrorDefinition,
    ) -> ErrorDefinitionContext<'a> {
        ErrorDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            error_definition,
        }
    }

    pub fn create_variable_declaration_context<'b>(
        &self,
        variable_declaration: &'a VariableDeclaration,
    ) -> VariableDeclarationContext<'a, 'b> {
        VariableDeclarationContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            definition_node: None,
            blocks: None,
            variable_declaration,
        }
    }

    pub fn create_user_defined_value_type_definition_context(
        &self,
        user_defined_value_type_definition: &'a UserDefinedValueTypeDefinition,
    ) -> UserDefinedValueTypeDefinitionContext<'a> {
        UserDefinedValueTypeDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            user_defined_value_type_definition,
        }
    }
}
