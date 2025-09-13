use crate::process::WorkspaceContextWrapper;

use aderyn_core::context::mcp::{
    project_overview::ProjectOverviewTool, ModelContextProtocolState, ModelContextProtocolTool,
};
use rmcp::{
    handler::server::{
        router::tool::ToolRouter,
        tool::{cached_schema_for_type, ToolRoute},
    },
    model::*,
    service::RequestContext,
    tool_handler, ErrorData as McpError, RoleServer, ServerHandler,
};
use std::sync::Arc;

macro_rules! make_route {
    ($tool:ty, $st:tt) => {{
        let t = <$tool>::new(Arc::clone(&$st));
        ToolRoute::new(
            Tool::new(
                t.name().to_string(),
                t.description().to_string(),
                cached_schema_for_type::<<$tool as ModelContextProtocolTool>::Input>(),
            ),
            move |a| t.execute(a),
        )
    }};
}

pub struct McpServer {
    tool_router: ToolRouter<Self>,
}

impl McpServer {
    pub fn new(raw_state: WorkspaceContextWrapper) -> Self {
        let state = Arc::new(ModelContextProtocolState {
            contexts: raw_state.contexts,
            root_path: raw_state.root_path,
        });
        let tools = vec![
            // All MCP tools must be listed here
            make_route!(ProjectOverviewTool, state),
        ];
        let mut tool_router = ToolRouter::new();
        tools.into_iter().for_each(|r| tool_router.add_route(r));
        Self { tool_router }
    }
}

#[tool_handler]
impl ServerHandler for McpServer {
    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Ask Aderyn questions about the Solidity codebase".to_string()),
        })
    }
}
