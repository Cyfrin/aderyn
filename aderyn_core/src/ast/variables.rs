use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Immutable,
    Mutable,
    Constant,
}

impl Display for Mutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    NonPayable,
    Payable,
    View,
    Pure,
}

impl Display for StateMutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Internal,
    External,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Default,
    Memory,
    Calldata,
    Storage,
}

impl Display for StorageLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclaration {
    pub base_functions: Option<Vec<NodeID>>,
    pub constant: bool,
    pub documentation: Option<Documentation>,
    pub function_selector: Option<String>,
    pub indexed: Option<bool>,
    pub mutability: Option<Mutability>,
    pub name: String,
    pub name_location: Option<String>,
    pub overrides: Option<OverrideSpecifier>,
    pub scope: NodeID,
    pub state_variable: bool,
    pub storage_location: StorageLocation,
    pub type_descriptions: TypeDescriptions,
    pub type_name: Option<TypeName>,
    pub value: Option<Expression>,
    pub visibility: Visibility,
    pub src: String,
    pub id: NodeID,
}

impl Node for VariableDeclaration {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_variable_declaration(self)? {
            if self.type_name.is_some() {
                self.type_name.as_ref().unwrap().accept(visitor)?;
            }
            if self.overrides.is_some() {
                self.overrides.as_ref().unwrap().accept(visitor)?;
            }
            if self.value.is_some() {
                self.value.as_ref().unwrap().accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_variable_declaration(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(overrides) = &self.overrides {
            visitor.visit_immediate_children(self.id, vec![overrides.id])?;
        }
        if let Some(value) = &self.value {
            if let Some(value_id) = value.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![value_id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.type_name.as_ref().unwrap()))?;

        if self.storage_location != StorageLocation::Default {
            f.write_fmt(format_args!(" {}", self.storage_location))?;
        }

        if let Some(mutability) = self.mutability.as_ref() {
            if mutability != &Mutability::Mutable {
                f.write_fmt(format_args!(" {mutability}"))?;
            }
        }

        if let Some(true) = self.indexed {
            f.write_str(" indexed")?;
        }

        if self.state_variable {
            f.write_fmt(format_args!(" {}", self.visibility))?;
        }

        if !self.name.is_empty() {
            f.write_fmt(format_args!(" {}", self.name))?;
        }

        if let Some(value) = self.value.as_ref() {
            f.write_fmt(format_args!(" = {value}"))?;
        }

        Ok(())
    }
}
