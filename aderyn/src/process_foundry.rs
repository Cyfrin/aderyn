use aderyn_core::{
    context::loader::ContextLoader,
    framework::foundry::{load_foundry, read_foundry_output_file},
    read_file_to_string,
    visitor::ast_visitor::Node,
};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

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
    foundry_intermediates
        .into_iter()
        .flatten()
        .for_each(|mut ast| {
            let absolute_path_clone = ast.absolute_path.clone(); // Clone the path

            match read_file_to_string(
                &root_path.join(Path::new(&ast.absolute_path.as_ref().unwrap())),
            ) {
                Ok(content) => {
                    ast.source = Some(content);
                }
                Err(err) => {
                    eprintln!(
                        "Error reading Solidity source file: {:?}",
                        absolute_path_clone.unwrap() // Use the cloned path here
                    );
                    eprintln!("{:?}", err);
                }
            }

            ast.accept(&mut context_loader).unwrap_or_else(|err| {
                // Exit with a non-zero exit code
                eprintln!("Error loading Foundry AST into ContextLoader");
                eprintln!("{:?}", err);
                std::process::exit(1);
            });
        });

    (src_path, context_loader)
}
