use askama::Template;
use derive_builder::Builder;
use serde::Serialize;

use crate::ast::NodeID;

#[derive(Template, Builder, Serialize)]
#[template(path = "mcp-tool-response/node_finder_search.md")]
#[builder(pattern = "owned")]
pub struct NodeFinderMatches {
    pub term: String,
    pub node_type: String,
    pub matching_nodes: Vec<NodeInfo>,
}

#[derive(Template, Builder, Serialize)]
#[template(path = "mcp-tool-response/node_finder_get_all.md")]
#[builder(pattern = "owned")]
pub struct NodeFinderAll {
    pub node_type: String,
    pub nodes: Vec<NodeInfo>,
}

#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct NodeInfo {
    pub name: String,
    pub node_id: NodeID,
    pub compilation_unit_index: usize,
}
