use std::{
    io::{Result, Write},
    path::PathBuf,
};

use crate::context::loader::ContextLoader;

use super::reporter::Report;

pub trait ReportPrinter<T> {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        loader: &ContextLoader,
        root_abs_path: PathBuf,
    ) -> Result<T>;
}
