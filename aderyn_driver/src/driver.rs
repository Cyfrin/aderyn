use crate::{lsp_report::LspReport, preprocess::make_context};
use aderyn_core::{
    detect::detector::{get_all_issue_detectors, IssueDetector, IssueSeverity},
    get_report,
    report::{
        json_printer::JsonPrinter, markdown_printer::MarkdownReportPrinter,
        sarif_printer::SarifPrinter,
    },
    run,
};
use field_access::FieldAccess;
use std::{collections::HashMap, sync::Arc};
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

/// One way pipeline. Used by CLI
pub fn drive(args: Args) {
    // Choose the detectors
    let detectors = detector_list(&args);

    let run_pipeline = || -> Result<(), Box<dyn std::error::Error>> {
        let output = args.output.clone();
        let cx_wrapper = make_context(&args).unwrap_or_else(|e| {
            eprintln!("Error making context: {}", e);
            std::process::exit(1);
        });
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
                detectors,
            )?;
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
            )?;
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
            )?;
        }
        Ok(())
    };

    // Kick-off
    run_pipeline().unwrap_or_else(|e| {
        eprintln!("Error driving aderyn: {}", e);
        std::process::exit(1);
    });
}

/// Drives and returns results. Used by LSP
pub fn drive_and_get_results(args: Args) -> Arc<Mutex<Option<LspReport>>> {
    // Choose the detectors
    let detectors = detector_list(&args);

    let ctx_wrapper = match make_context(&args) {
        Ok(ctx_wrapper) => ctx_wrapper,
        Err(_) => {
            return Arc::new(tokio::sync::Mutex::new(None));
        }
    };

    let root_rel_path = ctx_wrapper.root_path;
    let file_contents = ctx_wrapper
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

    let lsp_report = match get_report(&ctx_wrapper.contexts, &root_rel_path, detectors) {
        Ok(report) => {
            let high_issues = report.high_issues(&file_contents);
            let low_issues = report.low_issues(&file_contents);
            Some(LspReport::from(low_issues, high_issues, &root_rel_path))
        }
        Err(_) => None,
    };

    Arc::new(tokio::sync::Mutex::new(lsp_report))
}

fn detector_list(args: &Args) -> Vec<Box<dyn IssueDetector>> {
    get_all_issue_detectors()
        .into_iter()
        .filter(|d| !args.highs_only || d.severity() == IssueSeverity::High)
        .collect()
}
