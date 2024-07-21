use cyfrin_foundry_compilers::{artifacts::Source, CompilerInput, Solc};

use std::{
    process::{Command, Stdio},
    sync::Arc,
};

use crate::{
    ast::SourceUnit,
    context::{graph::traits::Transpose, workspace_context::WorkspaceContext},
};
use crate::{context::graph::WorkspaceCallGraph, visitor::ast_visitor::Node};

use super::ensure_valid_solidity_file;

#[cfg(test)]
pub fn load_solidity_source_unit_with_callgraphs(filepath: &str) -> WorkspaceContext {
    _load_solidity_source_unit(filepath, true)
}

#[cfg(test)]
pub fn load_solidity_source_unit(filepath: &str) -> WorkspaceContext {
    _load_solidity_source_unit(filepath, false)
}

#[cfg(test)]
fn _load_solidity_source_unit(filepath: &str, should_load_callgraphs: bool) -> WorkspaceContext {
    let solidity_file = &ensure_valid_solidity_file(filepath);
    let solidity_content = std::fs::read_to_string(solidity_file).unwrap();

    let compiler_input = CompilerInput::new(solidity_file.as_path()).unwrap();
    let compiler_input = compiler_input.first().unwrap(); // There's only 1 file in the path

    let version = Solc::detect_version(&Source {
        content: Arc::new(solidity_content.clone()),
    })
    .unwrap();

    let solc = Solc::find_or_install_svm_version(format!("{}", version)).unwrap();
    let solc_bin = solc.solc.to_str().unwrap();

    let file_arg = compiler_input
        .sources
        .first_key_value()
        .unwrap()
        .0
        .to_str()
        .unwrap();

    let command = Command::new(solc_bin)
        .args(["--ast-compact-json", file_arg])
        .current_dir("/")
        .stdout(Stdio::piped())
        .output();

    if let Ok(command) = command {
        let stdout = String::from_utf8(command.stdout).unwrap();
        let stderr = String::from_utf8(command.stderr).unwrap();
        println!("stderr = {}", stderr);

        let mut context = WorkspaceContext::default();
        let lines = stdout.lines().collect::<Vec<_>>();
        let mut idx = 0;
        let mut keep_picking = false;
        let mut ast_content = String::new();

        while idx < lines.len() {
            let line = lines[idx];

            let (separation, filename) = is_demarcation_line(line, vec![file_arg]);

            if separation {
                if !ast_content.is_empty() {
                    absorb_ast_content_into_context(
                        &ast_content,
                        solidity_content.clone(),
                        &mut context,
                    );
                }
                ast_content = String::new();
                keep_picking = false;

                if filename.is_some() {
                    keep_picking = true;
                }
            } else if keep_picking {
                ast_content.push_str(line);
            }

            idx += 1;
        }

        if !ast_content.is_empty() {
            absorb_ast_content_into_context(&ast_content, solidity_content.clone(), &mut context);
        }

        if should_load_callgraphs {
            load_callgraphs(&mut context);
        }

        context
    } else {
        eprintln!("Error running solc command");
        std::process::exit(1);
    }
}

fn load_callgraphs(context: &mut WorkspaceContext) {
    let forward_callgraph = WorkspaceCallGraph::from_context(context).unwrap();
    let reverse_callgraph = WorkspaceCallGraph {
        graph: forward_callgraph.graph.reverse(),
    };
    context.forward_callgraph = Some(forward_callgraph);
    context.reverse_callgraph = Some(reverse_callgraph);
}

fn absorb_ast_content_into_context(
    ast_content: &str,
    solidity_content: String,
    context: &mut WorkspaceContext,
) {
    let mut source_unit: SourceUnit = serde_json::from_str(ast_content).unwrap();
    source_unit.source = Some(solidity_content);
    source_unit.accept(context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading AST into WorkspaceContext");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}

fn is_demarcation_line(line: &str, file_args: Vec<&str>) -> (bool, Option<String>) {
    if line.starts_with("======= ") {
        let end_marker = line.find(" =======").unwrap();
        let filepath = &line["======= ".len()..end_marker];
        if file_args.iter().any(|file_arg| file_arg.contains(filepath)) {
            return (true, Some(filepath.to_string()));
        }
        return (true, None);
    }
    (false, None)
}

#[cfg(test)]
/// This function is dangerous to use because we force all the sol files into 1 Workspace Context.
/// As a result, we may override Node IDs. Therefore, this function is only available in cfg(test)
#[allow(dead_code)]
pub fn load_multiple_solidity_source_units_into_single_context(
    filepaths: &[&str],
) -> WorkspaceContext {
    use std::collections::HashMap;

    let mut context = WorkspaceContext::default();
    let mut file_args = vec![];
    let mut solc_bin: Option<String> = None;
    let mut solidity_content = HashMap::new();

    for &filepath in filepaths {
        let solidity_file = &ensure_valid_solidity_file(filepath);
        let this_solidity_content = std::fs::read_to_string(solidity_file).unwrap();

        let file_arg = std::fs::canonicalize(solidity_file).unwrap();
        let file_arg = file_arg.to_string_lossy().to_string();

        let version = Solc::detect_version(&Source {
            content: Arc::new(this_solidity_content.clone()),
        })
        .unwrap();

        let solc = Solc::find_or_install_svm_version(format!("{}", version)).unwrap();
        let this_solc_bin = solc.solc.to_string_lossy().to_string();

        if solc_bin.is_none() {
            solc_bin = Some(this_solc_bin.clone());
        } else if solc_bin.clone().unwrap() != this_solc_bin {
            panic!(
                "Multiple solidity versions not supported yet in the test architecture because \
                they demand creation of multiple contexts!"
            );
        }
        solidity_content.insert(file_arg.clone(), this_solidity_content);
        file_args.push(file_arg.clone());
    }

    let command = Command::new(solc_bin.clone().unwrap().clone())
        .arg("--ast-compact-json")
        .args(file_args.clone())
        .current_dir("/")
        .stdout(Stdio::piped())
        .output();

    if let Ok(command) = command {
        let stdout = String::from_utf8(command.stdout).unwrap();
        let stderr = String::from_utf8(command.stderr).unwrap();
        println!("stderr = {}", stderr);

        let lines = stdout.lines().collect::<Vec<_>>();
        let mut idx = 0;
        let mut keep_picking = false;
        let mut ast_content = String::new();
        let mut sol_content = String::new();

        let my_file_args = file_args.clone();
        let my_file_args: Vec<&str> = my_file_args.iter().map(|x| x.as_str()).collect();

        while idx < lines.len() {
            let line = lines[idx];

            let (separation, filename) = is_demarcation_line(line, my_file_args.clone());

            if separation {
                if !ast_content.is_empty() {
                    absorb_ast_content_into_context(
                        &ast_content,
                        sol_content.clone(),
                        &mut context,
                    );
                }
                ast_content = String::new();
                keep_picking = false;

                if let Some(name) = filename {
                    keep_picking = true;
                    let mut lookup = "/".to_string();
                    lookup.push_str(&name);
                    sol_content = solidity_content.get(lookup.as_str()).unwrap().clone();
                }
            } else if keep_picking {
                ast_content.push_str(line);
            }

            idx += 1;
        }

        if !ast_content.is_empty() {
            absorb_ast_content_into_context(&ast_content, sol_content, &mut context);
        }
    } else {
        eprintln!("Error running solc command");
        std::process::exit(1);
    }
    context
}
