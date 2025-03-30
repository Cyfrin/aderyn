use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for PragmaDirective {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_pragma_directive(self)?;
        visitor.end_visit_pragma_directive(self)
    }
    macros::accept_id!();
}
