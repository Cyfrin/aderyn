use aderyn_core::report::*;
use std::{collections::BTreeMap, path::Path};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};

/// Report structure that is tailored to aid LSP
pub struct LspReport {
    pub high_issues: HighIssues,
    pub low_issues: LowIssues,
    pub diagnostics: BTreeMap<Url, Vec<Diagnostic>>,
}

impl LspReport {
    pub fn from(low_issues: LowIssues, high_issues: HighIssues, root_rel_path: &Path) -> Self {
        fn create_diagnostic_from_issue(
            issue_body: &IssueBody,
            instance: &IssueInstance,
            severity: DiagnosticSeverity,
            root_rel_path: &Path,
        ) -> Option<(Url, Diagnostic)> {
            // Line number
            let line_no = instance.line_no.checked_sub(1)?;

            // Character position and range from the start of the line number
            let (pos_start, pos_range) = instance.src_char2.split_once(':')?;
            let pos_start = pos_start.parse::<u32>().unwrap_or_default().checked_sub(1)?;
            let pos_range = pos_range.parse::<u32>().unwrap_or_default();

            // Craft the diagnostic message
            let mut message = format!("Title: {}\n", issue_body.title);

            if !issue_body.description.is_empty() {
                message.push_str(&format!("\nDescription: {}\n", issue_body.description));
            }

            if let Some(hint) = instance.hint.clone() {
                if !hint.is_empty() {
                    message.push_str(&format!("\nHint: {}\n", hint));
                }
            }

            message.push_str(&format!(
               "\nTo ignore this warning, add:\n\n// aderyn-ignore-next-line({})\n\nor mark as false positive:\n\n// aderyn-fp-next-line({})\n\n",
                issue_body.detector_name,
                issue_body.detector_name,
            ));

            // Make the diagnostic that LSP can understand
            let diagnostic = Diagnostic {
                range: Range {
                    start: Position { line: line_no as u32, character: pos_start },
                    end: Position { line: line_no as u32, character: pos_start + pos_range },
                },
                severity: Some(severity),
                message,
                code: None,
                code_description: None,
                source: Some("Aderyn".to_string()),
                related_information: None,
                tags: None,
                data: None,
            };
            let mut full_contract_path = root_rel_path.to_path_buf();
            full_contract_path.push(instance.contract_path.clone());
            let full_contract_path = full_contract_path.canonicalize().ok()?;
            let full_contract_path_string = full_contract_path.to_string_lossy().to_string();
            let file_uri = Url::parse(&format!("file://{}", &full_contract_path_string)).ok()?;

            Some((file_uri, diagnostic))
        }

        let mut diagnostics = BTreeMap::new();

        for issue_body in &high_issues.issues {
            for instance in &issue_body.instances {
                let Some((file_url, diagnostic)) = create_diagnostic_from_issue(
                    issue_body,
                    instance,
                    DiagnosticSeverity::WARNING,
                    root_rel_path,
                ) else {
                    continue;
                };

                let file_diagnostics: &mut Vec<Diagnostic> =
                    diagnostics.entry(file_url).or_default();

                file_diagnostics.push(diagnostic);
            }
        }
        for issue_body in &low_issues.issues {
            for instance in &issue_body.instances {
                let Some((file_url, diagnostic)) = create_diagnostic_from_issue(
                    issue_body,
                    instance,
                    DiagnosticSeverity::INFORMATION,
                    root_rel_path,
                ) else {
                    continue;
                };

                let file_diagnostics: &mut Vec<Diagnostic> =
                    diagnostics.entry(file_url).or_default();

                file_diagnostics.push(diagnostic);
            }
        }
        Self { low_issues, high_issues, diagnostics }
    }
}
