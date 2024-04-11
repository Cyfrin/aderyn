use super::{
    extract_issue_bodies, HighIssues, Issue, IssueCount, LowIssues, MediumIssues, NcIssues,
};

#[derive(Default, PartialEq)]
pub struct Report {
    pub highs: Vec<Issue>,
    pub mediums: Vec<Issue>,
    pub lows: Vec<Issue>,
    pub ncs: Vec<Issue>,
}

impl Report {
    pub fn issue_count(&self) -> IssueCount {
        IssueCount {
            high: self.highs.len(),
            medium: self.mediums.len(),
            low: self.lows.len(),
            nc: self.ncs.len(),
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
