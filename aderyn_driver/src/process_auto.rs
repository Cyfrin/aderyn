use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
    process::{Command, Stdio},
};

use aderyn_core::context::workspace_context::WorkspaceContext;
use foundry_compilers::{CompilerInput, Solc};

pub fn with_project_root_at(
    root_path: &PathBuf,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
) -> (String, WorkspaceContext) {
    let absolute_root_path = &ensure_valid_root_path(root_path);
    let absolute_root_path_str = &absolute_root_path.to_string_lossy().to_string();
    let compiler_input = CompilerInput::new(absolute_root_path).unwrap();
    let solidity_files = compiler_input
        .into_iter()
        .filter(|c| c.language == "Solidity".to_string())
        .collect::<Vec<_>>();
    let solidity_files = &solidity_files[0]; // No Yul Support as of now

    // HashMap from "<Solc Version> --> [File1.sol, File2.sol, File3.sol]"
    let mut compilation_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (solidity_file, source) in solidity_files
        .sources
        .iter()
        .filter(|(solidity_file, _source)| {
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
        })
        .filter(|(solidity_file, _source)| {
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

    for (version, file_paths) in &compilation_groups {
        let solc = Solc::find_or_install_svm_version(format!("{}", version)).unwrap();
        let solc_bin = solc.solc.to_str().unwrap();

        let command = Command::new(solc_bin)
            .arg("--ast-compact-json")
            .args(file_paths)
            .current_dir("/")
            .stdout(Stdio::piped())
            .output();

        if let Ok(command) = command {
            let stdout = String::from_utf8(command.stdout).unwrap();
            println!("{}", stdout);
        } else {
            eprintln!("Error running solc command");
            std::process::exit(1);
        }
    }

    eprintln!("No compilation groups found!");
    std::process::exit(1);
}

fn ensure_valid_root_path(root_path: &PathBuf) -> PathBuf {
    if !root_path.exists() {
        eprintln!("{} does not exist!", root_path.to_string_lossy());
        std::process::exit(1);
    }
    root_path.canonicalize().unwrap()
}
