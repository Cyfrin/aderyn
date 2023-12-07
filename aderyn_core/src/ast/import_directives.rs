use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Expression,
    pub local: Option<String>,
    pub name_location: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ImportDirective {
    pub file: String,
    pub source_unit: NodeID,
    pub scope: NodeID,
    pub absolute_path: Option<String>,
    pub unit_alias: String,
    pub name_location: Option<String>,
    pub symbol_aliases: Vec<SymbolAlias>,
    pub src: String,
    pub id: NodeID,
}

impl Node for ImportDirective {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_import_directive(self)? {
            // TODO deviation from solc's structs
            for symbol_alias in &self.symbol_aliases {
                symbol_alias.foreign.accept(visitor)?;
            }
        }
        visitor.end_visit_import_directive(self)
    }
}
