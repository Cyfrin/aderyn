use foundry_compilers::{artifacts::Source, CompilerInput, Solc};

use std::{
    process::{Command, Stdio},
    sync::Arc,
};

use crate::visitor::ast_visitor::Node;
use crate::{ast::SourceUnit, context::workspace_context::WorkspaceContext};

use super::{ensure_valid_solidity_file, take_solidity_source_unit_loader_lock};

pub fn load_solidity_source_unit(filepath: &str) -> WorkspaceContext {
    let solidity_file = &ensure_valid_solidity_file(filepath);
    let solidity_content = std::fs::read_to_string(solidity_file).unwrap();

    let compiler_input = CompilerInput::new(solidity_file.as_path()).unwrap();
    let compiler_input = compiler_input.first().unwrap(); // There's only 1 file in the path

    let _lock = take_solidity_source_unit_loader_lock();
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
        println!("stdout = {}", stdout);
        println!("stderr = {}", stderr);

        let mut pick_next_line = false;
        let mut ast_content = String::new();
        for line in stdout.lines() {
            if line.starts_with("======= ") {
                let end_marker = line.find(" =======").unwrap();
                let filepath = &line["======= ".len()..end_marker];
                if file_arg.contains(filepath) {
                    pick_next_line = true;
                }
            } else if pick_next_line {
                ast_content = line.to_string();
                break;
            }
        }

        let mut source_unit: SourceUnit = serde_json::from_str(&ast_content).unwrap();
        let mut context = WorkspaceContext::default();
        source_unit.source = Some(solidity_content);
        source_unit.accept(&mut context).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error loading AST into WorkspaceContext");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
        context
    } else {
        eprintln!("Error running solc command");
        std::process::exit(1);
    }
}
