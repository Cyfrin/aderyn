use crate::context::{
    macros::mcp_success,
    mcp::{MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool},
};
use indoc::indoc;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ToolGuide;

impl ModelContextProtocolTool for ToolGuide {
    type Input = rmcp::model::EmptyObject;

    fn new(_state: Arc<ModelContextProtocolState>) -> Self {
        Self
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynGetToolGuide.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            "MUST be called at least once at the beginning so as to have that base knowledge required to help come \
            up with solutions to user's problems. Provides glossary, general approaches to common scenarios, \
            advanced tool calling strategies and other adhoc tips to leverage Aderyn's MCP tools for \
            high performance and accuracy."
        }.to_string()
    }

    fn execute(&self, _input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        let response = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/mcp-tool-response/tool_guide.md"
        ));
        mcp_success!(response)
    }
}
