use std::io::{Result, Write};

use crate::context::loader::ContextLoader;

use super::report::{Issue, Report};

pub trait ReportPrinter {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        loader: &ContextLoader,
    ) -> Result<()>;
    fn print_issue<W: Write>(&self, writer: W, issue: &Issue, loader: &ContextLoader)
        -> Result<()>;
}

pub struct MarkdownReportPrinter;

impl ReportPrinter for MarkdownReportPrinter {
    fn print_report<W: Write>(
        &self,
        mut writer: W,
        report: &Report,
        loader: &ContextLoader,
    ) -> Result<()> {
        writeln!(writer, "# Critical Issues")?;
        for issue in &report.criticals {
            self.print_issue(&mut writer, issue, loader)?;
        }
        writeln!(writer, "# High Issues")?;
        for issue in &report.highs {
            self.print_issue(&mut writer, issue, loader)?;
        }
        writeln!(writer, "# Medium Issues")?;
        for issue in &report.mediums {
            self.print_issue(&mut writer, issue, loader)?;
        }
        writeln!(writer, "# Low Issues")?;
        for issue in &report.lows {
            self.print_issue(&mut writer, issue, loader)?;
        }
        writeln!(writer, "# NC Issues")?;
        for issue in &report.ncs {
            self.print_issue(&mut writer, issue, loader)?;
        }
        writeln!(writer, "# Gas Issues")?;
        for issue in &report.gas {
            self.print_issue(&mut writer, issue, loader)?;
        }
        Ok(())
    }

    fn print_issue<W: Write>(
        &self,
        mut writer: W,
        issue: &Issue,
        loader: &ContextLoader,
    ) -> Result<()> {
        writeln!(writer, "## {}\n{}", issue.title, issue.description)?;
        for instance in &issue.instances {
            if let Some(node) = instance {
                let mut contract_path = "unknown";
                if let Some(source_unit_contract_path) =
                    loader.get_source_unit_contract_path_from(node)
                {
                    contract_path = source_unit_contract_path;
                }
                let mut source_location = "unknown";
                if let Some(src) = node.src() {
                    source_location = src;
                }
                writeln!(writer, "- Found in {}: {}", contract_path, source_location)?;
            }
        }
        Ok(())
    }
}
