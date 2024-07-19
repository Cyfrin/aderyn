use crate::ast::*;
use eyre::eyre;
use eyre::Result;
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
