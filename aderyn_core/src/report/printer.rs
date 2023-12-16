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
        root_rel_path: PathBuf,
        output_rel_path: Option<String>, // you writer 'W' may or may not be writing a file. Eg: it can simply consume and forget :P
    ) -> Result<T>;
}
