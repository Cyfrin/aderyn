use aderyn_core::{
    context::loader::ContextLoader, framework::hardhat::load_hardhat, read_file_to_string,
    visitor::ast_visitor::Node,
};
use std::path::{Path, PathBuf};

pub fn with_project_root_at(root_path: &PathBuf) -> (String, ContextLoader) {
    let mut context_loader = ContextLoader::default();

    println!("Framework detected. Hardhat mode engaged.");
    println!("Hardhat root path: {:?}", root_path);
    let src_path = root_path.join("contracts").to_str().unwrap().to_string();
    let hardhat_output = load_hardhat(root_path).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading Hardhat build info");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
    for (key, contract_source) in hardhat_output.output.sources.iter() {
        if key.starts_with("contracts/") {
            let absolute_path_clone = contract_source.ast.absolute_path.clone();
            let mut ast = contract_source.ast.clone();
            match read_file_to_string(&root_path.join(Path::new(
                &contract_source.ast.absolute_path.as_ref().unwrap(),
            ))) {
                Ok(content) => {
                    ast.source = Some(content);
                }
                Err(err) => {
                    eprintln!(
                        "Error reading Solidity source file: {:?}",
                        absolute_path_clone.unwrap()
                    );
                    eprintln!("{:?}", err);
                }
            }

            ast.accept(&mut context_loader).unwrap_or_else(|err| {
                // Exit with a non-zero exit code
                eprintln!("Error loading Hardhat AST into ContextLoader");
                eprintln!("{:?}", err);
            });
        }
    }

    (src_path, context_loader)
}
