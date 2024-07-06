use crate::ast::*;

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
