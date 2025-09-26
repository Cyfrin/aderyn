use crate::process::WorkspaceContextWrapper;

use aderyn_core::context::mcp::*;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, handler::server::router::tool::ToolRouter,
    model::*, service::RequestContext, tool_handler,
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
    ) -> Result<rmcp::model::InitializeResult, McpError> {
        Ok(ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Intelligently search for patterns in Solidity codebases. To get started, call the tool guide."
                    .to_string(),
            ),
        })
    }
}

#[derive(Clone)]
pub struct SingletonMcpServer(Arc<McpServer>);

impl SingletonMcpServer {
    pub fn new(inner: McpServer) -> Self {
        Self(Arc::new(inner))
    }
}

impl ServerHandler for SingletonMcpServer {
    fn initialize(
        &self,
        request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<rmcp::model::InitializeResult, McpError>> + Send + '_
    {
        let inner = self.0.clone();
        async move { inner.initialize(request, context).await }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParam,
        context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
        let inner = self.0.clone();
        async move { inner.call_tool(request, context).await }
    }

    fn list_tools(
        &self,
        request: Option<PaginatedRequestParam>,
        context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        let inner = self.0.clone();
        async move { inner.list_tools(request, context).await }
    }
}
