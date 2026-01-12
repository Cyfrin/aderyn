// Imports
use crate::context::workspace::WorkspaceContext;
use rmcp::{
    ErrorData as McpError,
    handler::server::{tool::ToolRoute, wrapper::Parameters},
    model::*,
    schemars::JsonSchema,
};
use solidity_ast::ProjectConfigInput;
use std::{any::Any, path::PathBuf, sync::Arc};
use strum::{Display, EnumString};

// Tools
pub mod callgraph;
pub mod contract_surface;
pub mod list_contracts;
pub mod node_finder;
pub mod node_summarizer;
pub mod project_overview;
pub mod tool_guide;

// Helpers
mod macros;

pub use callgraph::CallgraphTool;
pub use contract_surface::ContractSurfaceTool;
pub use list_contracts::ListContractsTool;
pub use node_finder::NodeFinderTool;
pub use node_summarizer::NodeSummarizerTool;
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

macro_rules! make_route {
    ($tool:ty, $st:tt) => {{
        let t = <$tool>::new(std::sync::Arc::clone(&$st));
        rmcp::handler::server::tool::ToolRoute::new(
            rmcp::model::Tool::new(
                t.name().to_string(),
                t.description().to_string(),
                rmcp::handler::server::tool::cached_schema_for_type::<
                    <$tool as crate::context::mcp::ModelContextProtocolTool>::Input,
                >(),
            ),
            move |a| t.execute(a),
        )
    }};
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
        make_route!(CallgraphTool, state),
        make_route!(NodeSummarizerTool, state),
        make_route!(NodeFinderTool, state),
    ]
}

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MCPToolNamePool {
    AderynGetToolGuide,
    AderynGetProjectOverview,
    AderynListContracts,
    AderynContractSurfaceInspector,
    AderynExploreCallgraphFromEntrypoint,
    AderynNodeSummarizer,
    AderynNodeFinder,
}
