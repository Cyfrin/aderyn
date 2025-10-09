use askama::Template;
use derive_builder::Builder;
use serde::Serialize;

use crate::ast::NodeID;

#[derive(Builder, Serialize, Template)]
#[template(path = "mcp-tool-response/list_contracts.md")]
#[builder(pattern = "owned")]
pub struct ContractsList {
    pub contracts_info: Vec<ContractInfo>,
    pub compilation_unit_index: usize,
}

#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct ContractInfo {
    pub name: String,
    pub filepath: String,
    pub node_id: NodeID,
}
