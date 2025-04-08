pub mod json;
pub mod lsp;
pub mod markdown;
pub mod sarif;
pub mod util;

use std::{
    io::{Result, Write},
    path::PathBuf,
};

use aderyn_core::{context::workspace_context::WorkspaceContext, report::Report};

pub enum OutputInterface {
    Json,
    Markdown,
    Sarif,
}

#[allow(clippy::borrowed_box)]
pub trait ReportPrinter<T> {
    #[allow(clippy::too_many_arguments)]
    fn print_report(
        &self,
        writer: &mut Box<dyn Write>,
        report: &Report,
        contexts: &[WorkspaceContext],
        root_rel_path: PathBuf,
        output_rel_path: Option<String>, /* you writer 'W' may or may not be writing a file. Eg:
                                          * it can simply consume and forget :P */
        no_snippets: bool,
        stdout: bool,
        detectors_used: &[(String, String)],
    ) -> Result<T>;
}
