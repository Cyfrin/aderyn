use crate::ast::*;
use std::fmt::Display;

impl Display for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

impl Display for EnumDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("enum {} {{\n", self.name))?;

        for member in self.members.iter() {
            f.write_fmt(format_args!("\t{member},"))?;
        }

        f.write_str("}")
    }
}
