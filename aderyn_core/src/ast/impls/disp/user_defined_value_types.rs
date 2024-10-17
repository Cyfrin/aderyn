use crate::ast::*;
use std::fmt::Display;

impl Display for UserDefinedValueTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("type {} is {}", self.name, self.underlying_type,))
    }
}
