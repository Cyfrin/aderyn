use std::io::{Result, Write};

use aderyn_core::{context::workspace::WorkspaceContext, report::*};
use serde::Serialize;

use super::util::files_details;

#[derive(Serialize)]
pub struct JsonContent {
    files_summary: FilesSummary,
    files_details: FilesDetails,
    issue_count: IssueCount,
    high_issues: HighIssues,
    low_issues: LowIssues,
    detectors_used: Vec<String>,
}

pub fn print_report(
    writer: &mut Box<dyn Write>,
    report: &Report,
    contexts: &[WorkspaceContext],
    stdout: bool,
    detectors_used: &[(String, String)],
) -> Result<()> {
    let mut all_files_details = FilesDetails::default();
    for context in contexts {
        all_files_details = all_files_details + &files_details(context);
    }

    all_files_details.files_details.sort_by(|a, b| a.file_path.cmp(&b.file_path));

    let mut all_files_summary = FilesSummary::default();
    for details in &all_files_details.files_details {
        all_files_summary.total_sloc += details.n_sloc;
        all_files_summary.total_source_units += 1;
    }

    let detectors_used_names: Vec<_> = detectors_used.iter().map(|x| x.0.clone()).collect();
    let (high_issues, low_issues) = report.detailed_issues(contexts);

    let content = JsonContent {
        files_summary: all_files_summary,
        files_details: all_files_details,
        issue_count: report.issue_count(),
        high_issues,
        low_issues,
        detectors_used: detectors_used_names,
    };
    let value = serde_json::to_value(content).unwrap();

    if stdout {
        println!("STDOUT START");
    }

    serde_json::to_writer_pretty(writer, &value)?;

    if stdout {
        println!("STDOUT END");
    }

    Ok(())
}
