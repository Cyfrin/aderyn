use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for Literal {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_literal(self)?;
        visitor.end_visit_literal(self)
    }
    macros::accept_id!();
}
