use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

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
