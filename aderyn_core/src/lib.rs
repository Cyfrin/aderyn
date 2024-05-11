pub mod ast;
pub mod audit;
pub mod context;
pub mod detect;
pub mod framework;
pub mod fscloc;
pub mod report;
pub mod visitor;

use audit::auditor::get_auditor_detectors;
use detect::detector::IssueDetector;
use eyre::Result;
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::{self};
use std::path::{Path, PathBuf};

use crate::context::workspace_context::WorkspaceContext;
use crate::detect::detector::IssueSeverity;

use crate::report::printer::ReportPrinter;
use crate::report::reporter::Report;
use crate::report::Issue;

#[allow(clippy::too_many_arguments)]
pub fn run<T>(
    contexts: &[WorkspaceContext],
    output_file_path: String,
    reporter: T,
    root_rel_path: PathBuf,
    no_snippets: bool,
    stdout: bool,
    auditor_mode: bool,
    detectors: Vec<Box<dyn IssueDetector>>,
) -> Result<(), Box<dyn Error>>
where
    T: ReportPrinter<()>,
{
    if !auditor_mode {
        return run_detector_mode(
            contexts,
            output_file_path,
            reporter,
            root_rel_path,
            no_snippets,
            stdout,
            detectors,
        );
    }
    run_auditor_mode(contexts)
}

fn run_auditor_mode(contexts: &[WorkspaceContext]) -> Result<(), Box<dyn Error>> {
    for (idx, context) in contexts.iter().enumerate() {
        println!("Context {idx} ... ");
        get_auditor_detectors().par_iter_mut().for_each(|detector| {
            let found = detector.detect(context).unwrap();
            if found {
                detector.print(context);
            }
        });
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_detector_mode<T>(
    contexts: &[WorkspaceContext],
    output_file_path: String,
    reporter: T,
    root_rel_path: PathBuf,
    no_snippets: bool,
    stdout: bool,
    mut detectors: Vec<Box<dyn IssueDetector>>,
) -> Result<(), Box<dyn Error>>
where
    T: ReportPrinter<()>,
{
    println!("Get Detectors");

    println!("Running {} detectors", detectors.len());

    let detectors_used = &detectors
        .iter()
        .map(|d| (d.name(), d.severity().to_string()))
        .collect::<Vec<_>>();
    let mut report: Report = Report::default();

    let issues_collection: Vec<(Issue, IssueSeverity)> = detectors
        .par_iter_mut()
        .flat_map(|detector| {
            let mut issue: Issue = Issue {
                title: detector.title(),
                description: detector.description(),
                detector_name: detector.name(),
                instances: Default::default(),
            };

            let mut detectors_instances = BTreeMap::new();

            let collection_of_instances = contexts
                .into_par_iter()
                .flat_map(|context| {
                    let mut d = detector.skeletal_clone();
                    if let Ok(found) = d.detect(context) {
                        if found {
                            let instances = d.instances();
                            return Some(instances);
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();

            for instances in collection_of_instances {
                detectors_instances.extend(instances);
            }

            if detectors_instances.is_empty() {
                return None;
            }

            issue.instances = detectors_instances;
            Some((issue, detector.severity()))
        })
        .collect();

    for (issue, severity) in issues_collection {
        match severity {
            IssueSeverity::High => {
                report.highs.push(issue);
            }
            IssueSeverity::Low => {
                report.lows.push(issue);
            }
        }
    }

    println!("Detectors run, processing found issues");

    println!("Found issues processed. Printing report");
    if !stdout {
        reporter.print_report(
            get_writer(&output_file_path)?,
            &report,
            contexts,
            root_rel_path,
            Some(output_file_path.clone()),
            no_snippets,
            stdout,
            detectors_used,
        )?;
        println!("Report printed to {}", output_file_path);
    } else {
        reporter.print_report(
            io::stdout(),
            &report,
            contexts,
            root_rel_path,
            Some(output_file_path.clone()),
            no_snippets,
            stdout,
            detectors_used,
        )?;
    }
    Ok(())
}

fn get_writer(filename: &str) -> io::Result<File> {
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
