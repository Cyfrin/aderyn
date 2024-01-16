pub mod ast;
pub mod context;
pub mod detect;
pub mod framework;
pub mod fscloc;
pub mod report;
pub mod visitor;

use detect::detector::Detector;
use eyre::Result;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::{self};
use std::path::{Path, PathBuf};

use crate::context::loader::ContextLoader;
use crate::detect::detector::{get_all_detectors, IssueSeverity};
use crate::report::printer::ReportPrinter;
use crate::report::reporter::Report;
use crate::report::Issue;

pub fn run_with_printer<T>(
    context_loader: &ContextLoader,
    output_file_path: String,
    reporter: T,
    root_rel_path: PathBuf,
    no_snippets: bool,
) -> Result<(), Box<dyn Error>>
where
    T: ReportPrinter<()>,
{
    let detectors = get_all_detectors();
    run_with_printer_and_given_detectors(
        context_loader,
        output_file_path,
        reporter,
        root_rel_path,
        no_snippets,
        detectors,
    )
}

pub fn run_with_printer_and_given_detectors<T>(
    context_loader: &ContextLoader,
    output_file_path: String,
    reporter: T,
    root_rel_path: PathBuf,
    no_snippets: bool,
    detectors: Vec<Box<dyn Detector>>,
) -> Result<(), Box<dyn Error>>
where
    T: ReportPrinter<()>,
{
    println!("Get Detectors");

    println!("Running {} detectors", detectors.len());

    let mut report: Report = Report::default();
    for mut detector in detectors {
        if let Ok(found) = detector.detect(context_loader) {
            if found {
                let issue: Issue = Issue {
                    title: detector.title(),
                    description: detector.description(),
                    instances: detector.instances(),
                };
                match detector.severity() {
                    IssueSeverity::Critical => {
                        report.criticals.push(issue);
                    }
                    IssueSeverity::High => {
                        report.highs.push(issue);
                    }
                    IssueSeverity::Medium => {
                        report.mediums.push(issue);
                    }
                    IssueSeverity::Low => {
                        report.lows.push(issue);
                    }
                    IssueSeverity::NC => {
                        report.ncs.push(issue);
                    }
                }
            }
        }
    }

    println!("Detectors run, processing found issues");

    println!("Found issues processed. Printing report");
    reporter.print_report(
        get_markdown_writer(&output_file_path)?,
        &report,
        context_loader,
        root_rel_path,
        Some(output_file_path.clone()),
        no_snippets,
    )?;

    println!("Report printed to {}", output_file_path);
    Ok(())
}

fn get_markdown_writer(filename: &str) -> io::Result<File> {
    let file_path = Path::new(filename);
    if let Some(parent_dir) = file_path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }
    if Path::new(filename).exists() {
        remove_file(filename)?; // If file exists, delete it
    }
    File::create(filename)
}

pub fn read_file_to_string(path: &PathBuf) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}
