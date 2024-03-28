use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub src: String,
    pub id: NodeID,
}

impl Node for ErrorDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_error_definition(self)? && self.documentation.is_some() {
            self.documentation.as_ref().unwrap().accept(visitor)?;
            self.parameters.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_error_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        // TODO: send documentation nodes to visitor
        visitor.visit_immediate_children(self.id, vec![self.parameters.id])
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for ErrorDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("error {}{}", self.name, self.parameters))
    }
}
