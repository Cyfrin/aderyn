use crate::ast::*;

use std::fmt::Display;

impl Display for UsingForDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "using {:?} for {}",
            self.library_name,
            match self.type_name.as_ref() {
                Some(type_name) => format!("{type_name}"),
                None => "_".to_string(),
            }
        ))
    }
}
