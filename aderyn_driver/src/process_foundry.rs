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
        .map(
            |output_filepath| match read_foundry_output_file(output_filepath.to_str().unwrap()) {
                Ok(foundry_output) => Some(foundry_output.ast),
                Err(err) => {
                    eprintln!(
                        "Error reading Foundry output file: {}: {}",
                        output_filepath.to_str().unwrap(),
                        err
                    );
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    // read_foundry_output_file and print an error message if it fails
    foundry_intermediates
        .into_iter()
        .flatten()
        .for_each(|mut ast| {
            let absolute_path_clone = ast.absolute_path.clone();

            match read_file_to_string(
                &root_path.join(Path::new(&ast.absolute_path.as_ref().unwrap())),
            ) {
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
                eprintln!("Error loading Foundry AST into ContextLoader");
                eprintln!("{:?}", err);
            });
        });

    let root_path_str = root_path
        .to_string_lossy()
        .into_owned()
        .trim_start_matches("./")
        .to_string();
    println!("root_path_str: {:?}", root_path_str);
    println!(
        "loaded_foundry.src_filepaths: {:?}",
        loaded_foundry.src_filepaths
    );
    context_loader.src_filepaths = loaded_foundry
        .src_filepaths
        .iter()
        .filter_map(|path| {
            let path_str = path.to_string_lossy();
            path_str.split(&root_path_str).nth(1).map(|s| s.to_string())
        })
        .collect::<Vec<_>>();

    println!(
        "context_loader.src_filepaths: {:?}",
        context_loader.src_filepaths
    );
    (src_path, context_loader)
}
