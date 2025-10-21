use super::render::*;
use crate::{
    ast::{ASTNode, NodeID},
    context::{
        macros::{mcp_error, mcp_success},
        mcp::{
            MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool,
            contract_surface::util::{
                get_classified_entrypoint_functions, get_inheritance_chain_info,
                get_total_state_variables,
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
            "Analyzes the surface area of a specific deployable contract within a compilation unit. Returns details\
            such as contract's state variables (own and inherited), and all entrypoint functions (own and inherited).\
            Use the Node ID from the list contracts tool to specify which contract to analyze."
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

        let Some(ASTNode::ContractDefinition(contract)) = context.nodes.get(&payload.node_id)
        else {
            return mcp_error!(
                "Node ID {} does not correspond to a contract definition",
                payload.node_id
            );
        };

        let (filepath, _, _) = context.get_node_sort_key_from_capturable(&contract.into());
        let total_state_variables = get_total_state_variables(context, contract);
        let reversed_chain = get_inheritance_chain_info(context, contract)?;
        let entrypoints = get_classified_entrypoint_functions(context, contract)?;

        let contract_surface = ContractSurfaceBuilder::default()
            .name(contract.name.clone())
            .node_id(payload.node_id)
            .filepath(filepath)
            .compilation_unit_index(payload.compilation_unit_index)
            .total_state_variables(total_state_variables)
            .reversed_chain(reversed_chain)
            .entrypoints(entrypoints)
            .build()
            .expect("failed to build contract surface");

        mcp_success!(contract_surface)
    }
}
