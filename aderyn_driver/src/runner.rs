use aderyn_core::{context::workspace_context::WorkspaceContext, detect::detector::IssueDetector};
use std::{error::Error, path::PathBuf};

use crate::{
    driver::CliArgsOutputConfig,
    interface::{lsp::LspReport, output_interface_router, OutputInterface},
};
use aderyn_core::report::*;

pub fn run_detector_mode(
    contexts: &[WorkspaceContext],
    root_rel_path: PathBuf,
    detectors: Vec<Box<dyn IssueDetector>>,
    output_config: &CliArgsOutputConfig,
) -> Result<(), Box<dyn Error>> {
    println!("Running {} detectors", detectors.len());

    let detectors_used =
        &detectors.iter().map(|d| (d.name(), d.severity().to_string())).collect::<Vec<_>>();

    let report = get_report(contexts, &root_rel_path, detectors)?;
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

    output_interface_router(
        output_interface,
        &report,
        contexts,
        root_rel_path,
        detectors_used,
        output_config,
    )?;

    Ok(())
}

pub fn run_lsp_mode(
    contexts: &[WorkspaceContext],
    root_rel_path: PathBuf,
    detectors: Vec<Box<dyn IssueDetector>>,
) -> Option<LspReport> {
    match get_report(contexts, &root_rel_path, detectors) {
        Ok(report) => {
            let (high_issues, low_issues) = report.detailed_issues(contexts);
            Some(LspReport::from(low_issues, high_issues, &root_rel_path))
        }
        Err(_) => None,
    }
}

pub fn run_auditor_mode(contexts: &[WorkspaceContext]) -> Result<(), Box<dyn Error>> {
    // TODO: Port logic from aderyn-core to here
    aderyn_core::run_auditor_mode(contexts)
}
