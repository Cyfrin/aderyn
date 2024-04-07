use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum LiteralKind {
    Bool,
    Number,
    String,
    HexString,
    Address,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub hex_value: Option<String>,
    pub value: Option<String>,
    pub subdenomination: Option<String>,
    pub kind: LiteralKind,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Node for Literal {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_literal(self)?;
        visitor.end_visit_literal(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let LiteralKind::String = self.kind {
            f.write_str("\"")?;
        }

        if let Some(value) = self.value.as_ref() {
            f.write_str(value.as_str())?;
        } else if let Some(hex_value) = self.hex_value.as_ref() {
            f.write_str(hex_value.as_str())?;
        }

        if let Some(subdenomination) = self.subdenomination.as_ref() {
            subdenomination.fmt(f)?;
        }

        if let LiteralKind::String = self.kind {
            f.write_str("\"")?;
        }

        Ok(())
    }
}
