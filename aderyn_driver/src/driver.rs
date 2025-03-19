use crate::{
    compile, config_helpers::derive_from_aderyn_toml, ensure_valid_root_path, lsp_report::LspReport,
};
use aderyn_core::{
    context::{
        graph::{Transpose, WorkspaceCallGraph},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{get_all_issue_detectors, IssueDetector, IssueSeverity},
    fscloc, get_report,
    report::{
        json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter,
        sarif_printer::SarifPrinter,
    },
    run,
};
use field_access::FieldAccess;
use std::{collections::HashMap, error::Error, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, FieldAccess)]
pub struct Args {
    pub root: String,
    pub output: String,
    pub src: Option<String>,
    pub path_excludes: Option<Vec<String>>,
    pub path_includes: Option<Vec<String>>,
    pub no_snippets: bool,
    pub skip_cloc: bool,
    pub skip_update_check: bool,
    pub stdout: bool,
    pub auditor_mode: bool,
    pub highs_only: bool,
    pub lsp: bool,
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

pub fn drive_and_get_results(args: Args) -> Arc<Mutex<Option<LspReport>>> {
    let detectors = if args.highs_only {
        get_all_issue_detectors()
            .into_iter()
            .filter(|d| d.severity() == IssueSeverity::High)
            .collect::<Vec<_>>()
    } else {
        get_all_issue_detectors()
    };

    let cx_wrapper = make_context(&args);
    let root_rel_path = cx_wrapper.root_path;
    let file_contents = cx_wrapper
        .contexts
        .iter()
        .flat_map(|context| context.source_units())
        .map(|source_unit| {
            (
                source_unit.absolute_path.as_ref().unwrap().to_owned(),
                source_unit.source.as_ref().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

    if let Ok(report) = get_report(&cx_wrapper.contexts, &root_rel_path, detectors) {
        let high_issues = report.high_issues(&file_contents);
        let low_issues = report.low_issues(&file_contents);
        let lsp_result = LspReport::from(low_issues, high_issues, &root_rel_path);
        return Arc::new(tokio::sync::Mutex::new(Some(lsp_result)));
    }

    Arc::new(tokio::sync::Mutex::new(None))
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
    if !args.lsp && !args.output.ends_with(".json") && !args.output.ends_with(".md") {
        eprintln!("Warning: output file lacks the \".md\" or \".json\" extension in its filename.");
    }

    let (root_path, src, exclude, include) = obtain_config_values(args).unwrap();

    let absolute_root_path = &ensure_valid_root_path(&root_path);

    let mut contexts: Vec<WorkspaceContext> =
        compile::project(&root_path, &src, &exclude, &include, args.lsp);

    if !args.lsp && contexts.iter().all(|c| c.src_filepaths.is_empty()) {
        eprintln!("No solidity files found in given scope!");
        std::process::exit(1);
    }

    for context in contexts.iter_mut() {
        let stats = fscloc::engine::count_lines_of_code_and_collect_line_numbers_to_ignore(
            absolute_root_path.as_path(),
            &context.src_filepaths,
            args.skip_cloc,
        );
        let sloc_stats = stats
            .lock()
            .unwrap()
            .iter()
            .map(|(key, value)| (key.to_owned(), value.code))
            .collect::<HashMap<_, _>>();

        let ignore_line_stats = stats
            .lock()
            .unwrap()
            .iter()
            .map(|(key, value)| (key.to_owned(), value.ignore_lines.clone()))
            .collect::<HashMap<_, _>>();

        // dbg!(&stats);
        context.set_sloc_stats(sloc_stats);
        context.set_ignore_lines_stats(ignore_line_stats);

        let inward_callgraph = WorkspaceCallGraph::from_context(context).unwrap();
        let outward_callgraph =
            WorkspaceCallGraph { raw_callgraph: inward_callgraph.raw_callgraph.reverse() };
        context.inward_callgraph = Some(inward_callgraph);
        context.outward_callgraph = Some(outward_callgraph);
    }
    // Using the source path, calculate the sloc

    WorkspaceContextWrapper { contexts, root_path }
}

/// Supplement the arguments with values from aderyn.toml and foundry.toml
#[allow(clippy::type_complexity)]
fn obtain_config_values(
    args: &Args,
) -> Result<(PathBuf, Option<String>, Option<Vec<String>>, Option<Vec<String>>), Box<dyn Error>> {
    let mut root_path = PathBuf::from(&args.root);

    let mut local_src = args.src.clone();
    let mut local_exclude = args.path_excludes.clone();
    let mut local_include = args.path_includes.clone();

    let aderyn_path = root_path.join("aderyn.toml");
    // Process aderyn.toml if it exists
    if aderyn_path.exists() {
        (root_path, local_src, local_exclude, local_include) =
            derive_from_aderyn_toml(&root_path, &local_src, &local_exclude, &local_include);
    }

    Ok((root_path, local_src, local_exclude, local_include))
}
