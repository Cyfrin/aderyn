use super::*;
use super::{node::*, *};
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PragmaDirective {
    pub literals: Vec<String>,
    pub src: String,
    pub id: NodeID,
}

impl BaseNode for PragmaDirective {
    fn accept(&self, visitor: &mut impl AstBaseVisitor) -> Result<()> {
        visitor.visit_pragma_directive(self)?;
        visitor.end_visit_pragma_directive(self)
    }
}

#[derive(Debug, PartialEq)]
pub struct PragmaDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub pragma_directive: &'a PragmaDirective,
}
