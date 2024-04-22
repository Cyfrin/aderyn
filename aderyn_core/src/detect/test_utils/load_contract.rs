use crate::context::workspace_context::WorkspaceContext;
use crate::framework::foundry::read_foundry_output_file;
use crate::read_file_to_string;
use crate::visitor::ast_visitor::Node;
use std::path::PathBuf;

pub fn load_contract(filepath: &str) -> WorkspaceContext {
    let path_buf_filepath = std::path::PathBuf::from(filepath);
    let mut context = WorkspaceContext::default();
    let foundry_output = read_foundry_output_file(path_buf_filepath.to_str().unwrap()).unwrap();
    let mut ast = foundry_output.ast.clone();
    // Get the path of the source file
    let mut new_path = PathBuf::new();
    for component in path_buf_filepath.components() {
        if component.as_os_str() == "out" {
            break;
        }
        new_path.push(component);
    }
    new_path.push(ast.absolute_path.as_ref().unwrap());
    match read_file_to_string(&new_path) {
        Ok(content) => {
            println!(
                "Loaded Solidity source file: {}",
                new_path.to_str().unwrap()
            );

            ast.source = Some(content);
        }
        Err(err) => {
            eprintln!(
                "Error reading Solidity source file: {}",
                new_path.to_str().unwrap()
            );
            eprintln!("{:?}", err);
        }
    }
    ast.accept(&mut context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading Hardhat AST into WorkspaceContext");
        eprintln!("{:?}", err);
    });
    context
}
