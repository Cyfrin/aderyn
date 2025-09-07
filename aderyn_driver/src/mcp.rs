use rmcp::{
    handler::server::router::tool::ToolRouter, model::*, service::RequestContext, tool,
    tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};

use crate::process::WorkspaceContextWrapper;

pub struct McpServer {
    ctx_wrapper: WorkspaceContextWrapper,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl McpServer {
    pub fn new(ctx_wrapper: WorkspaceContextWrapper) -> Self {
        Self { tool_router: Self::tool_router(), ctx_wrapper }
    }

    #[tool(description = "Check project configuration and see details")]
    fn aderyn_project_config(&self) -> Result<CallToolResult, McpError> {
        let mut config = String::new();
        config.push_str(&format!(
            "Project root: {}\n\n",
            self.ctx_wrapper.root_path.to_string_lossy().to_string()
        ));
        config.push_str(&format!("Compilation units:\n\n"));
        for (i, c) in self.ctx_wrapper.contexts.iter().enumerate() {
            config.push_str(&format!("Unit {}\n\n", i + 1));
            for f in c.src_filepaths.iter() {
                config.push_str(&format!("{}\n", f));
            }
            config.push_str("\n\n");
        }

        config.push_str(
            "Each unit (or) context is a set of files that compiles with it's own solc version.",
        );
        Ok(CallToolResult::success(vec![Content::text(config)]))
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
            instructions: Some("Ask questions about the Solidity codebase".to_string()),
        })
    }
}
