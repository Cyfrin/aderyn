use crate::ast::{Block, UncheckedBlock};
use std::fmt::Display;

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{\n")?;

        for statement in self.statements.iter() {
            f.write_fmt(format_args!("\t{statement};\n"))?;
        }

        f.write_str("}")
    }
}

impl Display for UncheckedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{\n")?;

        for statement in self.statements.iter() {
            f.write_fmt(format_args!("\t{statement};\n"))?;
        }

        f.write_str("}")
    }
}
