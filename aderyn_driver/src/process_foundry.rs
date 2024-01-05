use aderyn_core::{
    context::loader::ContextLoader,
    framework::foundry::{load_foundry, read_foundry_output_file},
    read_file_to_string,
    visitor::ast_visitor::Node,
};
use rayon::prelude::*;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub fn with_project_root_at(
    root_path: &PathBuf,
    exclude: &Option<Vec<String>>,
) -> (String, ContextLoader) {
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
                Ok(foundry_output) => {
                    if let Some(excludes) = &exclude {
                        if excludes.iter().any(|ex| {
                            foundry_output
                                .ast
                                .absolute_path
                                .as_ref()
                                .map_or(false, |path| path.contains(ex))
                        }) {
                            return None; // Skip if the path matches any exclude pattern
                        }
                    }
                    Some(foundry_output.ast)
                }
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

    // Get deduplicated list of paths that have already been filtered
    let intermediate_paths: HashSet<String> = foundry_intermediates
        .iter()
        .filter_map(|ast_option| ast_option.as_ref()?.absolute_path.clone())
        .collect();

    // read_foundry_output_file and print an error message if it fails
    foundry_intermediates
        .into_iter()
        .flatten()
        .for_each(|mut ast| {
            let absolute_path_clone = ast.absolute_path.clone();
            println!("Processing: {:?}", &absolute_path_clone);

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

    context_loader.src_filepaths = intermediate_paths.into_iter().collect();
    (src_path, context_loader)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_process_foundry_exclude() {
        let root_path = PathBuf::from("../tests/contract-playground");
        let exclude: Option<Vec<String>> =
            Some(vec!["AnotherHeavilyCommentedContract.sol".to_string()]);

        let (_, context_loader) = super::with_project_root_at(&root_path, &exclude);
        let contains_string = context_loader
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("AnotherHeavilyCommentedContract.sol"));
        assert!(!contains_string);
    }
}
