use askama::Template;
use derive_builder::Builder;

use crate::ast::NodeID;

#[derive(Builder, Template)]
#[template(path = "mcp-tool-response/contract_surface.md")]
#[builder(pattern = "owned")]
pub struct ContractSurface {
    pub name: String,
    pub node_id: NodeID,
    pub filepath: String,
    pub compilation_unit_index: usize,
    pub total_state_variables: usize,
    pub reversed_chain: Vec<ContractInfo>,
    pub entrypoints: EntrypointFunctions,
}

#[derive(Builder, Default)]
#[builder(pattern = "owned")]
pub struct EntrypointFunctions {
    pub external_functions: Vec<FunctionInfo>,
    pub public_functions: Vec<FunctionInfo>,
    pub fallback_function: Option<FunctionInfo>,
    pub receive_function: Option<FunctionInfo>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ContractInfo {
    pub name: String,
    pub node_id: NodeID,
    pub state_variables: Vec<String>,
    pub filepath: String,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct FunctionInfo {
    pub name: String,
    pub node_id: NodeID,
    pub containing_contract: ContainingContract,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ContainingContract {
    pub name: String,
    pub node_id: NodeID,
}
