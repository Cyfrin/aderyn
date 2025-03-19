#[cfg(test)]
use crate::context::graph::{Transpose, WorkspaceCallGraph};
use crate::{
    ast::SourceUnit, context::workspace_context::WorkspaceContext, visitor::ast_visitor::Node,
};
use foundry_compilers_aletheia::{
    derive_ast_and_evm_info, AstSourceFile, IncludeConfig, ProjectConfigInput,
    ProjectConfigInputBuilder, SolcVersionConfig, Source,
};
use semver::Version;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use super::ensure_valid_solidity_file;

pub fn load_solidity_source_unit(filepath: &str) -> WorkspaceContext {
    let solidity_file = &ensure_valid_solidity_file(filepath);

    let root = guess_root(&solidity_file.display().to_string());
    let suffix = solidity_file.strip_prefix(&root).unwrap();

    let project_config = ProjectConfigInputBuilder::new(&root)
        .with_include(IncludeConfig::Specific(vec![suffix.display().to_string()]))
        .build()
        .unwrap();

    make_context(&project_config)
}

/// Make sure all files belong to contract-playground
/// This function is dangerous to use because we force all the sol files into 1 Workspace Context.
/// As a result, we may override Node IDs. Therefore, this function is only available in cfg(test)
pub fn load_multiple_solidity_source_units_into_single_context(
    filepaths: &[&str],
    version: Version,
) -> WorkspaceContext {
    assert!(filepaths.len() > 0);
    let root = guess_root(filepaths[0]);

    let mut suffixes = vec![];

    for filepath in filepaths {
        let solidity_file = &ensure_valid_solidity_file(filepath);
        let suffix = solidity_file.strip_prefix(&root).unwrap();
        suffixes.push(suffix.display().to_string());
    }

    let project_config = ProjectConfigInputBuilder::new(&root)
        .with_include(IncludeConfig::Specific(suffixes))
        .with_solc_version(SolcVersionConfig::Specific(version))
        .build()
        .unwrap();

    make_context(&project_config)
}

fn guess_root(chunk: &str) -> PathBuf {
    if chunk.contains("contract-playground") {
        std::fs::canonicalize(Path::new("../tests/contract-playground")).unwrap()
    } else if chunk.contains("adhoc-sol-files") {
        std::fs::canonicalize(Path::new("../tests/adhoc-sol-files")).unwrap()
    } else if chunk.contains("2024-07-templegold") {
        std::fs::canonicalize(Path::new("../tests/2024-07-templegold/protocol")).unwrap()
    } else if chunk.contains("hardhat-js-playground") {
        std::fs::canonicalize(Path::new("../tests/hardhat-js-playground")).unwrap()
    } else if chunk.contains("ccip-contracts") {
        std::fs::canonicalize(Path::new("../tests/ccip-contracts")).unwrap()
    } else {
        todo!("add more roots as you see fit");
    }
}

fn make_context(project_config: &ProjectConfigInput) -> WorkspaceContext {
    let ast_evm_info = derive_ast_and_evm_info(project_config).unwrap();
    let ast_info = ast_evm_info.versioned_asts.first().unwrap();

    let mut context = WorkspaceContext::default();

    let sources = ast_info.sources.0.clone();
    let sources_ast = ast_info.compiler_output.sources.clone();
    let included = ast_info.included_files.clone();

    for cerror in ast_info.compiler_output.errors.clone() {
        if cerror.severity.is_error() {
            panic!("Compilation Error: {}", cerror);
        }
    }

    for (source_path, ast_source_file) in sources_ast {
        if included.contains(&source_path) {
            let content = sources.get(&source_path).cloned().expect("content not found");
            absorb_ast_content_into_context(
                ast_source_file,
                &mut context,
                content,
                &project_config.project_paths.root,
            );
            let relative_suffix =
                source_path.strip_prefix(&project_config.project_paths.root).unwrap();
            context.src_filepaths.push(relative_suffix.to_string_lossy().to_string());
        }
    }

    fn load_callgraphs(context: &mut WorkspaceContext) {
        let inward_callgraph = WorkspaceCallGraph::from_context(context).unwrap();
        let outward_callgraph =
            WorkspaceCallGraph { raw_callgraph: inward_callgraph.raw_callgraph.reverse() };
        context.inward_callgraph = Some(inward_callgraph);
        context.outward_callgraph = Some(outward_callgraph);
    }

    load_callgraphs(&mut context);

    context
}

fn absorb_ast_content_into_context(
    ast_source_file: AstSourceFile,
    context: &mut WorkspaceContext,
    content: Source,
    absolute_root_path: &Path,
) {
    let Some(ast_content) = ast_source_file.ast else {
        eprintln!("Warning: AST not found in output");
        return;
    };

    let Ok(mut source_unit) = serde_json::from_str::<SourceUnit>(&ast_content) else {
        eprintln!("Unable to serialize Source Unit from AST - \n{}\n", &ast_content);
        let error = serde_json::from_str::<SourceUnit>(&ast_content).unwrap_err();
        eprintln!("{:?}", error);
        std::process::exit(1);
    };

    // Set the source
    source_unit.source = Some(content.content.to_string());

    // Adjust the asbolute filepath to be relative
    let filepath = source_unit.absolute_path.as_ref().unwrap();
    let relative_path = PathBuf::from_str(filepath).unwrap();
    let relative_path = relative_path
        .strip_prefix(absolute_root_path)
        .expect("filepath in AST output is not absolute!");

    // Reset absolute path.
    source_unit.absolute_path = Some(relative_path.to_string_lossy().to_string());

    // TODO: Change absolute_path to type Path instead of String so we don't lose any unicode
    // characters (in the minority of cases)

    source_unit.accept(context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading AST into WorkspaceContext");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}
