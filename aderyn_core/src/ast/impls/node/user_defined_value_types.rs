use crate::{
    ast::*,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

impl Node for UserDefinedValueTypeDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_user_defined_value_type_definition(self)? {
            self.underlying_type.accept(visitor)?;
        }
        visitor.end_visit_user_defined_value_type_definition(self)?;
        Ok(())
    }
    macros::accept_id!();
}
