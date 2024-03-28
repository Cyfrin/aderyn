use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub name_location: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Node for EnumValue {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_enum_value(self)?;
        visitor.end_visit_enum_value(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub members: Vec<EnumValue>,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Node for EnumDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_enum_definition(self)? {
            list_accept(&self.members, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_enum_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let member_ids = &self
            .members
            .iter()
            .map(|x| x.id)
            .collect::<Vec<_>>()
            .clone();
        visitor.visit_immediate_children(self.id, member_ids.clone())?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for EnumDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("enum {} {{\n", self.name))?;

        for member in self.members.iter() {
            f.write_fmt(format_args!("\t{member},"))?;
        }

        f.write_str("}")
    }
}
