use crate::{
    ast::{ASTNode, NodeID},
    context::{
        macros::{mcp_error, mcp_success},
        mcp::{
            MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool,
            callgraph::{
                render::{
                    CallgraphToolResponseBuilder, ContractDataBuilder,
                    EntrypointFunctionDataBuilder,
                },
                utils::{build_post_order_nodes, build_raw_callgraph_for_entrypoint},
            },
        },
    },
};
use indoc::indoc;
use rmcp::{
    ErrorData as McpError, handler::server::wrapper::Parameters, model::CallToolResult, schemars,
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
    pub deployable_contract_node_id: NodeID,
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
        MCPToolNamePool::AderynExploreCallgraphFromEntrypoint.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            "The callgraph provider tool maps and analyzes function execution flows within Solidity smart contracts \
            by tracing all possible internal function calls and modifier executions triggered by the given entrypoint\
            function's Node ID. It provides inheritance-aware analysis across contract hierarchies and \
            imported libraries. It returns compilation unit index and NodeIDs of the various functions in the call chain."
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
            context.nodes.get(&payload.deployable_contract_node_id)
        else {
            return mcp_error!(
                "Node ID {} does not correspond to a contract definition",
                payload.deployable_contract_node_id
            );
        };

        let Some(ASTNode::FunctionDefinition(entrypoint)) =
            context.nodes.get(&payload.entrypoint_function_node_id)
        else {
            return mcp_error!(
                "Node ID {} does not correspond to a function",
                payload.entrypoint_function_node_id
            );
        };

        let contract_data = ContractDataBuilder::default()
            .name(contract.name.clone())
            .node_id(contract.id)
            .build()
            .expect("failed to build contract data");

        let entrypoint_function_data = EntrypointFunctionDataBuilder::default()
            .name(entrypoint.name.clone())
            .node_id(entrypoint.id)
            .build()
            .expect("failed to build entrypoint function data");

        let subgraph = build_raw_callgraph_for_entrypoint(context, contract, entrypoint)?;
        let post_order_data = build_post_order_nodes(context, &subgraph, entrypoint)?;

        let cg_response = CallgraphToolResponseBuilder::default()
            .compilation_unit_index(payload.compilation_unit_index)
            .contract(contract_data)
            .entrypoint_function(entrypoint_function_data)
            .graph(subgraph)
            .post_order_nodes(post_order_data)
            .build()
            .expect("failed to build callgraph tool response");

        mcp_success!(cg_response)
    }
}
