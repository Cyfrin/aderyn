use crate::loader::loader::ASTNode;

#[derive(Default)]
pub struct Report {
    pub criticals: Vec<Issue>,
    pub highs: Vec<Issue>,
    pub mediums: Vec<Issue>,
    pub lows: Vec<Issue>,
    pub ncs: Vec<Issue>,
    pub gas: Vec<Issue>,
}

#[derive(Default)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub instances: Vec<Option<ASTNode>>,
}
