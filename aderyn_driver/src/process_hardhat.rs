use aderyn_core::{
    context::workspace_context::WorkspaceContext, framework::hardhat::load_hardhat,
    read_file_to_string, visitor::ast_visitor::Node,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn with_project_root_at(
    root_path: &PathBuf,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
) -> (String, WorkspaceContext) {
    let mut context = WorkspaceContext::default();

    println!("Framework detected. Hardhat mode engaged.");
    println!("Hardhat root path: {:?}", root_path);
    let src_path = root_path.join("contracts").to_str().unwrap().to_string();
    let hardhat_output = load_hardhat(root_path).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading Hardhat build info");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });

    let filtered_output: HashMap<_, _> = hardhat_output
        .output
        .into_iter()
        .filter(|(_, contract_source)| {
            if let Some(scopes) = scope {
                scopes.iter().any(|sc| {
                    contract_source
                        .ast
                        .absolute_path
                        .as_ref()
                        .map_or(false, |path| path.contains(sc))
                })
            } else {
                true // If `scope` is not specified, do not filter out this contract_source.
            }
        })
        .filter(|(_, contract_source)| {
            if let Some(excludes) = &exclude {
                !excludes.iter().any(|ex| {
                    contract_source
                        .ast
                        .absolute_path
                        .as_ref()
                        .map_or(false, |path| path.contains(ex))
                })
            } else {
                true
            }
        })
        .collect();

    for (_, contract_source) in filtered_output.iter() {
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

        ast.accept(&mut context).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error loading Hardhat AST into WorkspaceContext");
            eprintln!("{:?}", err);
        });
    }
    context.src_filepaths = filtered_output.keys().cloned().collect();
    (src_path, context)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_process_hardhat() {
        let root_path = PathBuf::from("../tests/hardhat-js-playground");
        let (_, context) = super::with_project_root_at(&root_path, &None, &None);
        assert!(context.src_filepaths.len() > 6);
    }

    #[test]
    fn test_process_hardhat_scope() {
        let root_path = PathBuf::from("../tests/hardhat-js-playground");
        let scope: Option<Vec<String>> = Some(vec!["Counter.sol".to_string()]);

        let (_, context) = super::with_project_root_at(&root_path, &scope, &None);
        let contains_string = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("Counter.sol"));
        assert!(context.src_filepaths.len() == 1);
        assert!(contains_string);
    }

    #[test]
    fn test_process_hardhat_exclude() {
        let root_path = PathBuf::from("../tests/hardhat-js-playground");
        let exclude: Option<Vec<String>> = Some(vec!["Counter.sol".to_string()]);

        let (_, context) = super::with_project_root_at(&root_path, &None, &exclude);
        let contains_string = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("Counter.sol"));
        assert!(!contains_string);
    }

    #[test]
    fn test_process_hardhat_scope_and_exclude() {
        let root_path = PathBuf::from("../tests/hardhat-js-playground");
        let scope = Some(vec!["Inheritance".to_string()]);
        let exclude = Some(vec!["IContractInheritance.sol".to_string()]);
        let (_, context) = super::with_project_root_at(&root_path, &scope, &exclude);
        let has_extended_inheritance = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("ExtendedInheritance.sol"));
        let has_inheritance_base = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("InheritanceBase.sol"));
        let has_icontract_inheritance = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("IContractInheritance.sol"));

        assert!(has_extended_inheritance && has_inheritance_base && !has_icontract_inheritance);
    }

    #[test]
    fn test_process_hardhat_directory_scope_and_exclude() {
        let root_path = PathBuf::from("../tests/hardhat-js-playground");
        let scope = Some(vec!["contracts".to_string()]);
        let exclude = Some(vec!["IContractInheritance.sol".to_string()]);
        let (_, context) = super::with_project_root_at(&root_path, &scope, &exclude);
        let has_extended_inheritance = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("ExtendedInheritance.sol"));
        let has_inheritance_base = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("InheritanceBase.sol"));
        let has_icontract_inheritance = context
            .src_filepaths
            .iter()
            .any(|fp| fp.contains("IContractInheritance.sol"));

        assert!(has_extended_inheritance && has_inheritance_base && !has_icontract_inheritance);
    }
}
