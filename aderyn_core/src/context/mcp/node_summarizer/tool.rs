use crate::{
    ast::NodeID,
    context::{
        macros::{mcp_error, mcp_success},
        mcp::{
            MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool,
            node_summarizer::{render, utils::*},
        },
    },
};
use askama::Template;
use indoc::indoc;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars,
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
            "Given a compilation unit index and a Node ID, returns a focused summary of \
            that exact AST node (e.g. function, modifier, event, variable, struct). The summary is limited to \
            information directly available in the AST plus a source snippet"
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

        let Some(node) = context.nodes.get(&payload.node_id) else {
            return mcp_error!(
                "Node with ID {} not found in compilation unit index {}",
                payload.node_id,
                payload.compilation_unit_index
            );
        };

        let (filepath, _, _) = context.get_node_sort_key_pure(node);
        let code_snippet = get_code_snippet(context, node);

        let renderer = render::NodeSummaryBuilder::default()
            .compilation_unit_index(payload.compilation_unit_index)
            .node_id(payload.node_id)
            .filepath(filepath)
            .code(code_snippet)
            .containing_contract(get_containing_contract(context, node))
            .containing_function(get_containing_function(context, node))
            .containing_modifier(get_containing_modifier(context, node))
            .containing_callgraphs(get_containing_callgraphs(
                payload.compilation_unit_index,
                context,
                node,
            ))
            .build()
            .map_err(|_| McpError::internal_error("failed to build node summary", None))?;

        let text =
            renderer.render().map_err(|_| McpError::internal_error("failed to render", None))?;
        mcp_success!(text)
    }
}
