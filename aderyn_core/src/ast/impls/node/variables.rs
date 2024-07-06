use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;

impl VariableDeclaration {
    /// Returns the mutability of the variable that was declared.
    ///
    /// This is a helper to check variable mutability across Solidity versions.
    pub fn mutability(&self) -> &Mutability {
        if let Some(mutability) = &self.mutability {
            mutability
        } else if self.constant {
            &Mutability::Constant
        } else if self.state_variable {
            &Mutability::Mutable
        } else {
            unreachable!()
        }
    }
}

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
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
