use std::io::{Result, Write};

use crate::{ast::SourceUnit, context::loader::ContextLoader};

use super::reporter::{Issue, Report};

pub trait ReportPrinter {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        loader: &ContextLoader,
    ) -> Result<()>;
    fn print_table_of_contents<W: Write>(&self, writer: W, report: &Report) -> Result<()>;
    fn print_contract_summary<W: Write>(&self, writer: W, loader: &ContextLoader) -> Result<()>;
    fn print_issue<W: Write>(
        &self,
        writer: W,
        issue: &Issue,
        loader: &ContextLoader,
        severity: &str,
        number: i32,
    ) -> Result<()>;
}

pub struct MarkdownReportPrinter;

impl ReportPrinter for MarkdownReportPrinter {
    fn print_report<W: Write>(
        &self,
        mut writer: W,
        report: &Report,
        loader: &ContextLoader,
    ) -> Result<()> {
        self.print_table_of_contents(&mut writer, report)?;
        self.print_contract_summary(&mut writer, loader)?;
        let mut counter = 0;
        if !report.criticals.is_empty() {
            writeln!(writer, "# Critical Issues\n")?;
            for issue in &report.criticals {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "C", counter)?;
            }
        }
        if !report.highs.is_empty() {
            writeln!(writer, "# High Issues\n")?;
            counter = 0;
            for issue in &report.highs {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "H", counter)?;
            }
        }
        if !report.mediums.is_empty() {
            writeln!(writer, "# Medium Issues\n")?;
            counter = 0;
            for issue in &report.mediums {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "M", counter)?;
            }
        }
        if !report.lows.is_empty() {
            writeln!(writer, "# Low Issues\n")?;
            counter = 0;
            for issue in &report.lows {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "L", counter)?;
            }
        }
        if !report.ncs.is_empty() {
            writeln!(writer, "# NC Issues\n")?;
            counter = 0;
            for issue in &report.ncs {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "NC", counter)?;
            }
        }
        Ok(())
    }

    fn print_contract_summary<W: Write>(
        &self,
        mut writer: W,
        loader: &ContextLoader,
    ) -> Result<()> {
        writeln!(writer, "# Contract Summary\n")?;
        writeln!(writer, "Contracts analyzed:\n")?;
        for source_unit in loader.get_source_units() {
            writeln!(
                writer,
                "- {:?}",
                source_unit.absolute_path.as_ref().unwrap()
            )?;
        }
        writeln!(writer, "\n")?; // Add an extra newline for spacing
        Ok(())
    }

    fn print_table_of_contents<W: Write>(&self, mut writer: W, report: &Report) -> Result<()> {
        writeln!(writer, "# Table of Contents\n")?;
        writeln!(writer, "- [Contract Summary](#contract-summary)")?;
        if !report.criticals.is_empty() {
            writeln!(writer, "- [Critical Issues](#critical-issues)")?;
            for (index, issue) in report.criticals.iter().enumerate() {
                writeln!(
                    writer,
                    "  - [C-{}: {}](#C-{})",
                    index + 1,
                    issue.title,
                    index + 1
                )?;
            }
        }
        if !report.highs.is_empty() {
            writeln!(writer, "- [High Issues](#high-issues)")?;
            for (index, issue) in report.highs.iter().enumerate() {
                writeln!(
                    writer,
                    "  - [H-{}: {}](#H-{})",
                    index + 1,
                    issue.title,
                    index + 1
                )?;
            }
        }
        if !report.mediums.is_empty() {
            writeln!(writer, "- [Medium Issues](#medium-issues)")?;
            for (index, issue) in report.mediums.iter().enumerate() {
                writeln!(
                    writer,
                    "  - [M-{}: {}](#M-{})",
                    index + 1,
                    issue.title,
                    index + 1
                )?;
            }
        }
        if !report.lows.is_empty() {
            writeln!(writer, "- [Low Issues](#low-issues)")?;
            for (index, issue) in report.lows.iter().enumerate() {
                writeln!(
                    writer,
                    "  - [L-{}: {}](#L-{})",
                    index + 1,
                    issue.title,
                    index + 1
                )?;
            }
        }
        if !report.ncs.is_empty() {
            writeln!(writer, "- [NC Issues](#nc-issues)")?;
            for (index, issue) in report.ncs.iter().enumerate() {
                writeln!(
                    writer,
                    "  - [NC-{}: {}](#NC-{})",
                    index + 1,
                    issue.title,
                    index + 1
                )?;
            }
        }
        writeln!(writer, "\n")?; // Add an extra newline for spacing
        Ok(())
    }

    fn print_issue<W: Write>(
        &self,
        mut writer: W,
        issue: &Issue,
        loader: &ContextLoader,
        severity: &str,
        number: i32,
    ) -> Result<()> {
        writeln!(
            writer,
            "<a name=\"{}-{}\"></a>\n## {}-{}: {}\n\n{}\n", // <a name> is the anchor for the issue title
            severity, number, severity, number, issue.title, issue.description
        )?;
        for node in issue.instances.iter().flatten() {
            let mut contract_path = "unknown";
            let source_unit: &SourceUnit = loader.get_source_unit_from_child_node(node).unwrap();
            if let Some(path) = source_unit.absolute_path.as_ref() {
                contract_path = path;
            }
            let mut line_number = 0;
            let mut source_location = "unknown";
            if let Some(src) = node.src() {
                source_location = src;
                line_number = source_unit.source_line(src).unwrap();
            }
            writeln!(
                writer,
                "- Found in {}: {}. Line: {}",
                contract_path, source_location, line_number
            )?;
        }
        writeln!(writer, "\n")?; // Add an extra newline for spacing
        Ok(())
    }
}
