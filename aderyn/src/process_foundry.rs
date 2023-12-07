use aderyn_core::{
    context::loader::ContextLoader,
    framework::foundry::{load_foundry, read_foundry_output_file},
    read_file_to_string,
    visitor::ast_visitor::Node,
};
use rayon::prelude::*;
use std::path::PathBuf;

pub fn with_project_root_at(root_path: &PathBuf) -> (String, ContextLoader) {
    let mut context_loader = ContextLoader::default();

    println!("Framework detected: Foundry mode engaged.");
    println!("Foundry root path: {:?}", root_path);
    let loaded_foundry = load_foundry(root_path).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading Foundry Root");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
    let src_path_buf = root_path.join(&loaded_foundry.src_path);
    let src_path = src_path_buf.to_str().unwrap().to_string();
    println!("Foundry src path: {:?}", src_path);

    // Load the foundry output files into the context loader using the ASTs
    let foundry_intermediates = loaded_foundry
        .output_filepaths
        .par_iter()
        .map(|output_filepath| {
            if let Ok(foundry_output) = read_foundry_output_file(output_filepath.to_str().unwrap())
            {
                Some(foundry_output.ast)
            } else {
                eprintln!(
                    "Error reading Foundry output file: {}",
                    output_filepath.to_str().unwrap()
                );
                None
            }
        })
        .collect::<Vec<_>>();

    // read_foundry_output_file and print an error message if it fails
    foundry_intermediates.into_iter().flatten().for_each(|ast| {
        ast.accept(&mut context_loader).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error loading Foundry AST into ContextLoader");
            eprintln!("{:?}", err);
            std::process::exit(1);
        })
    });

    // Load the solidity source files into memory, and assign the content to the source_unit.source
    for source_filepath in loaded_foundry.src_filepaths {
        match read_file_to_string(&source_filepath) {
            Ok(content) => {
                // Convert the full_path to a string
                let full_path_str = source_filepath.to_str().unwrap_or("");

                // Find the index where "src/" starts
                let src_component = src_path_buf.file_name().unwrap().to_str().unwrap();
                if let Some(start_index) = full_path_str.find(src_component) {
                    let target_path = &full_path_str[start_index..];

                    // Search for a match and modify
                    for unit in &context_loader.source_units {
                        if let Some(ref abs_path) = unit.absolute_path {
                            if abs_path == target_path {
                                context_loader.set_source_unit_source_content(unit.id, content);
                                break;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!(
                    "Error reading Solidity source file: {}",
                    source_filepath.to_str().unwrap()
                );
                eprintln!("{:?}", err);
            }
        }
    }

    (src_path, context_loader)
}
