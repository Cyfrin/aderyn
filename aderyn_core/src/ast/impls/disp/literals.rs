use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;
use std::fmt::Display;

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let LiteralKind::String = self.kind {
            f.write_str("\"")?;
        }

        if let Some(value) = self.value.as_ref() {
            f.write_str(value.as_str())?;
        } else if let Some(hex_value) = self.hex_value.as_ref() {
            f.write_str(hex_value.as_str())?;
        }

        if let Some(subdenomination) = self.subdenomination.as_ref() {
            subdenomination.fmt(f)?;
        }

        if let LiteralKind::String = self.kind {
            f.write_str("\"")?;
        }

        Ok(())
    }
}
