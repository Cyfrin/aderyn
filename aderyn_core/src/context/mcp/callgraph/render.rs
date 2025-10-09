use askama::Template;
use derive_builder::Builder;
use serde::Serialize;

use crate::{ast::NodeID, context::graph::RawCallGraph};

#[derive(Builder, Serialize, Template)]
#[template(path = "mcp-tool-response/callgraph.md")]
#[builder(pattern = "owned")]
pub struct CallgraphToolResponse {
    pub compilation_unit_index: usize,
    pub contract: ContractData,
    pub entrypoint_function: EntrypointFunctionData,
    pub graph: RawCallGraph, /* NOTE: we do not supply the whole callgraph; only the
                              * subgraph traversed from the entrypoint function */
    pub post_order_nodes: Vec<NodeData>,
}

#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct ContractData {
    pub name: String,
    pub node_id: NodeID,
}

#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct EntrypointFunctionData {
    pub name: String,
    pub node_id: NodeID,
}

#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct NodeData {
    pub name: String,
    pub node_id: NodeID,
    pub called_nodes: Vec<NodeData>,
}
