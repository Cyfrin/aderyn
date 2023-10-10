use std::io::{Write, Result};

use super::report::{Report, Issue};

pub trait ReportPrinter {
    fn print_report<W: Write>(&self, writer: W, report: &Report) -> Result<()>;
    fn print_issue<W: Write>(&self, writer: W, issue: &Issue) -> Result<()>;
}

pub struct MarkdownReportPrinter;

impl ReportPrinter for MarkdownReportPrinter {
    fn print_report<W: Write>(&self, mut writer: W, report: &Report) -> Result<()> {
        writeln!(writer, "# Critical Issues")?;
        for issue in &report.criticals {
            self.print_issue(&mut writer, issue)?;
        }
        writeln!(writer, "# High Issues")?;
        for issue in &report.highs {
            self.print_issue(&mut writer, issue)?;
        }
        writeln!(writer, "# Medium Issues")?;
        for issue in &report.mediums {
            self.print_issue(&mut writer, issue)?;
        }
        writeln!(writer, "# Low Issues")?;
        for issue in &report.lows {
            self.print_issue(&mut writer, issue)?;
        }
        writeln!(writer, "# NC Issues")?;
        for issue in &report.ncs {
            self.print_issue(&mut writer, issue)?;
        }
        writeln!(writer, "# Gas Issues")?;
        for issue in &report.gas {
            self.print_issue(&mut writer, issue)?;
        }
        Ok(())
    }

    fn print_issue<W: Write>(&self, mut writer: W, issue: &Issue) -> Result<()> {
        writeln!(writer, "## {}\n{}", issue.title, issue.description)?;
        for instance in &issue.instances {
            if let Some(node) = instance {
                if let Some(src) = node.src() {
                    writeln!(writer, "- Found in source: {}", src)?;
                } else {
                    writeln!(writer, "- Found in an unknown source")?;
                }
            }
        }
        Ok(())
    }
}
