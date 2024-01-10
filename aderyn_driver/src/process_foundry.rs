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
    scope: &Option<Vec<String>>,
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

    let foundry_intermediates = loaded_foundry
        .output_filepaths
        .par_iter()
        .filter_map(|output_filepath| {
            match read_foundry_output_file(output_filepath.to_str().unwrap()) {
                Ok(foundry_output) => {
                    // Check for exclusion first, regardless of scope
                    if let Some(excludes) = exclude {
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
                    // Check for scope if provided
                    if let Some(scopes) = scope {
                        if scopes.iter().any(|sc| {
                            foundry_output
                                .ast
                                .absolute_path
                                .as_ref()
                                .map_or(false, |path| path.contains(sc))
                        }) {
                            Some(foundry_output.ast) // Include if it's within the scope
                        } else {
                            None // Skip if it's not within the scope
                        }
                    } else {
                        Some(foundry_output.ast) // Include if scope is not specified
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Error reading Foundry output file: {}: {}",
                        output_filepath.to_str().unwrap(),
                        err
                    );
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    // Get deduplicated list of paths that have already been filtered
    let intermediate_paths: HashSet<String> = foundry_intermediates
        .iter()
        .filter_map(|ast_option| ast_option.absolute_path.clone())
        .collect();

    // read_foundry_output_file and print an error message if it fails
    foundry_intermediates.into_iter().for_each(|mut ast| {
        let absolute_path_clone = ast.absolute_path.clone();

        match read_file_to_string(&root_path.join(Path::new(&ast.absolute_path.as_ref().unwrap())))
        {
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
    fn test_process_foundry() {
        let root_path = PathBuf::from("../tests/contract-playground");
        let (_, context_loader) = super::with_project_root_at(&root_path, &None, &None);
        assert!(context_loader.src_filepaths.len() > 10);
    }

    #[test]
    fn test_process_foundry_scope() {
        let root_path = PathBuf::from("../tests/contract-playground");
        let scope: Option<Vec<String>> = Some(vec![
            "AnotherHeavilyCommentedContract.sol".to_string(),
            "Counter.sol".to_string(),
        ]);

        let (_, context_loader) = super::with_project_root_at(&root_path, &scope, &None);
        let contains_string = context_loader
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("AnotherHeavilyCommentedContract.sol"));
        assert!(context_loader.src_filepaths.len() == 2);
        assert!(contains_string);
    }

    #[test]
    fn test_process_foundry_exclude() {
        let root_path = PathBuf::from("../tests/contract-playground");
        let exclude: Option<Vec<String>> =
            Some(vec!["AnotherHeavilyCommentedContract.sol".to_string()]);

        let (_, context_loader) = super::with_project_root_at(&root_path, &None, &exclude);
        let contains_string = context_loader
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("AnotherHeavilyCommentedContract.sol"));
        assert!(!contains_string);
    }

    #[test]
    fn test_process_foundry_scope_and_exclude() {
        let root_path = PathBuf::from("../tests/contract-playground");
        let scope = Some(vec!["Inheritance".to_string()]);
        let exclude = Some(vec!["IContractInheritance.sol".to_string()]);

        let (_, context_loader) = super::with_project_root_at(&root_path, &scope, &exclude);
        let contains_scope = context_loader
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("ExtendedInheritance.sol"));
        let contains_exclude = context_loader
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("IContractInheritance.sol"));
        assert!(contains_scope && !contains_exclude);
    }

    #[test]
    fn test_process_foundry_directory_scope_and_exclude() {
        let root_path = PathBuf::from("../tests/contract-playground");
        let scope = Some(vec!["uniswap".to_string()]);
        let exclude = Some(vec!["UniswapV2Swapper.sol".to_string()]);

        let (_, context_loader) = super::with_project_root_at(&root_path, &scope, &exclude);
        let contains_scope = context_loader
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("UniswapV3Swapper.sol"));
        let contains_exclude = context_loader
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("UniswapV2Swapper.sol"));
        assert!(contains_scope && !contains_exclude);
    }
}
