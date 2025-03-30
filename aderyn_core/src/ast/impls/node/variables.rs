use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for VariableDeclaration {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_variable_declaration(self)? {
            if self.type_name.is_some() {
                self.type_name.as_ref().unwrap().accept(visitor)?;
            }
            if self.overrides.is_some() {
                self.overrides.as_ref().unwrap().accept(visitor)?;
            }
            if self.value.is_some() {
                self.value.as_ref().unwrap().accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_variable_declaration(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(overrides) = &self.overrides {
            visitor.visit_immediate_children(self.id, vec![overrides.id])?;
        }
        if let Some(value) = &self.value {
            if let Some(value_id) = value.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![value_id])?;
            }
        }
        Ok(())
    }
    macros::accept_id!();
}
