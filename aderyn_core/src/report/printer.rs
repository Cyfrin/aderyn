use std::io::{Result, Write};

use crate::context::loader::ContextLoader;

use super::reporter::Report;

pub trait ReportPrinter<T> {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        loader: &ContextLoader,
    ) -> Result<T>;
}
