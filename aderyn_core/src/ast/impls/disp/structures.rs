use crate::ast::*;
use std::fmt::Display;

impl Display for StructDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("struct {} {{\n", self.name))?;

        for member in self.members.iter() {
            f.write_fmt(format_args!("\t{member};\n"))?;
        }

        f.write_str("}")
    }
}
