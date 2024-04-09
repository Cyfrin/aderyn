use crate::{process_foundry, process_hardhat, virtual_foundry};
use aderyn_core::{
    context::workspace_context::WorkspaceContext,
    detect::detector::IssueDetector,
    fscloc,
    report::{json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter},
    run_with_printer, run_with_printer_and_given_detectors,
};
use std::{env, fs::read_dir, path::PathBuf};

pub struct Args {
    pub root: String,
    pub output: String,
    pub exclude: Option<Vec<String>>,
    pub scope: Option<Vec<String>>,
    pub no_snippets: bool,
    pub stdout: bool,
}

enum Framework {
    Foundry,
    Hardhat,
}

pub fn drive(args: Args) {
    let output = args.output.clone();
    let cx_wrapper = make_context(&args);
    let root_rel_path = PathBuf::from(&args.root);
    let context = &cx_wrapper.context;

    if args.output.ends_with(".json") {
        // Load the workspace context into the run function, which runs the detectors
        run_with_printer(
            context,
            output,
            JsonPrinter,
            root_rel_path,
            args.no_snippets,
            args.stdout,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    } else {
        // Load the workspace context into the run function, which runs the detectors
        run_with_printer(
            context,
            output,
            MarkdownReportPrinter,
            root_rel_path,
            args.no_snippets,
            args.stdout,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    }
}

pub fn drive_with(args: Args, detectors: Vec<Box<dyn IssueDetector>>) {
    let output = args.output.clone();
    let cx_wrapper = make_context(&args);
    let root_rel_path = PathBuf::from(&args.root);
    let context = &cx_wrapper.context;

    if args.output.ends_with(".json") {
        // Load the workspace context into the run function, which runs the detectors
        run_with_printer_and_given_detectors(
            context,
            output,
            JsonPrinter,
            root_rel_path,
            args.no_snippets,
            args.stdout,
            detectors,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    } else {
        // Load the workspace context into the run function, which runs the detectors
        run_with_printer_and_given_detectors(
            context,
            output,
            MarkdownReportPrinter,
            root_rel_path,
            args.no_snippets,
            args.stdout,
            detectors,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    }
}

pub struct WorkspaceContextWrapper {
    pub context: WorkspaceContext,
    is_single_file: bool,
    safe_space: PathBuf,
}

// when the variable goes out of scope, workspace will be deleted !
impl Drop for WorkspaceContextWrapper {
    fn drop(&mut self) {
        if self.is_single_file {
            virtual_foundry::delete_safe_space(&self.safe_space);
        }
    }
}

fn make_context(args: &Args) -> WorkspaceContextWrapper {
    if !args.output.ends_with(".json") && !args.output.ends_with(".md") {
        eprintln!("Warning: output file lacks the \".md\" or \".json\" extension in its filename.");
    }

    let is_single_file = args.root.ends_with(".sol") && PathBuf::from(&args.root).is_file();
    let mut safe_space = PathBuf::new();

    let (src_path, mut context) = {
        if is_single_file {
            safe_space = virtual_foundry::build_isolated_workspace_for_file(&args.root);
            process_foundry::with_project_root_at(&safe_space, &args.scope, &args.exclude)
        } else {
            println!("Detecting framework...");
            let root_path = PathBuf::from(&args.root);
            let framework = detect_framework(root_path.clone()).unwrap_or_else(|| {
                // Exit with a non-zero exit code
                eprintln!("Error detecting framework");
                eprintln!("Neither foundry.toml nor hardhat.config.json was found in the project directory!");
                eprintln!();
                eprintln!("NOTE: \nIf Foundry is detected in the project root, Aderyn will first run `forge build --ast` to ensure that the contract compiles correctly and the latest artifacts are available.");
                eprintln!("If Hardhat is detected, Aderyn does not auto-compile. Make sure to run `hardhat compile` BEFORE running Aderyn.");
                std::process::exit(1);
            });

            // This whole block loads the solidity files and ASTs into the workspace context
            match framework {
                Framework::Foundry => {
                    process_foundry::with_project_root_at(&root_path, &args.scope, &args.exclude)
                }
                Framework::Hardhat => {
                    process_hardhat::with_project_root_at(&root_path, &args.scope, &args.exclude)
                }
            }
        }
    };

    let key = "ADERYN_CLOC_SKIP";

    let should_cloc = match env::var(key) {
        Ok(val) => val != "1",
        Err(_) => true,
    };

    if should_cloc {
        // Using the source path, calculate the sloc
        let stats =
            fscloc::engine::count_lines_of_code(&PathBuf::from(src_path), &context.src_filepaths);
        let stats = stats.lock().unwrap().to_owned();
        context.set_sloc_stats(stats);
    }

    WorkspaceContextWrapper {
        context,
        is_single_file,
        safe_space,
    }
}

fn detect_framework(path: PathBuf) -> Option<Framework> {
    // Canonicalize the path
    let canonical_path = path.canonicalize().expect("Failed to canonicalize path");

    // Check if the directory exists
    if !canonical_path.is_dir() {
        return None;
    }

    // Read the contents of the directory
    let entries = read_dir(&canonical_path).expect("Failed to read directory");

    for entry in entries.flatten() {
        let filename = entry.file_name();
        match filename.to_str() {
            Some("foundry.toml") => return Some(Framework::Foundry),
            Some("hardhat.config.js") | Some("hardhat.config.ts") => {
                return Some(Framework::Hardhat)
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod foundry_compiler_tests {
    use foundry_compilers::{artifacts::Source, CompilerInput, Solc};
    use std::{
        path::{Path, PathBuf},
        process::{Command, Stdio},
        sync::Arc,
    };

    #[test]
    fn admin_contract_exists() {
        let cargo_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let tests_contract_playground_path = cargo_root
            .join("../tests/contract-playground/")
            .canonicalize()
            .unwrap();

        let admin_contract = tests_contract_playground_path.join("src/AdminContract.sol");

        assert!(admin_contract.exists());
    }

    #[test]
    fn can_detect_version() {
        let cargo_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let tests_contract_playground_path = cargo_root
            .join("../tests/contract-playground/")
            .canonicalize()
            .unwrap();

        let admin_contract_file = tests_contract_playground_path.join("src/AdminContract.sol");
        let admin_contract_file_content = std::fs::read_to_string(admin_contract_file).unwrap();

        // This will install the compiler if not there
        let version = Solc::detect_version(&Source {
            content: Arc::new(admin_contract_file_content),
        });
        assert!(version.is_ok());
    }

    #[test]
    fn can_get_compiler_of_required_version() {
        let cargo_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let tests_contract_playground_path = cargo_root
            .join("../tests/contract-playground/")
            .canonicalize()
            .unwrap();

        let admin_contract_file = tests_contract_playground_path.join("src/AdminContract.sol");
        let admin_contract_file_content = std::fs::read_to_string(admin_contract_file).unwrap();

        // This will install the compiler if not there
        let version = Solc::detect_version(&Source {
            content: Arc::new(admin_contract_file_content),
        })
        .unwrap();

        let solc = Solc::find_or_install_svm_version(format!("{}", version));
        assert!(solc.is_ok());

        println!("Solc binary: {:?}", solc.unwrap());
    }

    #[test]
    fn can_generate_ast_for_admin_contract() {
        let cargo_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let tests_contract_playground_path = cargo_root
            .join("../tests/contract-playground/")
            .canonicalize()
            .unwrap();

        let admin_contract_file = tests_contract_playground_path.join("src/AdminContract.sol");
        let admin_contract_file_content = std::fs::read_to_string(&admin_contract_file).unwrap();

        // Step 1 - Gather the data to input to the compiler
        let compiler_input = CompilerInput::new(
            // This will work with root directory as well (it will fetch list of files)
            admin_contract_file.as_path(),
        )
        .unwrap();

        let ac_compiler_input = compiler_input.get(0).unwrap();

        // Step 2 - Detect the version of solc that can be used with compiler input
        let version = Solc::detect_version(&Source {
            // This will install the compiler if not there
            content: Arc::new(admin_contract_file_content),
        })
        .unwrap();

        // Step 3 -  Get a representation of binary to use for compiling
        let solc = Solc::find_or_install_svm_version(format!("{}", version)).unwrap();
        let solc_bin = solc.solc.to_str().unwrap();
        println!("Path to binary {}", solc_bin);

        assert!(Path::new(solc_bin).exists());

        // Step 4 - Run `solc --ast-compact-json <FILENAME.sol>`
        let command = Command::new(solc_bin)
            .args([
                "--ast-compact-json",
                ac_compiler_input
                    .sources
                    .first_key_value()
                    .unwrap()
                    .0
                    .to_str()
                    .unwrap(),
            ])
            .current_dir(tests_contract_playground_path)
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        let stdout = String::from_utf8(command.stdout).unwrap();

        println!("AST {}", stdout);
        assert!(!stdout.is_empty());
    }
}
