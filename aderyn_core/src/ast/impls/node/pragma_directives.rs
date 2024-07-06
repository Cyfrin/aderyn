use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;

impl Node for PragmaDirective {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_pragma_directive(self)?;
        visitor.end_visit_pragma_directive(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
