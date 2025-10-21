use crate::context::{
    macros::{mcp_error, mcp_success},
    mcp::{
        MCPToolNamePool, ModelContextProtocolState, ModelContextProtocolTool,
        node_finder::{render::*, utils::*},
    },
};
use indoc::indoc;
use regex::Regex;
use rmcp::{
    ErrorData as McpError, handler::server::wrapper::Parameters, model::CallToolResult, schemars,
};
use serde::Deserialize;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone)]
pub struct NodeFinderTool {
    state: Arc<ModelContextProtocolState>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeFinderPayload {
    /// Search function nodes by the exact function name
    #[serde(rename = "search_functions_by_exact_name")]
    function_name: Option<String>,
    /// Search modifier nodes by the exact modifier name
    #[serde(rename = "search_modifiers_by_exact_name")]
    modifier_name: Option<String>,
    /// Search contract class nodes by the exact contract class name
    #[serde(rename = "search_contract_classes_by_exact_name")]
    contract_class_name: Option<String>,
    /// Grep for symbols and references by using regular expressions.
    /// Input just the bare regex pattern, not wrapped in / or quotes.
    #[serde(rename = "search_nodes_by_grep")]
    regex_term: Option<String>,
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
    SearchNodesByRegex(String),
    GetAllEvents,
    GetAllErrors,
}

impl SearchType {
    fn get_search_name(&self) -> Option<String> {
        match self {
            SearchType::SearchFunctionsByName(name)
            | SearchType::SearchModifiersByName(name)
            | SearchType::SearchContractsByName(name) => Some(name.clone()),
            SearchType::GetAllEvents
            | SearchType::GetAllErrors
            | SearchType::SearchNodesByRegex(_) => None,
        }
    }
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
            "Retrieve nodes IDs and compilation unit indexes of nodes matched by either supplying the exact\
            names of functions, modifiers and contracts or grep them with a regular expression.\
            Important: Input only 1 search field (chose from functions, modifiers, contracts, events, errors and grep) \
            Optionally also input 'compilation_unit_index' to limit the search to a specific compilation unit."
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

        let received_search_opts: Vec<SearchType> = extract_search_options_from_payload(&payload);

        if received_search_opts.is_empty() {
            return mcp_error!(
                "Choose a single search option from contract, function, modifier, errors and events. None received"
            );
        }

        if received_search_opts.len() > 1 {
            return mcp_error!(
                "Choose a single search option from contract, function, modifier, errors and events. Multiple received"
            );
        }

        if let Some(Err(e)) = payload.regex_term.map(|regex_term| Regex::new(&regex_term)) {
            return mcp_error!("Invalid regex passed. Error: {}", e);
        }

        let search_term =
            received_search_opts.first().expect("no checks to ensure 1 received search option");

        if let Some(name) = search_term.get_search_name()
            && !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return mcp_error!(
                "To pass a regular expression as a search term, use the search by grep option"
            )
        }

        // Nodes whose names exactly match with the input term.
        let mut matching_contracts = vec![];
        let mut matching_functions = vec![];
        let mut matching_modifiers = vec![];

        // Nodes whose code snippet matches the grep test.
        let mut grepped_functions = vec![];
        let mut grepped_modifiers = vec![];
        let mut grepped_state_variables = vec![];

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
                    SearchType::SearchContractsByName(name)
                    | SearchType::SearchFunctionsByName(name)
                    | SearchType::SearchModifiersByName(name) => {
                        matching_contracts.extend(get_matching_contracts(i + 1, context, name));
                        matching_functions.extend(get_matching_functions(i + 1, context, name));
                        matching_modifiers.extend(get_matching_modifiers(i + 1, context, name));
                    }
                    SearchType::GetAllEvents | SearchType::GetAllErrors => {
                        events.extend(get_all_events(i + 1, context));
                        errors.extend(get_all_errors(i + 1, context));
                    }
                    SearchType::SearchNodesByRegex(term) => {
                        grepped_functions.extend(grep_functions(i + 1, context, term));
                        grepped_modifiers.extend(grep_modifiers(i + 1, context, term));
                        grepped_state_variables.extend(grep_state_variables(i + 1, context, term));
                    }
                }
            }
        }

        match search_term {
            SearchType::SearchContractsByName(name) => {
                mcp_success!(node_finder_matches(name, matching_contracts, "Contract"))
            }
            SearchType::SearchFunctionsByName(name) => {
                mcp_success!(node_finder_matches(name, matching_functions, "Function"))
            }
            SearchType::SearchModifiersByName(name) => {
                mcp_success!(node_finder_matches(name, matching_modifiers, "Modifier"))
            }
            SearchType::GetAllEvents => mcp_success!(node_finder_all(events, "Event")),
            SearchType::GetAllErrors => mcp_success!(node_finder_all(errors, "Error")),
            SearchType::SearchNodesByRegex(term) => {
                mcp_success!(node_grep_matches(
                    term,
                    grepped_state_variables,
                    grepped_functions,
                    grepped_modifiers
                ))
            }
        }
    }
}

fn extract_search_options_from_payload(payload: &NodeFinderPayload) -> Vec<SearchType> {
    // Keep the string if it's non empty after trimming
    let sanitize = |opt: &Option<String>| opt.as_ref().filter(|s| !s.trim().is_empty()).cloned();

    [
        payload.get_all_errors.filter(|&enabled| enabled).map(|_| SearchType::GetAllErrors),
        payload.get_all_events.filter(|&enabled| enabled).map(|_| SearchType::GetAllEvents),
        sanitize(&payload.regex_term).map(SearchType::SearchNodesByRegex),
        sanitize(&payload.contract_class_name).map(SearchType::SearchContractsByName),
        sanitize(&payload.function_name).map(SearchType::SearchFunctionsByName),
        sanitize(&payload.modifier_name).map(SearchType::SearchModifiersByName),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn node_finder_matches(term: &str, nodes: Vec<NodeInfo>, ty: &str) -> NodeFinderMatches {
    NodeFinderMatchesBuilder::default()
        .term(term.to_string())
        .matching_nodes(nodes)
        .node_type(ty.to_string())
        .build()
        .expect("failed to build renderer for node finder")
}

fn node_grep_matches(
    term: &str,
    state_vars: Vec<NodeInfo>,
    functions: Vec<NodeInfo>,
    modifiers: Vec<NodeInfo>,
) -> NodeFinderGrepMatches {
    let nodes = BTreeMap::from_iter(vec![
        ("State Variable".to_string(), state_vars),
        ("Function".to_string(), functions),
        ("Modifier".to_string(), modifiers),
    ]);
    NodeFinderGrepMatchesBuilder::default()
        .term(term.to_string())
        .nodes(nodes)
        .build()
        .expect("failed to build node finder grep matches")
}

fn node_finder_all(nodes: Vec<NodeInfo>, ty: &str) -> NodeFinderAll {
    NodeFinderAllBuilder::default()
        .nodes(nodes)
        .node_type(ty.to_string())
        .build()
        .expect("failed to build renderer for node finder")
}
