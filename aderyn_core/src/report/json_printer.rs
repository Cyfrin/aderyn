use std::{
    io::{Result, Write},
    path::PathBuf,
};

use crate::context::loader::ContextLoader;
use serde::Serialize;

use super::{
    printer::ReportPrinter, reporter::Report, CriticalIssues, FilesDetails, FilesSummary,
    HighIssues, IssueCount, LowIssues, MediumIssues, NcIssues,
};

#[derive(Serialize)]
pub struct JsonContent {
    files_summary: FilesSummary,
    files_details: FilesDetails,
    issue_count: IssueCount,
    critical_issues: CriticalIssues,
    high_issues: HighIssues,
    medium_issue: MediumIssues,
    low_issues: LowIssues,
    nc_issues: NcIssues,
}

pub struct JsonPrinter;

/**
 * JSON should mimick MD
    {
        "files_summary": {...},
        "files_details": {...},
        "issue_summary": {...},
        "critical_issues": {...},
        "high_issues": {...},
    ...
    }
*/

impl ReportPrinter<()> for JsonPrinter {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        loader: &ContextLoader,
        _: PathBuf,
        _: Option<String>,
        _: bool,
    ) -> Result<()> {
        let content = JsonContent {
            files_summary: loader.files_summary(),
            files_details: loader.files_details(),
            issue_count: report.issue_count(),
            critical_issues: report.critical_issues(),
            high_issues: report.high_issues(),
            medium_issue: report.medium_issues(),
            low_issues: report.low_issues(),
            nc_issues: report.nc_issues(),
        };
        let value = serde_json::to_value(content).unwrap();
        _ = serde_json::to_writer_pretty(writer, &value);
        Ok(())
    }
}
