use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
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
        self.accept_metadata(visitor)?;
        visitor.end_visit_identifier(self)
    }
    fn accept_metadata(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
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

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.argument_types.hash(state);
        self.name.hash(state);
        self.overloaded_declarations.hash(state);
        self.referenced_declaration.hash(state);
        self.type_descriptions.hash(state);
        self.src.hash(state);
        self.id.hash(state);
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
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
        self.accept_metadata(visitor)?;
        visitor.end_visit_identifier_path(self)
    }
    fn accept_metadata(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
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

impl Hash for IdentifierPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.referenced_declaration.hash(state);
        self.src.hash(state);
        self.id.hash(state);
    }
}

impl Display for IdentifierPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}
