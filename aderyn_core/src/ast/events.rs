use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub anonymous: bool,
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub src: String,
    pub id: NodeID,
}

impl Node for EventDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_event_definition(self)? {
            if self.documentation.is_some() {
                self.documentation.as_ref().unwrap().accept(visitor)?;
            }
            self.parameters.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_event_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        // TODO: documentation nodes
        visitor.visit_immediate_children(self.id, vec![self.parameters.id])?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for EventDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("event {}{}", self.name, self.parameters))
    }
}
