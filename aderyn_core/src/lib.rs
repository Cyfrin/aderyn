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
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
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
    context: &WorkspaceContext,
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
            context,
            output_file_path,
            reporter,
            root_rel_path,
            no_snippets,
            stdout,
            detectors,
        );
    }
    run_auditor_mode(context)
}

fn run_auditor_mode(context: &WorkspaceContext) -> Result<(), Box<dyn Error>> {
    get_auditor_detectors().par_iter_mut().for_each(|detector| {
        let found = detector.detect(context).unwrap();
        if found {
            detector.print(context);
        }
    });

    Ok(())
}

fn run_detector_mode<T>(
    context: &WorkspaceContext,
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

    let issues: Vec<Option<(Issue, IssueSeverity)>> = detectors
        .par_iter_mut()
        .map(|detector| {
            if let Ok(found) = detector.detect(context) {
                if found {
                    let issue: Issue = Issue {
                        title: detector.title(),
                        description: detector.description(),
                        detector_name: detector.name(),
                        instances: detector.instances(),
                    };
                    return Some((issue, detector.severity()));
                }
            }
            None
        })
        .collect();

    for (issue, severity) in issues.into_iter().flatten() {
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
            context,
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
            context,
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
