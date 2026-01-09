use aderyn_core::context::workspace::WorkspaceContext;
use std::error::Error;

use crate::{
    driver::CliArgsOutputConfig,
    interface::{OutputInterface, lsp::LspReport, output_interface_router, tables},
    process::WorkspaceContextWrapper,
};
use aderyn_core::report::*;

pub fn run_detector_mode(
    cx_wrapper: &WorkspaceContextWrapper,
    output_config: &CliArgsOutputConfig,
) -> Result<(), Box<dyn Error>> {
    println!("Running {} detectors", cx_wrapper.detectors.len());

    let detectors_used = &cx_wrapper
        .detectors
        .iter()
        .map(|d| (d.name(), d.severity().to_string()))
        .collect::<Vec<_>>();

    let detectors = cx_wrapper.detectors.iter().map(|d| d.skeletal_clone()).collect();
    let report = detect_issues(&cx_wrapper.contexts, &cx_wrapper.root_path, detectors)?;
    let output_file_path = output_config.output.clone();

    let output_interface = if output_file_path.ends_with(".json") {
        OutputInterface::Json
    } else if output_file_path.ends_with(".sarif") {
        OutputInterface::Sarif
    } else if output_file_path.ends_with(".md") {
        OutputInterface::Markdown
    } else {
        println!("Warning: Output file extension is unrecognized. Reverting to markdown..");
        OutputInterface::default()
    };

    output_interface_router(output_interface, &report, &cx_wrapper, detectors_used, output_config)?;

    Ok(())
}

pub fn run_lsp_mode(ctx_wrapper: &WorkspaceContextWrapper) -> Option<LspReport> {
    let detectors = ctx_wrapper.detectors.iter().map(|d| d.skeletal_clone()).collect();
    let (root_rel_path, contexts) = (&ctx_wrapper.root_path, &ctx_wrapper.contexts);
    match detect_issues(contexts, root_rel_path, detectors) {
        Ok(report) => {
            let (high_issues, low_issues) = report.detailed_issues(contexts);
            Some(LspReport::from(low_issues, high_issues, root_rel_path))
        }
        Err(_) => None,
    }
}

pub fn run_auditor_mode(contexts: &[WorkspaceContext]) -> Result<(), Box<dyn Error>> {
    tables::print_audit_info_tables(contexts)
}
