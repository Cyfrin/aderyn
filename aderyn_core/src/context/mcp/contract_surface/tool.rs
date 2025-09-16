use crate::{
    ast::NodeID,
    context::{
        macros::{mcp_error, mcp_success},
        mcp::{MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool},
    },
};
use indoc::indoc;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars, ErrorData as McpError,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct ContractSurfaceTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ContractSurfacePayload {
    /// The index of the compilation unit to analyze. Must be a positive integer starting from 1.
    /// Use the project overview tool first to see all available compilation units and their
    /// indices.
    pub compilation_unit_index: usize,
    /// The Node ID of the specific contract to analyze. Obtain this from the list contracts tool,
    /// which returns Node IDs for all deployable contracts in the compilation unit. Each contract
    /// has a unique Node ID within its compilation unit.
    pub node_id: NodeID,
}

impl ModelContextProtocolTool for ContractSurfaceTool {
    type Input = ContractSurfacePayload;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynContractSurfaceInspector.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            "Analyzes the surface area of a specific deployable contract within a compilation unit. Returns\
            details about the contract's state variables (own and inherited), and entrypoint functions. \
            Use the Node ID from the list contracts tool to specify which contract to analyze."
        }
        .to_string()
    }

    fn execute(&self, input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        let comp_unit_index = input.0.compilation_unit_index;
        if comp_unit_index < 1 || comp_unit_index > self.state.contexts.len() {
            return mcp_error!(
                "invalid value passed for compilation unit - must be in the range [1, {}] inclusive",
                self.state.contexts.len()
            );
        }
        let context = self.state.contexts.get(comp_unit_index - 1).expect("bounds check failed");
        mcp_success!("TODO")
    }
}
