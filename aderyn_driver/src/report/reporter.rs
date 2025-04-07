use std::collections::HashMap;

use aderyn_core::context::workspace_context::WorkspaceContext;

use super::{extract_issue_bodies, HighIssues, Issue, IssueCount, LowIssues};

#[derive(Default, PartialEq)]
pub struct Report {
    pub highs: Vec<Issue>,
    pub lows: Vec<Issue>,
}

impl Report {
    pub fn issue_count(&self) -> IssueCount {
        IssueCount { high: self.highs.len(), low: self.lows.len() }
    }

    pub fn detailed_issues(
        &self,
        contexts: &[WorkspaceContext],
    ) -> (HighIssues, LowIssues) {
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
