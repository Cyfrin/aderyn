use std::collections::HashMap;

use aderyn_core::context::workspace_context::WorkspaceContext;

use super::{HighIssues, Issue, IssueBody, IssueCount, IssueInstance, LowIssues};

#[derive(Default, PartialEq)]
pub struct Report {
    pub highs: Vec<Issue>,
    pub lows: Vec<Issue>,
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

pub fn extract_issue_bodies(
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
