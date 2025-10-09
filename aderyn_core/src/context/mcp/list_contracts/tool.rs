use crate::context::{
    macros::{mcp_error, mcp_success},
    mcp::{
        MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool,
        list_contracts::render::{ContractInfoBuilder, ContractsListBuilder},
    },
};
use indoc::indoc;
use rmcp::{
    ErrorData as McpError, handler::server::wrapper::Parameters, model::CallToolResult, schemars,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct ListContractsTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ListContractsPayload {
    /// The index of the compilation unit to analyze. Must be a positive integer starting from 1.
    /// Use the project overview tool first to see all available compilation units and their
    /// indices.
    pub compilation_unit_index: usize,
}

impl ModelContextProtocolTool for ListContractsTool {
    type Input = ListContractsPayload;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynListContracts.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            "Enumerates deployable contracts within a specific compilation unit. Returns contract names, \
            file names (relative to the project root) and node IDs."
        }
        .to_string()
    }

    fn execute(&self, input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        let comp_unit_idx = input.0.compilation_unit_index;
        if comp_unit_idx < 1 || comp_unit_idx > self.state.contexts.len() {
            return mcp_error!(
                "invalid value passed for compilation unit - must be in the range [1, {}] inclusive",
                self.state.contexts.len()
            );
        }
        let context = self.state.contexts.get(comp_unit_idx - 1).expect("bounds check failed");
        let mut contracts_info = vec![];
        for contract in context.deployable_contracts() {
            let (filepath, _, _) = context.get_node_sort_key_from_capturable(&contract.into());
            let contract_info = ContractInfoBuilder::default()
                .name(contract.name.clone())
                .filepath(filepath)
                .node_id(contract.id)
                .build()
                .expect("failed to build contract info");
            contracts_info.push(contract_info);
        }

        let contract_list = ContractsListBuilder::default()
            .compilation_unit_index(comp_unit_idx)
            .contracts_info(contracts_info)
            .build()
            .expect("failed to build contracts list");

        mcp_success!(contract_list)
    }
}
