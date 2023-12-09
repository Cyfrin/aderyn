pub mod ast;
pub mod context;
pub mod detect;
pub mod framework;
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
use crate::report::printer::{MarkdownReportPrinter, ReportPrinter};
use crate::report::reporter::{Issue, Report};

pub fn run_with_detectors(
    context_loader: ContextLoader,
    output_file_path: String,
    detectors: Vec<Box<dyn Detector>>,
) -> Result<(), Box<dyn Error>> {
    let mut report: Report = Report::default();
    for mut detector in detectors {
        if let Ok(found) = detector.detect(&context_loader) {
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

    let printer = MarkdownReportPrinter;
    println!("Found issues processed. Printing report");
    printer.print_report(
        get_markdown_writer(&output_file_path)?,
        &report,
        &context_loader,
    )?;

    println!("Report printed to {}", output_file_path);
    Ok(())
}

pub fn run(context_loader: ContextLoader, output_file_path: String) -> Result<(), Box<dyn Error>> {
    println!("Get Detectors");

    let detectors = get_all_detectors();

    println!("Running {} detectors", detectors.len());

    run_with_detectors(context_loader, output_file_path, detectors)
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
