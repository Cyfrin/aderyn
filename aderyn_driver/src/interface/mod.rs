pub mod json;
pub mod lsp;
pub mod markdown;
pub mod sarif;
pub mod tables;
pub mod util;

use std::{
    fs::{remove_file, File},
    io::{self, Result, Write},
    path::Path,
};

use aderyn_core::report::Report;

use crate::{driver::CliArgsOutputConfig, process::WorkspaceContextWrapper};

#[derive(Default)]
pub enum OutputInterface {
    Json,
    #[default]
    Markdown,
    Sarif,
}

pub fn output_interface_router(
    output_interface: OutputInterface,
    report: &Report,
    cx_wrapper: &WorkspaceContextWrapper,
    detectors_used: &[(String, String)],
    output_config: &CliArgsOutputConfig,
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

    println!("Detectors run, printing report.");

    let mut b: Box<dyn Write> = if output_config.stdout {
        Box::new(io::stdout())
    } else {
        Box::new(get_writer(&output_config.output)?)
    };

    match output_interface {
        OutputInterface::Json => {
            json::print_report(
                &mut b,
                report,
                &cx_wrapper.contexts,
                output_config.stdout,
                detectors_used,
            )?;
        }
        OutputInterface::Markdown => {
            markdown::print_report(
                &mut b,
                report,
                cx_wrapper,
                output_config.output.clone(),
                output_config.no_snippets,
            )?;
        }
        OutputInterface::Sarif => {
            sarif::print_report(&mut b, report, output_config.stdout)?;
        }
    }

    if !output_config.stdout {
        println!("Report printed to {}", output_config.output);
    }

    Ok(())
}
