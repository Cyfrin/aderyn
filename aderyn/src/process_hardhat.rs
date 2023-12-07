use aderyn_core::{
    context::loader::ContextLoader, framework::hardhat::load_hardhat, read_file_to_string,
    visitor::ast_visitor::Node,
};
use std::path::PathBuf;

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
            contract_source
                .ast
                .accept(&mut context_loader)
                .unwrap_or_else(|err| {
                    // Exit with a non-zero exit code
                    eprintln!("Error loading Hardhat AST into ContextLoader");
                    eprintln!("{:?}", err);
                    std::process::exit(1);
                });
            let source_file_path = root_path.join(key);
            match read_file_to_string(&source_file_path) {
                Ok(content) => {
                    for unit in &context_loader.source_units {
                        if let Some(ref abs_path) = unit.absolute_path {
                            if abs_path == key {
                                context_loader.set_source_unit_source_content(unit.id, content);
                                break;
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Error reading Solidity source file: {}",
                        source_file_path.to_str().unwrap()
                    );
                    eprintln!("{:?}", err);
                }
            }
        }
    }

    (src_path, context_loader)
}
