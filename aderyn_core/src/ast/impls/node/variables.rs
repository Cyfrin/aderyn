use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use std::fmt::Display;

impl Display for Mutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

impl Display for StateMutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

impl Display for StorageLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

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

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.type_name.as_ref().unwrap()))?;

        if self.storage_location != StorageLocation::Default {
            f.write_fmt(format_args!(" {}", self.storage_location))?;
        }

        if let Some(mutability) = self.mutability.as_ref() {
            if mutability != &Mutability::Mutable {
                f.write_fmt(format_args!(" {mutability}"))?;
            }
        }

        if let Some(true) = self.indexed {
            f.write_str(" indexed")?;
        }

        if self.state_variable {
            f.write_fmt(format_args!(" {}", self.visibility))?;
        }

        if !self.name.is_empty() {
            f.write_fmt(format_args!(" {}", self.name))?;
        }

        if let Some(value) = self.value.as_ref() {
            f.write_fmt(format_args!(" = {value}"))?;
        }

        Ok(())
    }
}
