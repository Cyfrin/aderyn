use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    ErrorData as McpError,
};
use std::sync::Arc;

use crate::context::mcp::{MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool};

#[derive(Clone)]
pub struct ProjectOverviewTool {
    state: Arc<ModelContextProtocolState>,
}

impl ModelContextProtocolTool for ProjectOverviewTool {
    type Input = rmcp::model::EmptyObject;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynGetProjectOverview.to_string()
    }

    fn description(&self) -> String {
        "Calling this tool once at the start is the best way to equip yourself with the base knowledge needed to help the user. It returns project configuration such as the root directory, source directory (where the contracts are kept), 3rd party libraries, remappings, list of source files, user preference for included and excluded files, etc. Also tells you what other tools to call following this".to_string()
    }

    fn execute(&self, _input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        let message = format!("The root is {}", self.state.root_path.to_string_lossy().to_string());
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }
}
