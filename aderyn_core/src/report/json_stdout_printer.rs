use std::{
    io::{self, Result, Write},
    path::PathBuf,
};

use crate::context::workspace_context::WorkspaceContext;
use serde::Serialize;

use super::{
    printer::ReportPrinter, reporter::Report, CriticalIssues, FilesDetails, HighIssues, LowIssues,
    MediumIssues, NcIssues,
};

#[derive(Serialize)]
pub struct JsonStdoutContent {
    files_details: FilesDetails,
    critical_issues: CriticalIssues,
    high_issues: HighIssues,
    medium_issues: MediumIssues,
    low_issues: LowIssues,
    nc_issues: NcIssues,
    detectors_used: Vec<String>,
}

pub struct JsonStdoutPrinter;

impl ReportPrinter<()> for JsonStdoutPrinter {
    fn print_report<W: Write>(
        &self,
        _: W,
        report: &Report,
        context: &WorkspaceContext,
        _: PathBuf,
        _: Option<String>,
        _: bool,
        detectors_used: &[(String, String)],
    ) -> Result<()> {
        let detectors_used_names: Vec<_> = detectors_used.iter().map(|x| x.0.clone()).collect();

        let content = JsonStdoutContent {
            files_details: context.files_details(),
            critical_issues: report.critical_issues(),
            high_issues: report.high_issues(),
            medium_issues: report.medium_issues(),
            low_issues: report.low_issues(),
            nc_issues: report.nc_issues(),
            detectors_used: detectors_used_names,
        };
        let value = serde_json::to_value(content).unwrap();
        _ = serde_json::to_writer_pretty(io::stdout(), &value);
        Ok(())
    }
}
