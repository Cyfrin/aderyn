use crate::context::mcp::{MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool};
use indoc::indoc;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars, ErrorData as McpError,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct ListContractsTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ListContractsPayload {
    /// The index of the compilation unit to analyze. Must be a positive integer starting from 1.
    /// Use the project overview tool first to see all available compilation units and their
    /// indices.
    pub compilation_unit_index: usize,
}

impl ModelContextProtocolTool for ListContractsTool {
    type Input = ListContractsPayload;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynListContracts.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            "Enumerates deployable contracts within a specific compilation unit. Returns contract names, \
            file names (relative to the project root) and node IDs."
        }
        .to_string()
    }

    fn execute(&self, input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        if let Some(context) = self.state.contexts.get(input.0.compilation_unit_index) {
            return Ok(CallToolResult::success(vec![Content::text("TODO")]));
        }
        Ok(CallToolResult::error(vec![Content::text(format!(
            "invalid value passed for compilation unit - must be in the range [1, {}] inclusive",
            self.state.contexts.len()
        ))]))
    }
}
