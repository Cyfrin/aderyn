#[cfg(test)]
use crate::{
    ast::SourceUnit,
    context::{
        graph::{Transpose, WorkspaceCallGraph},
        workspace_context::WorkspaceContext,
    },
    visitor::ast_visitor::Node,
};

use super::ensure_valid_solidity_file;

pub fn load_solidity_source_unit(filepath: &str) -> WorkspaceContext {
    let solidity_file = &ensure_valid_solidity_file(filepath);
    todo!("write logic")
}

fn absorb_ast_content_into_context(
    ast_content: &str,
    solidity_content: String,
    context: &mut WorkspaceContext,
) {
    fn load_callgraphs(context: &mut WorkspaceContext) {
        let inward_callgraph = WorkspaceCallGraph::from_context(context).unwrap();
        let outward_callgraph =
            WorkspaceCallGraph { raw_callgraph: inward_callgraph.raw_callgraph.reverse() };
        context.inward_callgraph = Some(inward_callgraph);
        context.outward_callgraph = Some(outward_callgraph);
    }
    let mut source_unit: SourceUnit = serde_json::from_str(ast_content).unwrap();
    source_unit.source = Some(solidity_content);
    source_unit.accept(context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading AST into WorkspaceContext");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
    load_callgraphs(context);
}

/// This function is dangerous to use because we force all the sol files into 1 Workspace Context.
/// As a result, we may override Node IDs. Therefore, this function is only available in cfg(test)
#[allow(dead_code)]
pub fn load_multiple_solidity_source_units_into_single_context(
    filepaths: &[&str],
) -> WorkspaceContext {
    todo!("load multiple")
}
