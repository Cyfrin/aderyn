pub mod json;
pub mod lsp;
pub mod markdown;
pub mod sarif;
pub mod util;

use std::{
    fs::{remove_file, File},
    io::{self, Result, Write},
    path::{Path, PathBuf},
};

use aderyn_core::{context::workspace_context::WorkspaceContext, report::Report};

use crate::driver::Args;

pub enum OutputInterface {
    Json,
    Markdown,
    Sarif,
}

pub fn output_interface_router(
    output_interface: OutputInterface,
    report: &Report,
    contexts: &[WorkspaceContext],
    root_rel_path: PathBuf,
    output_file_path: String, /* you writer 'W' may or may not be writing a file. Eg:
                               * it can simply consume and forget :P */
    detectors_used: &[(String, String)],
    args: &Args,
) -> Result<()> {
    let get_writer = |filename: &str| -> io::Result<File> {
        let file_path = Path::new(filename);
        if let Some(parent_dir) = file_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        if Path::new(filename).exists() {
            remove_file(filename)?; // If file exists, delete it
        }
        File::create(filename)
    };

    let mut b: Box<dyn Write> =
        if args.stdout { Box::new(io::stdout()) } else { Box::new(get_writer(&output_file_path)?) };

    match output_interface {
        OutputInterface::Json => {
            json::print_report(&mut b, &report, contexts, args.stdout, detectors_used)?;
        }
        OutputInterface::Markdown => {
            markdown::print_report(
                &mut b,
                &report,
                contexts,
                root_rel_path,
                output_file_path.clone(),
                args.no_snippets,
            )?;
        }
        OutputInterface::Sarif => {
            sarif::print_report(&mut b, &report, args.stdout)?;
        }
    }

    if !args.stdout {
        println!("Report printed to {}", output_file_path);
    }

    Ok(())
}
