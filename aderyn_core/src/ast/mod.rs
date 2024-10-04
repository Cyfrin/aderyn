#[allow(clippy::module_inception)]
pub mod ast;
pub mod ast_nodes;
pub mod impls;
pub mod macros;
pub mod magic;
pub mod node_type;
pub mod yul;

pub use self::{impls::*, magic::*, node_type::*, yul::*};

pub use ast::ASTNode;
pub use ast_nodes::*;
pub use NodeID;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn can_parse_ast() {
        fs::read_dir(PathBuf::from("../tests/ast")).unwrap().for_each(|path| {
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
