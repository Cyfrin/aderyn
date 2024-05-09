use std::{
    io::{self, Result, Write},
    path::PathBuf,
};

use crate::context::workspace_context::WorkspaceContext;
use serde::Serialize;
use serde_sarif::{
    sarif::ReportingDescriptor, sarif::Result as SarifResult, sarif::Run, sarif::Sarif, sarif::Tool,
};

use super::{
    printer::ReportPrinter, reporter::Report, FilesDetails, FilesSummary, HighIssues, LowIssues,
};

#[derive(Serialize)]
pub struct SarifContent {
    version: String,
    runs: Vec<Run>,
}

pub struct SarifPrinter;

impl ReportPrinter<()> for SarifPrinter {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        context: &WorkspaceContext,
        _: PathBuf,
        _: Option<String>,
        _: bool,
        stdout: bool,
        detectors_used: &[(String, String)],
    ) -> Result<()> {
        // TODO
        Ok(())
    }
}

fn create_sarif_results(report: &Report, context: &WorkspaceContext) -> Vec<SarifResult> {
    // Convert your report and context into SARIF results here
    vec![]
}
