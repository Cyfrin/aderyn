use std::{
    io::{self, Result, Write},
    path::PathBuf,
};

use crate::context::workspace_context::WorkspaceContext;
use serde::Serialize;

use super::{
    printer::ReportPrinter, reporter::Report, FilesDetails, FilesSummary, HighIssues, IssueCount,
    LowIssues,
};

#[derive(Serialize)]
pub struct JsonContent {
    files_summary: FilesSummary,
    files_details: FilesDetails,
    issue_count: IssueCount,
    high_issues: HighIssues,
    low_issues: LowIssues,
    detectors_used: Vec<String>,
}

pub struct JsonPrinter;

/**
 * JSON should mimick MD
    {
        "files_summary": {...},
        "files_details": {...},
        "issue_summary": {...},
        "high_issues": {...},
    ...
    }
*/

impl ReportPrinter<()> for JsonPrinter {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        contexts: &[WorkspaceContext],
        _: PathBuf,
        _: Option<String>,
        _: bool,
        stdout: bool,
        detectors_used: &[(String, String)],
    ) -> Result<()> {
        let mut all_files_details = FilesDetails::default();
        for context in contexts {
            all_files_details = all_files_details + &context.files_details();
        }

        all_files_details
            .files_details
            .sort_by(|a, b| a.file_path.cmp(&b.file_path));

        let mut all_files_summary = FilesSummary::default();
        for details in &all_files_details.files_details {
            all_files_summary.total_sloc += details.n_sloc;
            all_files_summary.total_source_units += 1;
        }

        let detectors_used_names: Vec<_> = detectors_used.iter().map(|x| x.0.clone()).collect();

        let content = JsonContent {
            files_summary: all_files_summary,
            files_details: all_files_details,
            issue_count: report.issue_count(),
            high_issues: report.high_issues(),
            low_issues: report.low_issues(),
            detectors_used: detectors_used_names,
        };
        let value = serde_json::to_value(content).unwrap();
        if stdout {
            println!("STDOUT START");
            let _ = serde_json::to_writer_pretty(io::stdout(), &value);
            println!("STDOUT END");
            return Ok(());
        }
        _ = serde_json::to_writer_pretty(writer, &value);
        Ok(())
    }
}
