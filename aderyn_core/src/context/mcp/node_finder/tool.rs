use crate::context::{
    macros::{mcp_error, mcp_success},
    mcp::{
        MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool,
        node_finder::{
            render::{self, NodeFinderAll, NodeFinderMatches, NodeInfo},
            utils::*,
        },
    },
};
use indoc::indoc;
use rmcp::{
    ErrorData as McpError, handler::server::wrapper::Parameters, model::CallToolResult, schemars,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct NodeFinderTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeFinderPayload {
    /// Search function nodes by the exact function name
    search_functions_by_name: Option<String>,
    /// Search modifier nodes by the exact modifier name
    search_modifiers_by_name: Option<String>,
    /// Search contract class nodes by the exact contract class name
    search_contract_classes_by_name: Option<String>,
    /// Get all the event definitions
    get_all_events: Option<bool>,
    /// Get all the error definitions
    get_all_errors: Option<bool>,
    /// Optional compilation unit index helps restrict the scope of search to the given compilation
    /// unit.
    compilation_unit_index: Option<usize>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub enum SearchType {
    SearchFunctionsByName(String),
    SearchModifiersByName(String),
    SearchContractsByName(String),
    GetAllEvents,
    GetAllErrors,
}

impl ModelContextProtocolTool for NodeFinderTool {
    type Input = NodeFinderPayload;

    fn new(state: Arc<ModelContextProtocolState>) -> Self {
        Self { state }
    }

    fn name(&self) -> String {
        MCPToolNamePool::AderynNodeFinder.to_string()
    }

    fn description(&self) -> String {
        indoc! {
            "Retrieve nodes IDs and compilation unit indexes of node definitions matched by supplying the exact\
            names of functions, modifiers and contracts. Optionally accepts 'compilation_unit_index' to limit the \
            search to a specific compilation unit. Input only 1 field out of functions, modifiers, contracts, \
            events and errors. Also use the exact node names extracted from other tools. \
            Regex (or) regular expressions will not work."
        }
        .to_string()
    }

    fn execute(&self, input: Parameters<Self::Input>) -> Result<CallToolResult, McpError> {
        let payload = input.0;

        if let Some(compilation_unit_index) = payload.compilation_unit_index
            && (compilation_unit_index < 1 || compilation_unit_index > self.state.contexts.len())
        {
            return mcp_error!(
                "Invalid compilation unit index: {}. Must be in range [1, {}]",
                compilation_unit_index,
                self.state.contexts.len()
            );
        }

        // Get non empty string if possible otherwise return None
        let get_nes = |opt_str: Option<String>| -> Option<String> {
            match opt_str {
                Some(s) if !s.trim().is_empty() => Some(s),
                Some(_) | None => None,
            }
        };

        let search_term: SearchType = {
            if payload.get_all_errors.is_some_and(|f| f) {
                SearchType::GetAllErrors
            } else if payload.get_all_events.is_some_and(|f| f) {
                SearchType::GetAllEvents
            } else if let Some(contract_name) = get_nes(payload.search_contract_classes_by_name) {
                SearchType::SearchContractsByName(contract_name)
            } else if let Some(function_name) = get_nes(payload.search_functions_by_name) {
                SearchType::SearchFunctionsByName(function_name)
            } else if let Some(modifier_name) = get_nes(payload.search_modifiers_by_name) {
                SearchType::SearchModifiersByName(modifier_name)
            } else {
                return mcp_error!(
                    "Choose a single option from contract, function, modifier, errors and events"
                );
            }
        };

        let mut matching_contracts = vec![];
        let mut matching_functions = vec![];
        let mut matching_modifiers = vec![];

        let mut events = vec![];
        let mut errors = vec![];

        for (i, context) in self.state.as_ref().contexts.iter().enumerate() {
            let should_add;

            if let Some(compilation_unit_index) = payload.compilation_unit_index {
                should_add = i == compilation_unit_index - 1;
            } else {
                should_add = true;
            }

            if should_add {
                match search_term {
                    SearchType::SearchContractsByName(ref name)
                    | SearchType::SearchFunctionsByName(ref name)
                    | SearchType::SearchModifiersByName(ref name) => {
                        matching_contracts.extend(get_matching_contracts(i + 1, context, name));
                        matching_functions.extend(get_matching_functions(i + 1, context, name));
                        matching_modifiers.extend(get_matching_modifiers(i + 1, context, name));
                    }
                    SearchType::GetAllEvents | SearchType::GetAllErrors => {
                        events.extend(get_all_events(i + 1, context));
                        errors.extend(get_all_errors(i + 1, context));
                    }
                }
            }
        }

        match search_term {
            SearchType::SearchContractsByName(ref name) => {
                mcp_success!(render_text_for_matches(name, matching_contracts, "Contract"))
            }
            SearchType::SearchFunctionsByName(ref name) => {
                mcp_success!(render_text_for_matches(name, matching_functions, "Function"))
            }
            SearchType::SearchModifiersByName(ref name) => {
                mcp_success!(render_text_for_matches(name, matching_modifiers, "Modifier"))
            }
            SearchType::GetAllEvents => mcp_success!(render_text(events, "Event")),
            SearchType::GetAllErrors => mcp_success!(render_text(errors, "Error")),
        }
    }
}

fn render_text_for_matches(term: &str, nodes: Vec<NodeInfo>, ty: &str) -> NodeFinderMatches {
    render::NodeFinderMatchesBuilder::default()
        .term(term.to_string())
        .matching_nodes(nodes)
        .node_type(ty.to_string())
        .build()
        .expect("failed to build renderer for node finder")
}

fn render_text(nodes: Vec<NodeInfo>, ty: &str) -> NodeFinderAll {
    render::NodeFinderAllBuilder::default()
        .nodes(nodes)
        .node_type(ty.to_string())
        .build()
        .expect("failed to build renderer for node finder")
}
