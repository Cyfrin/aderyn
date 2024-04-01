use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub visibility: Visibility,
    pub members: Vec<VariableDeclaration>,
    pub scope: NodeID,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Node for StructDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_struct_definition(self)? {
            list_accept(&self.members, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_struct_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let members_ids = &self.members.iter().map(|x| x.id).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, members_ids.clone())?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for StructDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("struct {} {{\n", self.name))?;

        for member in self.members.iter() {
            f.write_fmt(format_args!("\t{member};\n"))?;
        }

        f.write_str("}")
    }
}
