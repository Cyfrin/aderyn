pub mod ast;
pub mod compiler;
pub mod detector;
pub mod loader;
pub mod report;
pub mod visitor;

use eyre::Result;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::compiler::foundry::FoundryOutput;
use crate::detector::detector::{get_all_detectors, IssueSeverity};
use crate::loader::loader::ContractLoader;
use crate::report::printer::{MarkdownReportPrinter, ReportPrinter};
use crate::report::report::{Issue, Report};
use crate::visitor::ast_visitor::Node;

pub fn run(filepaths: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut contract_loader = ContractLoader::default();

    for filepath in filepaths {
        let foundry_output = read_foundry_output_file(filepath.to_str().unwrap())?;
        foundry_output.ast.accept(&mut contract_loader)?;
    }

    println!(
        "Contracts loaded, number of Node IDs found: {:?}",
        contract_loader.nodes.len()
    );

    println!("Get Detectors");

    let detectors = get_all_detectors();

    println!("Running {} detectors", detectors.len());

    let mut report: Report = Report::default();
    for mut detector in detectors {
        if let Ok(found) = detector.detect(&contract_loader) {
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
    printer.print_report(std::io::stdout(), &report, &contract_loader)?;

    Ok(())
}

fn read_foundry_output_file(filepath: &str) -> Result<FoundryOutput> {
    println!("Foundry output path: {:?}", filepath);
    Ok(serde_json::from_reader(BufReader::new(File::open(
        filepath,
    )?))?)
}
