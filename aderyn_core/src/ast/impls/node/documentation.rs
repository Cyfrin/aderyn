use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for Documentation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            Documentation::String(_opt_string) => {
                // TODO check if this is okay
                Ok(())
            }
            Documentation::Structured(opt_structured_documentation) => {
                if opt_structured_documentation.is_some() {
                    opt_structured_documentation.as_ref().unwrap().accept(visitor)?;
                }
                Ok(())
            }
        }
    }
}

impl Node for StructuredDocumentation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_structured_documentation(self)?;
        visitor.end_visit_structured_documentation(self)
    }
}
