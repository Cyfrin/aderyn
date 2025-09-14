// Tools
pub mod project_overview;

// Export tool
pub use project_overview::ProjectOverviewTool;
use solidity_ast::ProjectConfigInput;

// Imports
use crate::context::{macros::make_route, workspace::WorkspaceContext};
use rmcp::{
    handler::server::{tool::ToolRoute, wrapper::Parameters},
    model::*,
    schemars::JsonSchema,
    ErrorData as McpError,
};
use std::{any::Any, path::PathBuf, sync::Arc};
use strum::{Display, EnumString};

pub struct ModelContextProtocolState {
    pub contexts: Vec<WorkspaceContext>,
    pub root_path: PathBuf,
    pub project_config: ProjectConfigInput,
}

pub trait ModelContextProtocolTool: Send + Sync + Clone {
    type Input: JsonSchema + Any + Send;

    fn new(ctx_wrapper: Arc<ModelContextProtocolState>) -> Self;

    // Appears to the MCP client
    fn name(&self) -> String;

    // LLM uses this information to decide if this tool should be called
    fn description(&self) -> String;

    // Tool execution logic
    fn execute(&self, input: Parameters<Self::Input>) -> Result<CallToolResult, McpError>;
}

pub fn get_all_mcp_tools<T: Send + Sync + 'static>(
    state: Arc<ModelContextProtocolState>,
) -> Vec<ToolRoute<T>> {
    let tools = vec![
        // register MCP tools here
        make_route!(ProjectOverviewTool, state),
    ];
    tools
}

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum MCPToolNamePool {
    AderynGetProjectOverview,
}
