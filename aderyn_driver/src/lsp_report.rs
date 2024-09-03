use std::{collections::BTreeMap, path::PathBuf};

use aderyn_core::report::{HighIssues, LowIssues};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};

use crate::driver::Args;

/// Report structure that is tailored to aid LSP
pub struct LspReport {
    pub high_issues: HighIssues,
    pub low_issues: LowIssues,
    pub diagnostics: BTreeMap<Url, Vec<Diagnostic>>,
}

impl LspReport {
    pub fn from(low_issues: LowIssues, high_issues: HighIssues, args: Args) -> Self {
        let mut diagnostics = BTreeMap::new();

        for issue_body in &high_issues.issues {
            for instance in &issue_body.instances {
                let line_no = instance.line_no - 1; // 0-index
                let Some((pos_start, pos_range)) = instance.src_char.split_once(':') else {
                    continue;
                };
                let mut pos_start: u32 = pos_start.parse().unwrap_or_default();
                pos_start -= 1; // 0-index
                let pos_range: u32 = pos_range.parse().unwrap_or_default();

                let message = format!(
                    "{} {} {}",
                    issue_body.title,
                    issue_body.description,
                    instance.hint.clone().unwrap_or_default()
                );

                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_no as u32,
                            character: pos_start,
                        },
                        end: Position {
                            line: line_no as u32,
                            character: pos_start + pos_range,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    message,
                    code: None,
                    code_description: None,
                    source: Some("Aderyn".to_string()),
                    related_information: None,
                    tags: None,
                    data: None,
                };
                let mut full_contract_path = PathBuf::from(args.root.clone());
                full_contract_path.push(instance.contract_path.clone());
                let Ok(full_contract_path) = full_contract_path.canonicalize() else {
                    continue;
                };

                let full_contract_path_string = full_contract_path.to_string_lossy().to_string();
                let Ok(file_uri) = Url::parse(&format!("file://{}", &full_contract_path_string))
                else {
                    continue;
                };

                let file_diagnostics: &mut Vec<Diagnostic> =
                    diagnostics.entry(file_uri).or_default();

                file_diagnostics.push(diagnostic);
            }
        }
        for issue_body in &low_issues.issues {
            for instance in &issue_body.instances {
                let line_no = instance.line_no - 1; // 0-index
                let Some((pos_start, pos_range)) = instance.src_char.split_once(':') else {
                    continue;
                };
                let mut pos_start: u32 = pos_start.parse().unwrap_or_default();
                if pos_start >= 1 {
                    pos_start -= 1;
                } // 0-index
                let pos_range: u32 = pos_range.parse().unwrap_or_default();

                let message = format!(
                    "{} {} {}",
                    issue_body.title,
                    issue_body.description,
                    instance.hint.clone().unwrap_or_default()
                );

                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_no as u32,
                            character: pos_start,
                        },
                        end: Position {
                            line: line_no as u32,
                            character: pos_start + pos_range,
                        },
                    },
                    severity: Some(DiagnosticSeverity::INFORMATION),
                    message,
                    code: None,
                    code_description: None,
                    source: Some("Aderyn".to_string()),
                    related_information: None,
                    tags: None,
                    data: None,
                };
                let mut full_contract_path = PathBuf::from(args.root.clone());
                full_contract_path.push(instance.contract_path.clone());
                let Ok(full_contract_path) = full_contract_path.canonicalize() else {
                    continue;
                };

                let full_contract_path_string = full_contract_path.to_string_lossy().to_string();
                let Ok(file_uri) = Url::parse(&format!("file://{}", &full_contract_path_string))
                else {
                    continue;
                };

                let file_diagnostics: &mut Vec<Diagnostic> =
                    diagnostics.entry(file_uri).or_default();

                file_diagnostics.push(diagnostic);
            }
        }
        Self {
            low_issues,
            high_issues,
            diagnostics,
        }
    }
}
