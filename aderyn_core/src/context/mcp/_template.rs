use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars, ErrorData as McpError,
};
use std::sync::Arc;

use crate::context::mcp::{MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool};

#[derive(Clone)]
pub struct ProjectOverviewTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SingleValueInput {
    a: String,
}

impl ModelContextProtocolTool for ProjectOverviewTool {
    type Input = SingleValueInput;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::SAMPLE.to_string()
    }

    fn description(&self) -> String {
        "SAMPLE".to_string()
    }

    fn execute(&self, _input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        let message = "REPLACE ME";
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }
}
