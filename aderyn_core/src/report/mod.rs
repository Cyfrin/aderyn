use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::Add,
};

use serde::Serialize;

use crate::{ast::NodeID, context::workspace_context::WorkspaceContext};

pub mod json_printer;
pub mod markdown_printer;
pub mod printer;
pub mod reporter;
pub mod sarif_printer;
pub mod util;

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
    total_source_units: usize,
    total_sloc: usize,
}

#[derive(Serialize, Default)]
pub struct FilesDetails {
    files_details: Vec<FilesDetail>,
}

impl Add<&FilesDetails> for FilesDetails {
    type Output = FilesDetails;
    fn add(mut self, rhs: &FilesDetails) -> Self::Output {
        for fd in &rhs.files_details {
            if self
                .files_details
                .iter()
                .all(|x| x.file_path != fd.file_path)
            {
                self.files_details.push(fd.clone());
            }
        }
        self
    }
}

#[derive(Serialize, Clone)]
pub struct FilesDetail {
    file_path: String,
    n_sloc: usize,
}

#[derive(Serialize)]
pub struct IssueCount {
    high: usize,
    low: usize,
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

pub fn extract_issue_bodies(
    issues: &[Issue],
    file_contents: &HashMap<String, &String>,
) -> Vec<IssueBody> {
    issues
        .iter()
        .map(|cr| {
            let instances: Vec<_> = cr
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

            let mut all_instances = vec![];
            all_instances.extend(instances);

            IssueBody {
                title: cr.title.clone(),
                description: cr.description.clone(),
                instances: all_instances,
                detector_name: cr.detector_name.clone(),
            }
        })
        .collect()
}

impl WorkspaceContext {
    pub fn files_details(&self) -> FilesDetails {
        let sloc_stats = &self.sloc_stats;

        let mut source_units = self.source_units_context.clone();
        source_units.sort_by_key(|su: &crate::ast::SourceUnit| {
            su.absolute_path.as_deref().unwrap_or("").to_string()
        });

        let mut seen_paths = HashSet::new();
        let files_details = source_units
            .iter()
            .filter_map(|source_unit| {
                let filepath = source_unit.absolute_path.as_ref()?;
                if seen_paths.insert(filepath.clone()) {
                    let report = sloc_stats.iter().find(|r| r.0.contains(filepath))?;
                    Some(FilesDetail {
                        file_path: filepath.to_owned(),
                        n_sloc: *report.1,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        FilesDetails { files_details }
    }
}
