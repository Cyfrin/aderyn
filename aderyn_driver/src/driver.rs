use crate::{ensure_valid_root_path, process_auto};
use aderyn_core::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{get_all_issue_detectors, IssueDetector},
    fscloc,
    report::{json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter},
    run,
};
use std::path::PathBuf;

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

    let mut contexts =
        process_auto::with_project_root_at(&root_path, &args.scope, &args.exclude, &args.src);

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
