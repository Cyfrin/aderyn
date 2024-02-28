

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

    pub fn critical_issues(&self) -> CriticalIssues {
        CriticalIssues {
            issues: extract_issue_bodies(&self.criticals),
        }
    }

    pub fn high_issues(&self) -> HighIssues {
        HighIssues {
            issues: extract_issue_bodies(&self.highs),
        }
    }
    pub fn medium_issues(&self) -> MediumIssues {
        MediumIssues {
            issues: extract_issue_bodies(&self.mediums),
        }
    }
    pub fn low_issues(&self) -> LowIssues {
        LowIssues {
            issues: extract_issue_bodies(&self.lows),
        }
    }
    pub fn nc_issues(&self) -> NcIssues {
        NcIssues {
            issues: extract_issue_bodies(&self.ncs),
        }
    }
}
