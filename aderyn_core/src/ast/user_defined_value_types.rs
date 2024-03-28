use super::*;
use crate::visitor::ast_visitor::ASTConstVisitor;
use crate::visitor::ast_visitor::Node;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub underlying_type: TypeName,
    pub name: String,
    pub name_location: Option<String>,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Display for UserDefinedValueTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "type {} is {}",
            self.name, self.underlying_type,
        ))
    }
}

impl Node for UserDefinedValueTypeDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_user_defined_value_type_definition(self)? {
            self.underlying_type.accept(visitor)?;
        }
        visitor.end_visit_user_defined_value_type_definition(self)?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
