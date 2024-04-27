use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use aderyn_core::{
    ast::SourceUnit, context::workspace_context::WorkspaceContext, visitor::ast_visitor::Node,
};
use foundry_compilers::{CompilerInput, Solc};

use crate::ensure_valid_root_path;

pub fn with_project_root_at(
    root_path: &Path,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
) -> Vec<WorkspaceContext> {
    let absolute_root_path = &ensure_valid_root_path(root_path);
    let absolute_root_path_str = &absolute_root_path.to_string_lossy().to_string();
    let compiler_input = CompilerInput::new(absolute_root_path).unwrap();
    let solidity_files = compiler_input
        .into_iter()
        .filter(|c| c.language == *"Solidity")
        .collect::<Vec<_>>();
    let solidity_files = &solidity_files[0]; // No Yul Support as of now

    // HashMap from "<Solc Version> --> [File1.sol, File2.sol, File3.sol]"
    let mut compilation_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (solidity_file, source) in solidity_files
        .sources
        .iter()
        .filter(|(solidity_file, _)| {
            passes_scope(
                scope,
                solidity_file.canonicalize().unwrap().as_path(),
                absolute_root_path_str,
            )
        })
        .filter(|(solidity_file, _)| {
            passes_exclude(
                exclude,
                solidity_file.canonicalize().unwrap().as_path(),
                absolute_root_path_str,
            )
        })
    {
        if let Ok(version) = Solc::detect_version(source) {
            match compilation_groups.entry(format!("{}", version)) {
                Entry::Occupied(mut o) => {
                    o.get_mut().push(solidity_file.into());
                }
                Entry::Vacant(v) => {
                    v.insert(vec![solidity_file.into()]);
                }
            };
        }
    }

    // dbg!(&compilation_groups);

    let mut remappings = vec![];
    if let Some(custom_remappings) = read_remappings(root_path) {
        remappings.extend(custom_remappings);
        remappings.dedup();
    }

    let mut workspace_contexts: Vec<WorkspaceContext> = vec![];

    for (version, file_paths) in &compilation_groups {
        let solc = Solc::find_or_install_svm_version(version).unwrap();
        let solc_bin = solc.solc.to_str().unwrap();

        let command = Command::new(solc_bin)
            .args(remappings.clone())
            .arg("--ast-compact-json")
            .args(file_paths)
            .current_dir(root_path)
            .stdout(Stdio::piped())
            .output();

        if let Ok(command) = command {
            if !command.status.success() {
                let msg = String::from_utf8(command.stderr).unwrap();
                println!("stderr = {}", msg);
                println!("cwd = {}", root_path.display());
                panic!("Error running solc command");
            }
            let stdout = String::from_utf8(command.stdout).unwrap();
            let mut context = WorkspaceContext::default();
            // dbg!(&stdout)

            let mut pick_next_line = false;
            let mut src_filepaths = vec![];

            for line in stdout.lines() {
                if line.starts_with("======= ") {
                    let end_marker = line.find(" =======").unwrap();
                    let filepath = &PathBuf::from(&line["======= ".len()..end_marker]);
                    if passes_scope(
                        scope,
                        root_path.join(filepath).canonicalize().unwrap().as_path(),
                        absolute_root_path_str,
                    ) && passes_exclude(
                        exclude,
                        root_path.join(filepath).canonicalize().unwrap().as_path(),
                        absolute_root_path_str,
                    ) {
                        src_filepaths.push(filepath.to_string_lossy().to_string());
                        pick_next_line = true;
                    }
                } else if pick_next_line {
                    let ast_content = line.to_string();
                    let mut source_unit: SourceUnit = serde_json::from_str(&ast_content).unwrap();
                    let filepath = source_unit.absolute_path.as_ref().unwrap();
                    source_unit.source = std::fs::read_to_string(&root_path.join(filepath)).ok();
                    // dbg!(&filepath);
                    source_unit.absolute_path = Some(filepath.to_string());
                    // dbg!(&filepath);

                    source_unit.accept(&mut context).unwrap_or_else(|err| {
                        // Exit with a non-zero exit code
                        eprintln!("Error loading AST into WorkspaceContext");
                        eprintln!("{:?}", err);
                        std::process::exit(1);
                    });

                    pick_next_line = false;
                }
            }

            // println!("{:#?}", context);
            // println!("New context !");
            context.src_filepaths = src_filepaths;
            workspace_contexts.push(context);
        } else {
            eprintln!("Error running solc command");
        }
    }

    workspace_contexts
}

fn passes_scope(
    scope: &Option<Vec<String>>,
    solidity_file: &Path,
    absolute_root_path_str: &str,
) -> bool {
    let sol_path_string = solidity_file.to_string_lossy().to_string();
    let pos = sol_path_string.find(absolute_root_path_str).unwrap();
    let window = &sol_path_string[pos + absolute_root_path_str.len()..];

    if let Some(scope) = scope {
        for include in scope {
            if window.contains(include) {
                return true;
            }
        }
        return false;
    }

    true
}

fn passes_exclude(
    exclude: &Option<Vec<String>>,
    solidity_file: &Path,
    absolute_root_path_str: &str,
) -> bool {
    let sol_path_string = solidity_file.to_string_lossy().to_string();
    let pos = sol_path_string.find(absolute_root_path_str).unwrap();
    let window = &sol_path_string[pos + absolute_root_path_str.len()..];

    if let Some(exclude) = exclude {
        for dont_include in exclude {
            if window.contains(dont_include) {
                return false;
            }
        }
        return true;
    }

    true
}

// Return a list of remappings in the format ["a=b", "c=d", "e=f"]
// where direct imports a,c,e map to b,d,f
fn read_remappings(root_path: &Path) -> Option<Vec<String>> {
    // Look for a file called `remappings` in the project root. If not present, assume project doesn't require remappings
    let remappings_file = root_path.join("remappings").canonicalize().ok()?;
    let remappings_content = std::fs::read_to_string(remappings_file).unwrap();
    Some(remappings_content.lines().map(|x| x.to_owned()).collect())
}
