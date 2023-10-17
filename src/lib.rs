pub mod ast;
pub mod context;
pub mod detector;
pub mod framework;
pub mod report;
pub mod visitor;

use eyre::Result;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use crate::context::loader::ContextLoader;
use crate::detector::detector::{get_all_detectors, IssueSeverity};
use crate::framework::foundry::FoundryOutput;
use crate::report::printer::{MarkdownReportPrinter, ReportPrinter};
use crate::report::report::{Issue, Report};
use crate::visitor::ast_visitor::Node;

pub fn run(filepaths: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut context_loader = ContextLoader::default();

    for filepath in filepaths {
        // read_foundry_output_file and print an error message if it fails
        if let Ok(foundry_output) = read_foundry_output_file(filepath.to_str().unwrap()) {
            foundry_output.ast.accept(&mut context_loader)?;
        } else {
            eprintln!(
                "Error reading Foundry output file: {}",
                filepath.to_str().unwrap()
            );
        }
    }

    println!(
        "Contracts loaded, number of Node IDs found: {:?}",
        context_loader.nodes.len()
    );

    println!("Get Detectors");

    let detectors = get_all_detectors();

    println!("Running {} detectors", detectors.len());

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
                    IssueSeverity::Gas => {
                        report.gas.push(issue);
                    }
                }
            }
        }
    }

    println!("Detectors run, printing report");

    let printer = MarkdownReportPrinter;
    printer.print_report(get_markdown_writer("report.md")?, &report, &context_loader)?;

    Ok(())
}

fn get_markdown_writer(filename: &str) -> io::Result<File> {
    if Path::new(filename).exists() {
        remove_file(filename)?; // If file exists, delete it
    }
    File::create(filename)
}

fn read_foundry_output_file(filepath: &str) -> Result<FoundryOutput> {
    println!("Foundry output path: {:?}", filepath);
    Ok(serde_json::from_reader(BufReader::new(File::open(
        filepath,
    )?))?)
}
