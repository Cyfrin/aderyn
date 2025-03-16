pub mod ast;
pub mod audit;
pub mod context;
pub mod detect;
pub mod fscloc;
pub mod report;
pub mod visitor;

use audit::auditor::{get_auditor_detectors, AuditorPrinter, BasicAuditorPrinter};
use detect::detector::IssueDetector;
use eyre::Result;
use fscloc::cloc::When;
use prettytable::Row;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::{
    collections::{btree_map::Entry, BTreeMap, HashMap},
    error::Error,
    fs::{remove_file, File},
    io::{self},
    path::{Path, PathBuf},
};

use crate::{context::workspace_context::WorkspaceContext, detect::detector::IssueSeverity};

use crate::report::{printer::ReportPrinter, reporter::Report, Issue};

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
    let audit_detectors_with_output = get_auditor_detectors()
        .par_iter_mut()
        .flat_map(|detector| {
            // Keys -> detector's title
            // Value -> (table titles, table rows)
            let mut grouped_instances: BTreeMap<String, (Row, Vec<Row>)> = BTreeMap::new();

            for context in contexts {
                let mut d = detector.skeletal_clone();
                if let Ok(found) = d.detect(context) {
                    if found {
                        match grouped_instances.entry(d.title()) {
                            Entry::Occupied(o) => o.into_mut().1.extend(d.table_rows()),
                            Entry::Vacant(v) => {
                                v.insert((d.table_titles(), d.table_rows()));
                            }
                        };
                    }
                }
            }

            grouped_instances
        })
        .collect::<Vec<_>>();

    for (title, (table_titles, table_rows)) in audit_detectors_with_output {
        let num_instances = table_rows.len();
        BasicAuditorPrinter::print(&title, table_titles, table_rows);
        if num_instances > 0 {
            println!("Number of instances: {}", num_instances);
        }
    }

    Ok(())
}

pub fn get_report(
    contexts: &[WorkspaceContext],
    root_rel_path: &Path,
    detectors: Vec<Box<dyn IssueDetector>>,
) -> Result<Report, Box<dyn Error>> {
    let mut ignore_lines = HashMap::new();
    for context in contexts {
        ignore_lines.extend(context.ignore_lines_stats.clone());
    }

    let mut report: Report = Report::default();

    let issues_collection: Vec<(Issue, IssueSeverity)> = detectors
        .par_iter()
        .flat_map(|detector| {
            let mut issue: Issue = Issue {
                title: detector.title(),
                description: detector.description(),
                detector_name: detector.name(),
                instances: Default::default(),
                hints: Default::default(),
            };

            let mut detectors_instances = BTreeMap::new();
            let mut detector_hints = BTreeMap::new();

            let collection_of_instances = contexts
                .into_par_iter()
                .map(|context| {
                    let mut d = detector.skeletal_clone();
                    if let Ok(found) = d.detect(context) {
                        if found {
                            let instances = d.instances();
                            let hints = d.hints();
                            return (instances, hints, context.src_filepaths.clone());
                        }
                    }
                    (Default::default(), Default::default(), context.src_filepaths.clone())
                })
                .collect::<Vec<_>>();

            // Commit detector instances
            //
            // NOTE: Possible merge conflict here
            //
            // For a given detector D, in a file F,
            //
            // Context C1 captures instances A, B, C
            // Context C2 captures instances B, C, D
            //
            // This is a conflict!
            //
            // We need a strategy to resolve this and it depends on the detector
            //
            // For example, if the detector determines that A, B, C are immutable when considering
            // one set of files but B, C, D when considering another set of files, it is only safe
            // to conclude that the B, C are immutable.
            //
            // Such a technique to resolve this conflict would be called INTERSECTION strategy
            //
            // Alternative way would be UNION strategy
            //

            // NOTE: Intersection strategy logic
            #[allow(clippy::complexity)]
            let mut grouped_instances: BTreeMap<
                String,
                Vec<BTreeMap<(String, usize, String), i64>>,
            > = Default::default();

            for (instances, hints, src_filepaths) in collection_of_instances {
                let mut grouped_instances_context: BTreeMap<
                    String,
                    BTreeMap<(String, usize, String), i64>,
                > = BTreeMap::new();

                for (key, value) in instances {
                    match grouped_instances_context.entry(key.0.clone()) {
                        Entry::Vacant(v) => {
                            let mut mini_btree = BTreeMap::new();
                            mini_btree.insert(key, value);
                            v.insert(mini_btree);
                        }
                        Entry::Occupied(mut o) => {
                            o.get_mut().insert(key, value);
                        }
                    };
                }

                for key in src_filepaths {
                    if let Entry::Vacant(v) = grouped_instances_context.entry(key) {
                        v.insert(Default::default());
                    }
                }

                for (key, value) in grouped_instances_context {
                    match grouped_instances.entry(key.clone()) {
                        Entry::Vacant(v) => {
                            v.insert(vec![value]);
                        }
                        Entry::Occupied(mut o) => {
                            o.get_mut().push(value);
                        }
                    }
                }

                detector_hints.extend(hints);
            }

            for (_filename, value) in grouped_instances {
                // Find the common instances across all the contexts' BTrees.

                let mut selected_instances = BTreeMap::new();

                for instances in &value {
                    for instance in instances {
                        if value.iter().all(|tree| tree.contains_key(&instance.0.clone())) {
                            selected_instances.insert(instance.0.clone(), *instance.1);
                        }
                    }
                }

                detectors_instances.extend(selected_instances);
            }
            // NOTE: Union strategy would work something like this
            //
            // for (instances, hints, _src_filepaths) in collection_of_instances.into_iter() {
            //       if instances.is_empty() {
            //           continue;
            //       }
            //       detectors_instances.extend(instances);
            //       detector_hints.extend(hints);
            //  }

            if detectors_instances.is_empty() {
                return None;
            }

            issue.instances = detectors_instances
                .into_iter()
                .filter(|(instance, _)| {
                    let lines_to_ignore_in_file = ignore_lines
                        .get(
                            &dunce::canonicalize(root_rel_path.join(&instance.0).as_path())
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                        )
                        .unwrap();
                    if lines_to_ignore_in_file.is_empty() {
                        return true;
                    }

                    for ignore_condition in lines_to_ignore_in_file.iter() {
                        if ignore_condition.which == instance.1 {
                            match &ignore_condition.when {
                                When::Always => {
                                    return false;
                                }
                                When::ForDetectorsWithNames(names) => {
                                    if names.iter().any(|name| *name == detector.name()) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    true
                })
                .collect();

            issue.hints = detector_hints;

            if issue.instances.is_empty() {
                return None;
            }

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

    Ok(report)
}

#[allow(clippy::too_many_arguments)]
fn run_detector_mode<T>(
    contexts: &[WorkspaceContext],
    output_file_path: String,
    reporter: T,
    root_rel_path: PathBuf,
    no_snippets: bool,
    stdout: bool,
    detectors: Vec<Box<dyn IssueDetector>>,
) -> Result<(), Box<dyn Error>>
where
    T: ReportPrinter<()>,
{
    let mut ignore_lines = HashMap::new();
    for context in contexts {
        ignore_lines.extend(context.ignore_lines_stats.clone());
    }

    println!("Get Detectors");

    println!("Running {} detectors", detectors.len());

    let detectors_used =
        &detectors.iter().map(|d| (d.name(), d.severity().to_string())).collect::<Vec<_>>();
    println!("Detectors run, processing found issues");

    let report = get_report(contexts, &root_rel_path, detectors)?;

    println!("Found issues processed. Printing report");

    let file_contents = contexts
        .iter()
        .flat_map(|context| context.source_units())
        .map(|source_unit| {
            (
                source_unit.absolute_path.as_ref().unwrap().to_owned(),
                source_unit.source.as_ref().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

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
            &file_contents,
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
            &file_contents,
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
