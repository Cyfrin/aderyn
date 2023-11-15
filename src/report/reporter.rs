use crate::context::loader::{ASTNode, ContextLoader};

#[derive(Default, PartialEq)]
pub struct Report {
    pub criticals: Vec<Issue>,
    pub highs: Vec<Issue>,
    pub mediums: Vec<Issue>,
    pub lows: Vec<Issue>,
    pub ncs: Vec<Issue>,
}

impl Report {
    pub fn post_process(&mut self, loader: &ContextLoader) {
        sort_issue_instances(&mut self.criticals, loader);
        sort_issue_instances(&mut self.highs, loader);
        sort_issue_instances(&mut self.mediums, loader);
        sort_issue_instances(&mut self.lows, loader);
        sort_issue_instances(&mut self.ncs, loader);
    }
}

fn sort_issue_instances(issues: &mut Vec<Issue>, loader: &ContextLoader) {
    for issue in issues {
        issue.sort_instances(loader);
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub instances: Vec<Option<ASTNode>>,
}

impl Issue {
    fn sort_instances(&mut self, loader: &ContextLoader) {
        self.instances.sort_by(|a, b| {
            let a_key = loader.get_node_sort_key(a.as_ref().unwrap());
            let b_key = loader.get_node_sort_key(b.as_ref().unwrap());
            a_key.cmp(&b_key)
        });
    }
}
