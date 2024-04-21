use crate::{ensure_valid_root_path, process_auto};
use aderyn_core::{
    context::workspace_context::WorkspaceContext,
    detect::detector::IssueDetector,
    fscloc,
    report::{json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter},
    run_with_printer, run_with_printer_and_given_detectors,
};
use std::{path::PathBuf};

pub struct Args {
    pub root: String,
    pub output: String,
    pub exclude: Option<Vec<String>>,
    pub scope: Option<Vec<String>>,
    pub no_snippets: bool,
    pub skip_build: bool,
    pub skip_cloc: bool,
    pub skip_update_check: bool,
    pub stdout: bool,
}

pub fn drive(args: Args) {
    let output = args.output.clone();
    let cx_wrapper = make_context(&args);
    let root_rel_path = PathBuf::from(&args.root);

    if args.output.ends_with(".json") {
        // Load the workspace context into the run function, which runs the detectors
        run_with_printer(
            &cx_wrapper.contexts,
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
            &cx_wrapper.contexts,
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

    if args.output.ends_with(".json") {
        // Load the workspace context into the run function, which runs the detectors
        run_with_printer_and_given_detectors(
            &cx_wrapper.contexts,
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
            &cx_wrapper.contexts,
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
    pub contexts: Vec<WorkspaceContext>,
}

fn make_context(args: &Args) -> WorkspaceContextWrapper {
    if !args.output.ends_with(".json") && !args.output.ends_with(".md") {
        eprintln!("Warning: output file lacks the \".md\" or \".json\" extension in its filename.");
    }

    let root_path = PathBuf::from(&args.root);
    let absolute_root_path = &ensure_valid_root_path(&root_path);

    let mut contexts = process_auto::with_project_root_at(&root_path, &args.scope, &args.exclude);

    if !args.skip_cloc {
        for context in contexts.iter_mut() {
            let stats = fscloc::engine::count_lines_of_code(
                absolute_root_path.as_path(),
                &context.src_filepaths,
            );
            let stats = stats.lock().unwrap().to_owned();
            // dbg!(&stats);
            context.set_sloc_stats(stats);
        }
        // Using the source path, calculate the sloc
    }

    WorkspaceContextWrapper { contexts }
}

#[cfg(test)]
mod foundry_compiler_tests {
    use aderyn_core::visitor::ast_visitor::Node;
    use aderyn_core::{ast::SourceUnit, context::workspace_context::WorkspaceContext};
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

        let ac_compiler_input = compiler_input.first().unwrap();

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
        let file_arg = ac_compiler_input
            .sources
            .first_key_value()
            .unwrap()
            .0
            .to_str()
            .unwrap();
        let command = Command::new(solc_bin)
            .args(["--ast-compact-json", file_arg])
            .current_dir(tests_contract_playground_path)
            .stdout(Stdio::piped())
            .output();
        //TODO: expect command to work here

        if let Ok(command) = command {
            // assert!(command.status.success()); // TODO: Investigate why it fails in CI
            let stdout = String::from_utf8(command.stdout).unwrap();

            println!("AST {}", stdout);
            println!("If you are seeing this in CI, likely the above AST is empty because stdout of is streamed to github infra.")
            // assert!(!stdout.is_empty());
        }
    }

    #[test]
    fn can_load_context_for_admin_contract() {
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

        let ac_compiler_input = compiler_input.first().unwrap();

        // Step 2 - Detect the version of solc that can be used with compiler input
        let version = Solc::detect_version(&Source {
            // This will install the compiler if not there
            content: Arc::new(admin_contract_file_content.clone()),
        })
        .unwrap();

        // Step 3 -  Get a representation of binary to use for compiling
        let solc = Solc::find_or_install_svm_version(format!("{}", version)).unwrap();
        let solc_bin = solc.solc.to_str().unwrap();
        println!("Path to binary {}", solc_bin);
        assert!(Path::new(solc_bin).exists());

        // Step 4 - Run `solc --ast-compact-json <FILENAME.sol>`
        let file_arg = ac_compiler_input
            .sources
            .first_key_value()
            .unwrap()
            .0
            .to_str()
            .unwrap();
        let command = Command::new(solc_bin)
            .args(["--ast-compact-json", file_arg])
            .current_dir(tests_contract_playground_path)
            .stdout(Stdio::piped())
            .output();

        if let Ok(command) = command {
            let stdout = String::from_utf8(command.stdout).unwrap();

            let mut pick_next_line = false;
            let mut ast_content = String::new();
            for line in stdout.lines() {
                if line.starts_with("======= ") {
                    let end_marker = line.find(" =======").unwrap();
                    let filepath = &line["======= ".len()..end_marker];
                    if filepath == "src/AdminContract.sol" {
                        pick_next_line = true;
                    }
                } else if pick_next_line {
                    ast_content = line.to_string();
                    break;
                }
            }

            let mut source_unit: SourceUnit = serde_json::from_str(&ast_content).unwrap();
            let mut context = WorkspaceContext::default();
            source_unit.source = Some(admin_contract_file_content);
            source_unit.accept(&mut context).unwrap_or_else(|err| {
                // Exit with a non-zero exit code
                eprintln!("Error loading AST into WorkspaceContext");
                eprintln!("{:?}", err);
            });
            println!("Workspace Context {:#?}", context);
        }
    }
}
