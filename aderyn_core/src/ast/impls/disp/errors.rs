use crate::ast::*;
use std::fmt::Display;

impl Display for ErrorDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("error {}{}", self.name, self.parameters))
    }
}
