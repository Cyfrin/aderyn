use crate::{
    ast::{ASTNode, NodeID},
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
pub struct CallgraphTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CallgraphPayload {
    /// The index of the compilation unit to analyze. Must be a positive integer starting from 1.
    /// Use the project overview tool first to see all available compilation units and their
    /// indices.
    pub compilation_unit_index: usize,
    /// The Node ID of the main contract to analyze. Obtain this from the list contracts tool,
    /// which returns Node IDs for all deployable contracts in the compilation unit. Each contract
    /// has a unique Node ID within its compilation unit.
    pub orignal_contract_node_id: NodeID,
    /// The Node ID of the specific entrypoint function in the contract to start exploring the
    /// callgraph from. Obtain this from contract surface area tool.
    pub entrypoint_function_node_id: NodeID,
}

impl ModelContextProtocolTool for CallgraphTool {
    type Input = CallgraphPayload;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynCallgraphExplorer.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            "The callgraph provider tool maps and analyzes function execution flows within Solidity smart contracts \
            by tracing all possible internal function calls and modifier executions triggered when given the Node ID of \
            an entrypoint function. It provides inheritance-aware analysis across contract hierarchies and \
            imported libraries"
        }
        .to_string()
    }

    fn execute(&self, input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        let payload = input.0;

        if payload.compilation_unit_index < 1
            || payload.compilation_unit_index > self.state.contexts.len()
        {
            return mcp_error!(
                "Invalid compilation unit index: {}. Must be in range [1, {}]",
                payload.compilation_unit_index,
                self.state.contexts.len()
            );
        }

        let context = self
            .state
            .contexts
            .get(payload.compilation_unit_index - 1)
            .expect("Compilation unit index bounds check failed");

        let Some(ASTNode::ContractDefinition(contract)) =
            context.nodes.get(&payload.orignal_contract_node_id)
        else {
            return mcp_error!(
                "Node ID {} does not correspond to a contract definition",
                payload.orignal_contract_node_id
            );
        };

        mcp_success!("TODO")
    }
}
