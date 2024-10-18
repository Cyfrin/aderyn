use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

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
            symbol_alias_ids.push(symbol_alias.foreign.id);
        }
        visitor.visit_immediate_children(self.id, symbol_alias_ids)?;
        Ok(())
    }
    macros::accept_id!();
}
