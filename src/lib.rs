pub mod ast;
pub mod context;
pub mod detect;
pub mod framework;
pub mod report;
pub mod visitor;

use eyre::Result;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io;
use std::path::Path;

use crate::context::loader::ContextLoader;
use crate::detect::detector::{get_all_detectors, IssueSeverity};
use crate::report::printer::{MarkdownReportPrinter, ReportPrinter};
use crate::report::reporter::{Issue, Report};

pub fn run(context_loader: ContextLoader) -> Result<(), Box<dyn Error>> {
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
                }
            }
        }
    }

    println!("Detectors run, processing found issues");

    let printer = MarkdownReportPrinter;
    report.post_process(&context_loader);
    println!("Found issues processed. Printing report");
    printer.print_report(get_markdown_writer("report.md")?, &report, &context_loader)?;

    println!("Report printed to ./report.md");
    Ok(())
}

fn get_markdown_writer(filename: &str) -> io::Result<File> {
    if Path::new(filename).exists() {
        remove_file(filename)?; // If file exists, delete it
    }
    File::create(filename)
}
