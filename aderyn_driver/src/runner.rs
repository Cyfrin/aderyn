use aderyn_core::{context::workspace_context::WorkspaceContext, detect::detector::IssueDetector};
use std::{
    error::Error,
    fs::{remove_file, File},
    io,
    path::{Path, PathBuf},
};

use crate::interface::{lsp::LspReport, ReportPrinter};
use aderyn_core::report::*;

#[allow(clippy::too_many_arguments)]
pub fn run_detector_mode<T>(
    contexts: &[WorkspaceContext],
    output_file_path: String,
    reporter: T,
    root_rel_path: PathBuf,
    no_snippets: bool,
    stdout: bool,
    detectors: Vec<Box<dyn IssueDetector>>,
) -> Result<(), Box<dyn Error>>
where
    T: ReportPrinter<()>,
{
    println!("Running {} detectors", detectors.len());

    let detectors_used =
        &detectors.iter().map(|d| (d.name(), d.severity().to_string())).collect::<Vec<_>>();

    let report = get_report(contexts, &root_rel_path, detectors)?;

    println!("Detectors run, processing found issues");
    println!("Found issues processed. Printing report");

    let get_writer = |filename: &str| -> io::Result<File> {
        let file_path = Path::new(filename);
        if let Some(parent_dir) = file_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        if Path::new(filename).exists() {
            remove_file(filename)?; // If file exists, delete it
        }
        File::create(filename)
    };

    if !stdout {
        reporter.print_report(
            get_writer(&output_file_path)?,
            &report,
            contexts,
            root_rel_path,
            Some(output_file_path.clone()),
            no_snippets,
            stdout,
            detectors_used,
        )?;
        println!("Report printed to {}", output_file_path);
    } else {
        reporter.print_report(
            io::stdout(),
            &report,
            contexts,
            root_rel_path,
            Some(output_file_path.clone()),
            no_snippets,
            stdout,
            detectors_used,
        )?;
    }
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
