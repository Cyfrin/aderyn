use crate::{
    ast::{ASTNode, NodeID},
    context::{
        macros::{mcp_error, mcp_success},
        mcp::{MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool},
    },
};
use askama::Template;
use indoc::indoc;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars, ErrorData as McpError,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct NodeSummarizerTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeSummarizerPayload {
    /// The index of the compilation unit to analyze. Must be a positive integer starting from 1.
    /// Use the project overview tool first to see all available compilation units and their
    /// indices.
    pub compilation_unit_index: usize,
    /// The Node ID for which you want to see the code snippet and some basic summary about it,
    /// such as parent contract ID, etc.
    pub node_id: NodeID,
}

impl ModelContextProtocolTool for NodeSummarizerTool {
    type Input = NodeSummarizerPayload;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynNodeSummarizer.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            ""
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

        mcp_success!("TODO")
    }
}
