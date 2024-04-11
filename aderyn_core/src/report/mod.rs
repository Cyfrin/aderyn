use std::collections::{BTreeMap, HashSet};

use serde::Serialize;

use crate::{ast::NodeID, context::workspace_context::WorkspaceContext};

pub mod json_printer;
pub mod markdown_printer;
pub mod printer;
pub mod reporter;
pub mod util;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub detector_name: String,
    // Keys are source file name and line number
    // Value is ASTNode.src
    pub instances: BTreeMap<(String, usize, String), NodeID>,
}

#[derive(Serialize)]
pub struct FilesSummary {
    total_source_units: usize,
    total_sloc: usize,
}

#[derive(Serialize)]
pub struct FilesDetails {
    files_details: Vec<FilesDetail>,
}

#[derive(Serialize)]
pub struct FilesDetail {
    file_path: String,
    n_sloc: usize,
}

#[derive(Serialize)]
pub struct IssueCount {
    high: usize,
    medium: usize,
    low: usize,
    nc: usize,
}

#[derive(Serialize)]
pub struct IssueInstance {
    contract_path: String,
    line_no: usize,
    src: String,
}

#[derive(Serialize)]
pub struct IssueBody {
    title: String,
    description: String,
    detector_name: String,
    instances: Vec<IssueInstance>,
}

#[derive(Serialize)]
pub struct HighIssues {
    issues: Vec<IssueBody>,
}

#[derive(Serialize)]
pub struct MediumIssues {
    issues: Vec<IssueBody>,
}

#[derive(Serialize)]
pub struct LowIssues {
    issues: Vec<IssueBody>,
}

#[derive(Serialize)]
pub struct NcIssues {
    issues: Vec<IssueBody>,
}

pub fn extract_issue_bodies(issues: &[Issue]) -> Vec<IssueBody> {
    issues
        .iter()
        .map(|cr| {
            let instances = cr
                .instances
                .keys()
                .map(|(contract_path, line_no, src_location)| IssueInstance {
                    contract_path: contract_path.clone(),
                    line_no: *line_no,
                    src: src_location.clone(),
                })
                .collect();

            IssueBody {
                title: cr.title.clone(),
                description: cr.description.clone(),
                instances,
                detector_name: cr.detector_name.clone(),
            }
        })
        .collect()
}

impl WorkspaceContext {
    pub fn files_summary(&self) -> FilesSummary {
        FilesSummary {
            total_source_units: self.src_filepaths.len(),
            total_sloc: self.sloc_stats.iter().fold(0, |acc, x| acc + *x.1),
        }
    }

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
