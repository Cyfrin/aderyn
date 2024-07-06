pub mod contracts;
pub mod documentation;
pub mod enumerations;
pub mod errors;
pub mod events;
pub mod expressions;
pub mod functions;
pub mod identifiers;
pub mod impls;
pub mod import_directives;
pub mod literals;
pub mod macros;
pub mod magic;
pub mod modifiers;
pub mod node_type;
pub mod pragma_directives;
pub mod raw_nodes;
pub mod source_units;
pub mod statements;
pub mod structures;
pub mod types;
pub mod user_defined_value_types;
pub mod using_for_directives;
pub mod variables;
pub mod yul;

pub use self::{
    contracts::*, documentation::*, enumerations::*, errors::*, events::*, expressions::*,
    functions::*, identifiers::*, impls::*, import_directives::*, literals::*, magic::*,
    modifiers::*, node_type::*, pragma_directives::*, source_units::*, statements::*,
    structures::*, types::*, user_defined_value_types::*, using_for_directives::*, variables::*,
    yul::*,
};

pub use raw_nodes::*;
pub use NodeID;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn can_parse_ast() {
        fs::read_dir(PathBuf::from("../tests/ast"))
            .unwrap()
            .for_each(|path| {
                let path = path.unwrap().path();
                let path_str = path.to_string_lossy();

                let input = fs::read_to_string(&path).unwrap();
                let result: Result<SourceUnit, _> = serde_json::from_str(&input);
                match result {
                    Err(e) => {
                        println!("... {path_str} fail: {e}");
                        panic!();
                    }
                    Ok(_) => {
                        println!("... {path_str} ok");
                    }
                }
            })
    }
}
