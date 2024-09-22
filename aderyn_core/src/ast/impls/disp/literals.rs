use crate::ast::*;
use std::fmt::Display;

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let LiteralKind::String = self.kind {
            f.write_str("\"")?;
        }

        if let Some(value) = self.value.as_ref() {
            f.write_str(value.as_str())?;
        }
        f.write_str(self.hex_value.as_str())?;

        if let Some(subdenomination) = self.subdenomination.as_ref() {
            subdenomination.fmt(f)?;
        }

        if let LiteralKind::String = self.kind {
            f.write_str("\"")?;
        }

        Ok(())
    }
}
