use std::{
    collections::{btree_map::Entry, BTreeMap, HashMap},
    error::Error,
    ops::Add,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    ast::NodeID,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
    stats::When,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;

#[derive(Default, PartialEq)]
pub struct Report {
    pub highs: Vec<Issue>,
    pub lows: Vec<Issue>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub detector_name: String,
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    // Value is ASTNode.src
    pub instances: BTreeMap<(String, usize, String), NodeID>,
    pub hints: BTreeMap<(String, usize, String), String>,
}

#[derive(Serialize, Default)]
pub struct FilesSummary {
    pub total_source_units: usize,
    pub total_sloc: usize,
}

#[derive(Serialize, Default)]
pub struct FilesDetails {
    pub files_details: Vec<FilesDetail>,
}

impl Add<&FilesDetails> for FilesDetails {
    type Output = FilesDetails;
    fn add(mut self, rhs: &FilesDetails) -> Self::Output {
        for fd in &rhs.files_details {
            if self.files_details.iter().all(|x| x.file_path != fd.file_path) {
                self.files_details.push(fd.clone());
            }
        }
        self
    }
}

#[derive(Serialize, Clone)]
pub struct FilesDetail {
    pub file_path: String,
    pub n_sloc: usize,
}

#[derive(Serialize)]
pub struct IssueCount {
    pub high: usize,
    pub low: usize,
}

#[derive(Serialize, Debug)]
pub struct IssueInstance {
    pub contract_path: String,
    pub line_no: usize,

    /// byte_offset:byte_length
    pub src: String,

    /// char_offset:char_length
    /// Here, char_offset is counted from the beginning of the file
    pub src_char: String,

    /// char_offset:char_length
    /// Here, char_offset is counted from the beginning of the line_no
    #[serde(skip_serializing)]
    pub src_char2: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

#[derive(Serialize)]
pub struct IssueBody {
    pub title: String,
    pub description: String,
    pub detector_name: String,
    pub instances: Vec<IssueInstance>,
}

#[derive(Serialize)]
pub struct HighIssues {
    pub issues: Vec<IssueBody>,
}

#[derive(Serialize)]
pub struct LowIssues {
    pub issues: Vec<IssueBody>,
}

pub fn detect_issues(
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
                            return (
                                instances,
                                hints,
                                context.src_filepaths.clone(),
                                context.included.clone(),
                            );
                        }
                    }
                    (
                        Default::default(),
                        Default::default(),
                        context.src_filepaths.clone(),
                        context.included.clone(),
                    )
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

            for (instances, hints, src_filepaths, included) in collection_of_instances {
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
                    if !included.contains(&PathBuf::from_str(&key).unwrap()) {
                        continue;
                    }
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
                    let Some(lines_to_ignore_in_file) = ignore_lines.get(
                        &dunce::canonicalize(root_rel_path.join(&instance.0).as_path())
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                    ) else {
                        panic!(
                            "File Not Found in Ignore stats: {}",
                            &dunce::canonicalize(root_rel_path.join(&instance.0).as_path())
                                .unwrap()
                                .to_string_lossy()
                                .to_string()
                        );
                    };

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

impl Report {
    pub fn issue_count(&self) -> IssueCount {
        IssueCount { high: self.highs.len(), low: self.lows.len() }
    }

    pub fn detailed_issues(&self, contexts: &[WorkspaceContext]) -> (HighIssues, LowIssues) {
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

        let high_issues = HighIssues { issues: extract_issue_bodies(&self.highs, &file_contents) };
        let low_issues = LowIssues { issues: extract_issue_bodies(&self.lows, &file_contents) };
        (high_issues, low_issues)
    }
}

fn extract_issue_bodies(
    issues: &[Issue],
    file_contents: &HashMap<String, &String>,
) -> Vec<IssueBody> {
    issues
        .iter()
        .map(|cr| {
            let all_instances: Vec<_> = cr
                .instances
                .keys()
                .map(|(contract_path, line_no, src_location)| {
                    // Calculate character based offset & length position here
                    let (byte_offset_str, byte_len_str) = src_location.split_once(':').unwrap();
                    let byte_offset: usize = byte_offset_str.parse().unwrap();
                    let byte_length: usize = byte_len_str.parse().unwrap();
                    let content = *file_contents.get(contract_path).unwrap();
                    let mut current_line_no = 0;
                    let mut pre_line_char_offset = 0;
                    let mut char_offset = 0;
                    let mut stop_counting_preline_offset = false;
                    for (byte_offset_so_far, c) in content.char_indices() {
                        if byte_offset_so_far == byte_offset {
                            break;
                        }
                        if c == '\n' {
                            current_line_no += 1;
                            if current_line_no == line_no - 1 {
                                stop_counting_preline_offset = true;
                            }
                        }
                        if !stop_counting_preline_offset {
                            pre_line_char_offset += 1;
                        }
                        char_offset += 1;
                    }
                    let mut char_len = 0;
                    for (byte_offset_so_far, _) in content.as_str()[byte_offset..].char_indices() {
                        if byte_offset_so_far == byte_length {
                            break;
                        }
                        char_len += 1;
                    }

                    let hint = cr.hints.get(&(
                        contract_path.to_string(),
                        *line_no,
                        src_location.to_string(),
                    ));

                    IssueInstance {
                        contract_path: contract_path.clone(),
                        line_no: *line_no,
                        src: src_location.clone(),
                        src_char: format!("{}:{}", char_offset, char_len),
                        src_char2: format!("{}:{}", char_offset - pre_line_char_offset, char_len),
                        hint: hint.cloned(),
                    }
                })
                .collect();

            IssueBody {
                title: cr.title.clone(),
                description: cr.description.clone(),
                instances: all_instances,
                detector_name: cr.detector_name.clone(),
            }
        })
        .collect()
}
