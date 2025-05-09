use crate::{
    compile,
    config::supplement_values_from_aderyn_toml,
    driver::{CliArgsCommonConfig, CliArgsInputConfig},
};
use aderyn_core::{
    context::{
        graph::{LegacyWorkspaceCallGraph, Transpose, WorkspaceCallGraphs},
        router::Router,
        workspace::WorkspaceContext,
    },
    stats,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct WorkspaceContextWrapper {
    pub contexts: Vec<WorkspaceContext>,
    pub root_path: PathBuf,
}

pub struct PreprocessedConfig {
    pub root_path: PathBuf,
    pub src: Option<String>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

pub fn make_context(
    args: &CliArgsInputConfig,
    common: &CliArgsCommonConfig,
) -> Result<WorkspaceContextWrapper, Box<dyn std::error::Error + Send + Sync>> {
    // Preprocess config by supplementing CLI args with aderyn.toml if exists
    let preprocessed_config = obtain_config_values(args.clone())?;

    let root_path = preprocessed_config.root_path.clone();

    // Compilation steps:
    // 1. Processes the above preprocessed config by translating them to runtime values Thanks to
    //    Cyfrin/solidity-ast-rs
    // 2. Parse those files and prepare ASTs.
    let mut contexts: Vec<WorkspaceContext> = compile::project(preprocessed_config, common.lsp)?;

    // Only make this an error when it's not in LSP mode
    if !common.lsp && contexts.iter().all(|c| c.src_filepaths.is_empty()) {
        let error = "No solidity files found in given scope!";
        eprintln!("{}", error);
        return Err(error.into());
    }

    // Supplement the context
    // 1. Inject nSLOC stats
    // 2. Collect lines marked by aderyn-ignore-line, aderyn-ignore-next-line
    // 3. Inject Legacy Callgraph
    // 4. Inject Router
    // 5. Inject New Callgraph
    for context in contexts.iter_mut() {
        let absolute_root_path = &ensure_valid_root_path(&root_path);
        let stats = stats::collect_stats(absolute_root_path.as_path(), common.skip_cloc, context);
        let sloc_stats: HashMap<String, usize> =
            stats.iter().map(|(key, value)| (key.to_owned(), value.code)).collect();

        let ignore_line_stats: HashMap<String, Vec<stats::IgnoreLine>> =
            stats.iter().map(|(key, value)| (key.to_owned(), value.ignore_lines.clone())).collect();

        context.set_sloc_stats(sloc_stats);
        context.set_ignore_lines_stats(ignore_line_stats);

        let inward_callgraph = LegacyWorkspaceCallGraph::from_context(context)?;
        let outward_callgraph =
            LegacyWorkspaceCallGraph { raw_callgraph: inward_callgraph.raw_callgraph.reverse() };
        context.inward_callgraph = Some(inward_callgraph);
        context.outward_callgraph = Some(outward_callgraph);

        let router = Router::build(context);
        context.router = Some(router);

        let callgraphs = WorkspaceCallGraphs::build(context);
        context.callgraphs = Some(callgraphs);
    }

    Ok(WorkspaceContextWrapper { contexts, root_path })
}

/// Supplement the CLI arguments with values from aderyn.toml
fn obtain_config_values(
    args: CliArgsInputConfig,
) -> Result<PreprocessedConfig, Box<dyn std::error::Error + Send + Sync>> {
    let root_path = PathBuf::from(&args.root);
    let aderyn_path = root_path.join("aderyn.toml");

    let current = PreprocessedConfig {
        root_path,
        src: args.src,
        exclude: args.path_excludes,
        include: args.path_includes,
    };

    // Process aderyn.toml if it exists
    if aderyn_path.exists() {
        return supplement_values_from_aderyn_toml(current);
    }
    Ok(current)
}

fn ensure_valid_root_path(root_path: &Path) -> PathBuf {
    if !root_path.exists() {
        eprintln!("{} does not exist!", root_path.to_string_lossy());
        std::process::exit(1);
    }
    std::fs::canonicalize(root_path).unwrap()
}
