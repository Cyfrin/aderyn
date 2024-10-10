use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for ErrorDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_error_definition(self)? && self.documentation.is_some() {
            self.documentation.as_ref().unwrap().accept(visitor)?;
            self.parameters.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_error_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        // TODO: send documentation nodes to visitor
        visitor.visit_immediate_children(self.id, vec![self.parameters.id])
    }
    macros::accept_id!();
}
