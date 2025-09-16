// Imports
use crate::context::{macros::make_route, workspace::WorkspaceContext};
use rmcp::{
    handler::server::{tool::ToolRoute, wrapper::Parameters},
    model::*,
    schemars::JsonSchema,
    ErrorData as McpError,
};
use solidity_ast::ProjectConfigInput;
use std::{any::Any, path::PathBuf, sync::Arc};
use strum::{Display, EnumString};

// Tools
pub mod contract_surface;
pub mod list_contracts;
pub mod project_overview;
pub mod tool_guide;

pub use contract_surface::ContractSurfaceTool;
pub use list_contracts::ListContractsTool;
pub use project_overview::ProjectOverviewTool;
pub use tool_guide::ToolGuide;

pub struct ModelContextProtocolState {
    pub contexts: Vec<WorkspaceContext>,
    pub root_path: PathBuf,
    pub project_config: ProjectConfigInput,
}

pub trait ModelContextProtocolTool: Send + Sync + Clone {
    type Input: JsonSchema + Any + Send;

    fn new(state: Arc<ModelContextProtocolState>) -> Self;

    // Appears to the MCP client
    fn name(&self) -> String;

    // LLM uses this information to decide if this tool should be called
    fn description(&self) -> String;

    // Tool execution logic
    fn execute(&self, input: Parameters<Self::Input>) -> Result<CallToolResult, McpError>;
}

pub fn get_all_mcp_tools<T>(state: Arc<ModelContextProtocolState>) -> Vec<ToolRoute<T>>
where
    T: Send + Sync + 'static,
{
    vec![
        // register MCP tools here
        make_route!(ToolGuide, state),
        make_route!(ProjectOverviewTool, state),
        make_route!(ListContractsTool, state),
        make_route!(ContractSurfaceTool, state),
    ]
}

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum MCPToolNamePool {
    AderynToolGuide,
    AderynGetProjectOverview,
    AderynListContracts,
    AderynContractSurfaceInspector,
}
