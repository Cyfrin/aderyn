use crate::{
    config_helpers::{append_from_foundry_toml, derive_from_aderyn_toml},
    ensure_valid_root_path, process_auto,
};
use aderyn_core::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{get_all_issue_detectors, IssueDetector, IssueSeverity},
    fscloc,
    report::{
        json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter,
        sarif_printer::SarifPrinter,
    },
    run,
};
use std::{error::Error, path::PathBuf};

#[derive(Clone)]
pub struct Args {
    pub root: String,
    pub output: String,
    pub src: Option<Vec<String>>,
    pub path_excludes: Option<Vec<String>>,
    pub path_includes: Option<Vec<String>>,
    pub no_snippets: bool,
    pub skip_build: bool,
    pub skip_cloc: bool,
    pub skip_update_check: bool,
    pub stdout: bool,
    pub auditor_mode: bool,
    pub highs_only: bool,
}

pub fn drive(args: Args) {
    let detectors = if args.highs_only {
        get_all_issue_detectors()
            .into_iter()
            .filter(|d| d.severity() == IssueSeverity::High)
            .collect::<Vec<_>>()
    } else {
        get_all_issue_detectors()
    };
    drive_with(args, detectors);
}

pub fn drive_with(args: Args, detectors_list: Vec<Box<dyn IssueDetector>>) {
    let output = args.output.clone();
    let cx_wrapper = make_context(&args);
    let root_rel_path = cx_wrapper.root_path;

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
            detectors_list,
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
            detectors_list,
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
            detectors_list,
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
    pub root_path: PathBuf,
}

fn make_context(args: &Args) -> WorkspaceContextWrapper {
    if !args.output.ends_with(".json") && !args.output.ends_with(".md") {
        eprintln!("Warning: output file lacks the \".md\" or \".json\" extension in its filename.");
    }

    let (root_path, src, exclude, remappings, include) = obtain_config_values(args).unwrap();

    let absolute_root_path = &ensure_valid_root_path(&root_path);
    println!(
        "Root: {:?}, Src: {:?}, Include: {:?}, Exclude: {:?}",
        absolute_root_path, src, include, exclude
    );

    let mut contexts: Vec<WorkspaceContext> =
        process_auto::with_project_root_at(&root_path, &src, &exclude, &remappings, &include);

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

    WorkspaceContextWrapper {
        contexts,
        root_path,
    }
}

/// Supplement the arguments with values from aderyn.toml and foundry.toml
#[allow(clippy::type_complexity)]
fn obtain_config_values(
    args: &Args,
) -> Result<
    (
        PathBuf,
        Option<Vec<String>>,
        Option<Vec<String>>,
        Option<Vec<String>>,
        Option<Vec<String>>,
    ),
    Box<dyn Error>,
> {
    let mut root_path = PathBuf::from(&args.root);

    let mut local_src = args.src.clone();
    let mut local_exclude = args.path_excludes.clone();
    let mut local_remappings = None;
    let mut local_include = args.path_includes.clone();

    let aderyn_path = root_path.join("aderyn.toml");
    // Process aderyn.toml if it exists
    if aderyn_path.exists() {
        (
            root_path,
            local_src,
            local_exclude,
            local_remappings,
            local_include,
        ) = derive_from_aderyn_toml(
            &root_path,
            &local_src,
            &local_exclude,
            &local_remappings,
            &local_include,
        );
    }

    let foundry_path = root_path.join("foundry.toml");
    // Process foundry.toml if it exists
    if foundry_path.exists() {
        (local_src, local_exclude, local_remappings) =
            append_from_foundry_toml(&root_path, &local_src, &local_exclude, &local_remappings);
    }

    Ok((
        root_path,
        local_src,
        local_exclude,
        local_remappings,
        local_include,
    ))
}
