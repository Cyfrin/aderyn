use crate::process::WorkspaceContextWrapper;

use aderyn_core::context::mcp::*;
use rmcp::{
    handler::server::router::tool::ToolRouter, model::*, service::RequestContext, tool_handler,
    ErrorData as McpError, RoleServer, ServerHandler,
};
use std::sync::Arc;

pub struct McpServer {
    tool_router: ToolRouter<Self>,
}

impl McpServer {
    pub fn new(raw_state: WorkspaceContextWrapper) -> Self {
        let state = Arc::new(ModelContextProtocolState {
            contexts: raw_state.contexts,
            root_path: raw_state.root_path,
            project_config: raw_state.project_config,
        });
        let tools = get_all_mcp_tools(state);
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
            instructions: Some(
                "Intelligently navigate Solidity codebases. To get started, call the tool guide."
                    .to_string(),
            ),
        })
    }
}
