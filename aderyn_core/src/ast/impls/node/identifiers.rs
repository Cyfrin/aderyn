use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for Identifier {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_identifier(self)?;
        self.accept_metadata(visitor)?;
        visitor.end_visit_identifier(self)
    }
    fn accept_metadata(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    macros::accept_id!();
}

impl Node for IdentifierPath {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_identifier_path(self)?;
        self.accept_metadata(visitor)?;
        visitor.end_visit_identifier_path(self)
    }
    fn accept_metadata(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    macros::accept_id!();
}
