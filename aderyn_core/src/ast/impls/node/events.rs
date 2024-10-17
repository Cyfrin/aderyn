use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for EventDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_event_definition(self)? {
            if self.documentation.is_some() {
                self.documentation.as_ref().unwrap().accept(visitor)?;
            }
            self.parameters.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_event_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        // TODO: documentation nodes
        visitor.visit_immediate_children(self.id, vec![self.parameters.id])?;
        Ok(())
    }
    macros::accept_id!();
}
