use std::{
    io::{Result, Write},
    path::PathBuf,
};

use super::reporter::Report;
use crate::context::workspace_context::WorkspaceContext;

#[allow(clippy::borrowed_box)]
pub trait ReportPrinter<T> {
    #[allow(clippy::too_many_arguments)]
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        contexts: &[WorkspaceContext],
        root_rel_path: PathBuf,
        output_rel_path: Option<String>, // you writer 'W' may or may not be writing a file. Eg: it can simply consume and forget :P
        no_snippets: bool,
        stdout: bool,
        detectors_used: &[(String, String)],
    ) -> Result<T>;
}
