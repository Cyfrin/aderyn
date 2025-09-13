// Tools
pub mod project_overview;

use crate::context::workspace::WorkspaceContext;
use rmcp::{
    handler::server::wrapper::Parameters, model::*, schemars::JsonSchema, ErrorData as McpError,
};
use std::{any::Any, path::PathBuf, sync::Arc};
use strum::{Display, EnumString};

pub struct ModelContextProtocolState {
    pub contexts: Vec<WorkspaceContext>,
    pub root_path: PathBuf,
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

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum MCPToolNamePool {
    AderynGetProjectOverview,
}
