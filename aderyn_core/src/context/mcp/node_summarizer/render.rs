use askama::Template;
use derive_builder::Builder;

use crate::ast::NodeID;

#[derive(Builder, Template)]
#[template(path = "mcp-tool-response/node_summarizer.md")]
#[builder(pattern = "owned")]
pub struct NodeSummary {
    pub compilation_unit_index: usize,
    pub node_id: NodeID,
    pub filepath: String,
    pub code: String,
    pub containing_contract: Option<NodeInfo>,
    pub containing_modifier: Option<NodeInfo>,
    pub containing_function: Option<NodeInfo>,
    pub containing_callgraphs: Vec<EntrypointCallgraphInfo>,
}

#[derive(Default, Builder)]
#[builder(pattern = "owned")]
pub struct NodeInfo {
    pub name: String,
    pub node_id: NodeID,
}

#[derive(Default, Builder)]
#[builder(pattern = "owned")]
pub struct EntrypointCallgraphInfo {
    pub compilation_unit_index: usize,
    pub deployable_contract_id: NodeID,
    pub entrypoint_function_id: NodeID,
}
