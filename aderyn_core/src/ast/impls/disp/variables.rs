use crate::ast::*;
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
