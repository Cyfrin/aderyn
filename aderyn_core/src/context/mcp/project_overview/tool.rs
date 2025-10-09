use super::render;
use crate::context::{
    macros::mcp_success,
    mcp::{
        MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool,
        project_overview::render::*,
    },
};
use indoc::indoc;
use rmcp::{ErrorData as McpError, handler::server::wrapper::Parameters, model::CallToolResult};
use std::{path::PathBuf, str::FromStr, sync::Arc};

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
        indoc! {
            "It returns project configuration such as the root directory, source directory \
            (where the contracts are kept), 3rd party libraries, remappings, list of source \
            files, user preference for included and excluded files, etc."
        }
        .to_string()
    }

    fn execute(&self, _input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        // remappings
        let mut remapping_strings = vec![];
        for r in self.state.project_config.project_paths.remappings.iter() {
            remapping_strings.push(r.to_string());
        }
        // compilation units
        let mut compilation_units = vec![];
        for context in self.state.contexts.iter() {
            let mut file_entries = vec![];
            let mut included_count = 0;
            for file in context.src_filepaths.iter() {
                let file_entry;
                if context.included.contains(&PathBuf::from_str(file).unwrap()) {
                    file_entry = FileEntryBuilder::default()
                        .path(file.clone())
                        .included(true)
                        .build()
                        .map_err(|_| {
                            McpError::internal_error("failed to build file entry", None)
                        })?;
                    included_count += 1;
                } else {
                    file_entry = FileEntryBuilder::default()
                        .path(file.clone())
                        .included(false)
                        .build()
                        .map_err(|_| {
                            McpError::internal_error("failed to build file entry", None)
                        })?;
                }
                file_entries.push(file_entry);
            }
            let compilation_unit = CompilationUnitBuilder::default()
                .files(file_entries)
                .included_count(included_count)
                .build()
                .map_err(|_| McpError::internal_error("failed to build compilation unit", None))?;
            compilation_units.push(compilation_unit);
        }

        let project_overview = render::ProjectOverviewBuilder::default()
            .root(self.state.root_path.to_string_lossy().to_string())
            .source(self.state.project_config.project_paths.sources.to_string_lossy().to_string())
            .remappings(remapping_strings)
            .compilation_units(compilation_units)
            .build()
            .expect("failed to build project overview");

        mcp_success!(project_overview)
    }
}
