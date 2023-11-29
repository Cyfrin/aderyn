use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TypeName {
    FunctionTypeName(FunctionTypeName),
    ArrayTypeName(ArrayTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
    ElementaryTypeName(ElementaryTypeName),
    String(String),
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::FunctionTypeName(function_type_name) => function_type_name
                .type_descriptions
                .type_string
                .as_ref()
                .unwrap()
                .fmt(f),
            TypeName::ArrayTypeName(array_type_name) => array_type_name.fmt(f),
            TypeName::Mapping(mapping) => mapping.fmt(f),
            TypeName::UserDefinedTypeName(user_defined_type_name) => user_defined_type_name.fmt(f),
            TypeName::ElementaryTypeName(elementary_type_name) => elementary_type_name.fmt(f),
            TypeName::String(string) => string.fmt(f),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub state_mutability: Option<StateMutability>,
    pub name: String,
    pub type_descriptions: TypeDescriptions,
}

impl PartialEq for ElementaryTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.state_mutability.eq(&other.state_mutability)
            && self.type_descriptions.eq(&other.type_descriptions)
    }
}

impl Display for ElementaryTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())?;

        if let Some(state_mutability) = self.state_mutability {
            if state_mutability != StateMutability::NonPayable {
                f.write_fmt(format_args!(" {}", state_mutability))?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedTypeName {
    pub path_node: Option<IdentifierPath>,
    pub referenced_declaration: NodeID,
    pub name: Option<String>,
    pub type_descriptions: TypeDescriptions,
}

impl PartialEq for UserDefinedTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.referenced_declaration
            .eq(&other.referenced_declaration)
    }
}

impl Display for UserDefinedTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_node) = self.path_node.as_ref() {
            f.write_fmt(format_args!("{}", path_node))
        } else {
            f.write_fmt(format_args!("{}", self.name.as_deref().unwrap_or("")))
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTypeName {
    pub visibility: Visibility,
    pub state_mutability: StateMutability,
    pub parameter_types: ParameterList,
    pub return_parameter_types: ParameterList,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeName {
    pub base_type: Box<TypeName>,
    pub length: Option<Literal>,
    pub type_descriptions: TypeDescriptions,
}

impl Display for ArrayTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.base_type))?;
        f.write_str("[")?;

        if let Some(length) = self.length.as_ref() {
            f.write_fmt(format_args!("{}", length))?;
        }

        f.write_str("]")
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub key_type: Box<TypeName>,
    pub value_type: Box<TypeName>,
    pub type_descriptions: TypeDescriptions,
}

impl Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "mapping({} => {})",
            self.key_type, self.value_type
        ))
    }
}
