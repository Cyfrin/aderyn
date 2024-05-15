use std::{
    collections::{BTreeMap, HashSet},
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
pub struct LowIssues {
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
