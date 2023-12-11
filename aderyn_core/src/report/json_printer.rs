use std::io::{Result, Write};

use crate::context::loader::ContextLoader;
use serde::Serialize;

use super::{
    printer::ReportPrinter,
    reporter::{Issue, Report},
};

#[derive(Serialize)]
pub struct JsonContent {
    files_summary: FilesSummary,
    files_details: FilesDetails,
    issue_count: IssueCount,
    critical_issues: CriticalIssues,
    high_issues: HighIssues,
    medium_issue: MediumIssues,
    low_issues: LowIssues,
    nc_issues: NcIssues,
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

pub struct JsonPrinter;

/**
 * JSON should mimick MD
    {
        "files_summary": {...},
        "files_details": {...},
        "issue_summary": {...},
        "critical_issues": {...},
        "high_issues": {...},
    ...
    }
*/

impl ReportPrinter<()> for JsonPrinter {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        loader: &ContextLoader,
    ) -> Result<()> {
        let content = JsonContent {
            files_summary: loader.files_summary(),
            files_details: loader.files_details(),
            issue_count: report.issue_count(),
            critical_issues: report.critical_issues(),
            high_issues: report.high_issues(),
            medium_issue: report.medium_issues(),
            low_issues: report.low_issues(),
            nc_issues: report.nc_issues(),
        };
        let value = serde_json::to_value(content).unwrap();
        _ = serde_json::to_writer_pretty(writer, &value);
        Ok(())
    }
}

impl ContextLoader {
    fn files_summary(&self) -> FilesSummary {
        FilesSummary {
            total_source_units: self.source_units.len(),
            total_sloc: self.sloc_stats.code,
        }
    }

    fn files_details(&self) -> FilesDetails {
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

fn extract_issue_bodies(issues: &[Issue]) -> Vec<IssueBody> {
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

impl Report {
    fn issue_count(&self) -> IssueCount {
        IssueCount {
            critical: self.criticals.len(),
            high: self.highs.len(),
            medium: self.mediums.len(),
            low: self.lows.len(),
            nc: self.ncs.len(),
        }
    }

    fn critical_issues(&self) -> CriticalIssues {
        CriticalIssues {
            issues: extract_issue_bodies(&self.criticals),
        }
    }

    fn high_issues(&self) -> HighIssues {
        HighIssues {
            issues: extract_issue_bodies(&self.highs),
        }
    }
    fn medium_issues(&self) -> MediumIssues {
        MediumIssues {
            issues: extract_issue_bodies(&self.mediums),
        }
    }
    fn low_issues(&self) -> LowIssues {
        LowIssues {
            issues: extract_issue_bodies(&self.lows),
        }
    }
    fn nc_issues(&self) -> NcIssues {
        NcIssues {
            issues: extract_issue_bodies(&self.ncs),
        }
    }
}
