use std::{
    io::{Result, Write},
    path::PathBuf,
};

use crate::context::workspace_context::WorkspaceContext;

use super::reporter::Report;

pub trait ReportPrinter<T> {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        context: &WorkspaceContext,
        root_rel_path: PathBuf,
        output_rel_path: Option<String>, // you writer 'W' may or may not be writing a file. Eg: it can simply consume and forget :P
        no_snippets: bool,
    ) -> Result<T>;
}
