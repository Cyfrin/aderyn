use crate::context::workspace_context::WorkspaceContext;

use super::{
    extract_issue_bodies, CriticalIssues, HighIssues, Issue, IssueCount, LowIssues, MediumIssues,
    NcIssues,
};

#[derive(Default, PartialEq)]
pub struct Report {
    pub criticals: Vec<Issue>,
    pub highs: Vec<Issue>,
    pub mediums: Vec<Issue>,
    pub lows: Vec<Issue>,
    pub ncs: Vec<Issue>,
}

impl Report {
    pub fn issue_count(&self) -> IssueCount {
        IssueCount {
            critical: self.criticals.len(),
            high: self.highs.len(),
            medium: self.mediums.len(),
            low: self.lows.len(),
            nc: self.ncs.len(),
        }
    }

    pub fn critical_issues(&self, context: &WorkspaceContext) -> CriticalIssues {
        CriticalIssues {
            issues: extract_issue_bodies(&self.criticals, context),
        }
    }

    pub fn high_issues(&self, context: &WorkspaceContext) -> HighIssues {
        HighIssues {
            issues: extract_issue_bodies(&self.highs, context),
        }
    }
    pub fn medium_issues(&self, context: &WorkspaceContext) -> MediumIssues {
        MediumIssues {
            issues: extract_issue_bodies(&self.mediums, context),
        }
    }
    pub fn low_issues(&self, context: &WorkspaceContext) -> LowIssues {
        LowIssues {
            issues: extract_issue_bodies(&self.lows, context),
        }
    }
    pub fn nc_issues(&self, context: &WorkspaceContext) -> NcIssues {
        NcIssues {
            issues: extract_issue_bodies(&self.ncs, context),
        }
    }
}
