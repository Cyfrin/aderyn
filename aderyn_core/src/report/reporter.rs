use super::{extract_issue_bodies, HighIssues, Issue, IssueCount, LowIssues};

#[derive(Default, PartialEq)]
pub struct Report {
    pub highs: Vec<Issue>,
    pub lows: Vec<Issue>,
}

impl Report {
    pub fn issue_count(&self) -> IssueCount {
        IssueCount {
            high: self.highs.len(),
            low: self.lows.len(),
        }
    }

    pub fn high_issues(&self) -> HighIssues {
        HighIssues {
            issues: extract_issue_bodies(&self.highs),
        }
    }

    pub fn low_issues(&self) -> LowIssues {
        LowIssues {
            issues: extract_issue_bodies(&self.lows),
        }
    }
}
