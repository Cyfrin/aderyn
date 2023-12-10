use std::io::{Result, Write};

use crate::context::loader::ContextLoader;
use serde::Serialize;

use super::{printer::ReportPrinter, reporter::Report};

#[derive(Serialize)]
pub struct JsonContent {
    issue_count: IssueCount,
}

#[derive(Serialize)]
pub struct IssueCount {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub nc: usize,
}

pub struct JsonPrinter;

impl ReportPrinter<()> for JsonPrinter {
    fn print_report<W: Write>(&self, writer: W, report: &Report, _: &ContextLoader) -> Result<()> {
        let content = JsonContent {
            issue_count: IssueCount {
                critical: report.criticals.len(),
                high: report.highs.len(),
                medium: report.mediums.len(),
                low: report.lows.len(),
                nc: report.ncs.len(),
            },
        };
        let value = serde_json::to_value(content).unwrap();
        _ = serde_json::to_writer(writer, &value);
        Ok(())
    }
}
