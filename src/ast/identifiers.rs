use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub name: String,
    pub overloaded_declarations: Vec<NodeID>,
    pub referenced_declaration: NodeID,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Node for Identifier {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_identifier(self)?;
        visitor.end_visit_identifier(self)
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.argument_types.eq(&other.argument_types)
            && self.name.eq(&other.name)
            && self
                .overloaded_declarations
                .eq(&other.overloaded_declarations)
            && self
                .referenced_declaration
                .eq(&other.referenced_declaration)
            && self.type_descriptions.eq(&other.type_descriptions)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub name: String,
    pub referenced_declaration: Option<NodeID>,
    pub src: String,
    pub id: NodeID,
}

impl Node for IdentifierPath {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_identifier_path(self)?;
        visitor.end_visit_identifier_path(self)
    }
}

impl PartialEq for IdentifierPath {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
            && self
                .referenced_declaration
                .eq(&other.referenced_declaration)
    }
}

impl Display for IdentifierPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}
