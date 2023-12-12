use std::collections::BTreeMap;

use serde::Serialize;

use crate::context::loader::ContextLoader;

pub mod json_printer;
pub mod markdown_printer;
pub mod printer;
pub mod reporter;
pub mod util;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Issue {
    pub title: String,
    pub description: String,
    // Keys are source file name and line number
    // Value is ASTNode.src
    pub instances: BTreeMap<(String, usize), String>,
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
    critical: usize,
    high: usize,
    medium: usize,
    low: usize,
    nc: usize,
}

#[derive(Serialize)]
pub struct IssueInstance {
    contract_path: String,
    line_no: usize,
}

#[derive(Serialize)]
pub struct IssueBody {
    title: String,
    description: String,
    instances: Vec<IssueInstance>,
}

#[derive(Serialize)]
pub struct CriticalIssues {
    issues: Vec<IssueBody>,
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
                .map(|(contract_path, line_no)| IssueInstance {
                    contract_path: contract_path.clone(),
                    line_no: *line_no,
                })
                .collect();

            IssueBody {
                title: cr.title.clone(),
                description: cr.description.clone(),
                instances,
            }
        })
        .collect()
}

impl ContextLoader {
    pub fn files_summary(&self) -> FilesSummary {
        FilesSummary {
            total_source_units: self.source_units.len(),
            total_sloc: self.sloc_stats.code,
        }
    }

    pub fn files_details(&self) -> FilesDetails {
        let sloc_stats = &self.sloc_stats;

        let mut source_units = self.source_units.clone();
        source_units.sort_by_key(|su: &crate::ast::SourceUnit| {
            su.absolute_path.as_deref().unwrap_or("").to_string()
        });

        let files_details = source_units
            .iter()
            .map(|source_unit| {
                let filepath = source_unit.absolute_path.as_ref().unwrap();
                let report: &tokei::Report = sloc_stats
                    .reports
                    .iter()
                    .find(|r| r.name.to_str().map_or(false, |s| s.contains(filepath)))
                    .unwrap();
                FilesDetail {
                    file_path: filepath.to_owned(),
                    n_sloc: report.stats.code,
                }
            })
            .collect::<Vec<_>>();

        FilesDetails { files_details }
    }
}
