use std::collections::BTreeMap;

#[derive(Default, PartialEq)]
pub struct Report {
    pub criticals: Vec<Issue>,
    pub highs: Vec<Issue>,
    pub mediums: Vec<Issue>,
    pub lows: Vec<Issue>,
    pub ncs: Vec<Issue>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Issue {
    pub title: String,
    pub description: String,
    // Keys are source file name and line number
    // Value is ASTNode.src
    pub instances: BTreeMap<(String, usize), String>,
}
