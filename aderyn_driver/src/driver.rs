use crate::{
    ensure_valid_root_path, foundry_config_helpers::derive_from_foundry_toml, process_auto,
    process_foundry,
};
use aderyn_core::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{get_all_issue_detectors, IssueDetector},
    fscloc,
    report::{
        json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter,
        sarif_printer::SarifPrinter,
    },
    run,
};
use std::{
    error::Error,
    fs::read_dir,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Args {
    pub root: String,
    pub output: String,
    pub src: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub scope: Option<Vec<String>>,
    pub no_snippets: bool,
    pub skip_build: bool,
    pub skip_cloc: bool,
    pub skip_update_check: bool,
    pub stdout: bool,
    pub auditor_mode: bool,
    pub icf: bool,
}

pub fn drive(args: Args) {
    drive_with(args, get_all_issue_detectors());
}

pub fn drive_with(args: Args, detectors: Vec<Box<dyn IssueDetector>>) {
    let output = args.output.clone();
    let cx_wrapper = make_context(&args);
    let root_rel_path = PathBuf::from(&args.root);

    if args.output.ends_with(".json") {
        // Load the workspace context into the run function, which runs the detectors
        run(
            &cx_wrapper.contexts,
            output,
            JsonPrinter,
            root_rel_path,
            args.no_snippets,
            args.stdout,
            args.auditor_mode,
            detectors,
        )
        .unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error running aderyn");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
    } else if args.output.ends_with(".sarif") {
        run(
            &cx_wrapper.contexts,
            output,
            SarifPrinter,
            root_rel_path,
            args.no_snippets,
            args.stdout,
            args.auditor_mode,
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
        run(
            &cx_wrapper.contexts,
            output,
            MarkdownReportPrinter,
            root_rel_path,
            args.no_snippets,
            args.stdout,
            args.auditor_mode,
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

    let (scope, exclude, src, remappings) = calculate_scope_exclude_and_src(args).unwrap();

    println!("Src - {:?}, Exclude - {:?}", src, exclude);

    let mut contexts: Vec<WorkspaceContext> = {
        if args.icf {
            process_auto::with_project_root_at(&root_path, &scope, &exclude, &src, &remappings)
        } else {
            if !is_foundry(&PathBuf::from(&args.root)) {
                // Exit with a non-zero exit code
                eprintln!("foundry.toml wasn't found in the project directory!");
                eprintln!();
                eprintln!("NOTE: \nAderyn will first run `forge build --ast` to ensure that the contract compiles correctly and the latest artifacts are available.");
                eprintln!("If you are using Hardhat, consider shifting to `--icf` mode");
                std::process::exit(1);
            };

            vec![process_foundry::with_project_root_at(
                &root_path,
                &scope,
                &exclude,
                args.skip_build,
            )]
        }
    };

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

#[allow(clippy::type_complexity)]
fn calculate_scope_exclude_and_src(
    args: &Args,
) -> Result<
    (
        Option<Vec<String>>, // Scope
        Option<Vec<String>>, // Exclude
        Option<Vec<String>>, // Src
        Option<Vec<String>>, // Remappings
    ),
    Box<dyn Error>,
> {
    let root_path = PathBuf::from(&args.root);
    for entry in std::fs::read_dir(&root_path)? {
        let entry = entry?;
        if entry.file_name() == "foundry.toml" {
            // If it is a foundry project, we auto fill scope, exclude, src from foundry.toml
            return Ok(derive_from_foundry_toml(
                &root_path,
                &args.scope,
                &args.exclude,
                &args.src,
            ));
        }
    }
    Ok((
        args.scope.clone(),
        args.exclude.clone(),
        args.src.clone(),
        None,
    ))
}

fn is_foundry(path: &Path) -> bool {
    // Canonicalize the path
    let canonical_path = path.canonicalize().expect("Failed to canonicalize path");

    // Check if the directory exists
    if !canonical_path.is_dir() {
        return false;
    }

    // Read the contents of the directory
    let entries = read_dir(&canonical_path).expect("Failed to read directory");

    for entry in entries.flatten() {
        let filename = entry.file_name();
        if matches!(filename.to_str(), Some("foundry.toml")) {
            return true;
        }
    }

    false
}
