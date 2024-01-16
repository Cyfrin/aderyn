use crate::{process_foundry, process_hardhat, virtual_foundry};
use aderyn_core::{
    context::loader::ContextLoader,
    detect::detector::Detector,
    fscloc,
    report::{json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter},
    run_with_printer, run_with_printer_and_given_detectors,
};
use std::{fs::read_dir, path::PathBuf};

pub struct Args {
    pub root: String,
    pub output: String,
    pub exclude: Option<Vec<String>>,
    pub scope: Option<Vec<String>>,
    pub no_snippets: bool,
}

enum Framework {
    Foundry,
    Hardhat,
}

pub fn drive(args: Args) {
    let output = args.output.clone();
    let cx_wrapper = make_context_loader(&args);
    let root_rel_path = PathBuf::from(&args.root);
    let context_loader = &cx_wrapper.context_loader;

    if args.output.ends_with(".json") {
        // Load the context loader into the run function, which runs the detectors
        run_with_printer(
            context_loader,
            output,
            JsonPrinter,
            root_rel_path,
            args.no_snippets,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    } else {
        // Load the context loader into the run function, which runs the detectors
        run_with_printer(
            context_loader,
            output,
            MarkdownReportPrinter,
            root_rel_path,
            args.no_snippets,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    }
}

pub fn drive_with(args: Args, detectors: Vec<Box<dyn Detector>>) {
    let output = args.output.clone();
    let cx_wrapper = make_context_loader(&args);
    let root_rel_path = PathBuf::from(&args.root);
    let context_loader = &cx_wrapper.context_loader;

    if args.output.ends_with(".json") {
        // Load the context loader into the run function, which runs the detectors
        run_with_printer_and_given_detectors(
            context_loader,
            output,
            JsonPrinter,
            root_rel_path,
            args.no_snippets,
            detectors,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    } else {
        // Load the context loader into the run function, which runs the detectors
        run_with_printer_and_given_detectors(
            context_loader,
            output,
            MarkdownReportPrinter,
            root_rel_path,
            args.no_snippets,
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

pub struct ContextLoaderWrapper {
    pub context_loader: ContextLoader,
    is_single_file: bool,
    safe_space: PathBuf,
}

// when the variable goes out of scope, workspace will be deleted !
impl Drop for ContextLoaderWrapper {
    fn drop(&mut self) {
        if self.is_single_file {
            virtual_foundry::delete_safe_space(&self.safe_space);
        }
    }
}

fn make_context_loader(args: &Args) -> ContextLoaderWrapper {
    if !args.output.ends_with(".json") && !args.output.ends_with(".md") {
        eprintln!("Warning: output file lacks the \".md\" or \".json\" extension in its filename.");
    }

    let is_single_file = args.root.ends_with(".sol") && PathBuf::from(&args.root).is_file();
    let mut safe_space = PathBuf::new();

    let (src_path, mut context_loader) = {
        if is_single_file {
            safe_space = virtual_foundry::build_isolated_workspace_for_file(&args.root);
            process_foundry::with_project_root_at(&safe_space, &args.scope, &args.exclude)
        } else {
            println!("Detecting framework...");
            let root_path = PathBuf::from(&args.root);
            let framework = detect_framework(root_path.clone()).unwrap_or_else(|| {
                // Exit with a non-zero exit code
                eprintln!("Error detecting framework");
                std::process::exit(1);
            });

            // This whole block loads the solidity files and ASTs into the context loader
            // TODO: move much of this gutsy stuff into the foundry / hardhat modules.
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

    // Using the source path, calculate the sloc
    let stats = fscloc::engine::count_lines_of_code(
        &PathBuf::from(src_path),
        &context_loader.src_filepaths,
    );
    let stats = stats.lock().unwrap().to_owned();
    context_loader.set_sloc_stats(stats);

    ContextLoaderWrapper {
        context_loader,
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
