use crate::ast::*;

impl VariableDeclaration {
    /// Returns the mutability of the variable that was declared.
    ///
    /// This is a helper to check variable mutability across Solidity versions.
    pub fn mutability(&self) -> Option<&Mutability> {
        if let Some(mutability) = &self.mutability {
            Some(mutability)
        } else if self.constant {
            Some(&Mutability::Constant)
        } else if self.state_variable {
            Some(&Mutability::Mutable)
        } else {
            None
        }
    }
}
