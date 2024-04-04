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
            self.accept_metadata(visitor)?;
        }
        visitor.end_visit_import_directive(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let mut symbol_alias_ids = vec![];
        for symbol_alias in &self.symbol_aliases {
            if let Some(expr_id) = symbol_alias.foreign.get_node_id() {
                symbol_alias_ids.push(expr_id);
            }
        }
        visitor.visit_immediate_children(self.id, symbol_alias_ids)?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
