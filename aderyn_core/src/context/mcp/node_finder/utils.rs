use crate::{
    ast::{NodeID, NodeType},
    context::{
        mcp::node_finder::render::{NodeInfo, NodeInfoBuilder},
        workspace::WorkspaceContext,
    },
};

// Matches functions, modifiers and contracts by their exact names.

#[inline]
pub fn get_matching_functions(idx: usize, context: &WorkspaceContext, name: &str) -> Vec<NodeInfo> {
    match_nodes_bt_exact_name(idx, context, Some(name), NodeType::FunctionDefinition)
}

#[inline]
pub fn get_matching_modifiers(idx: usize, context: &WorkspaceContext, name: &str) -> Vec<NodeInfo> {
    match_nodes_bt_exact_name(idx, context, Some(name), NodeType::ModifierDefinition)
}

#[inline]
pub fn get_matching_contracts(idx: usize, context: &WorkspaceContext, name: &str) -> Vec<NodeInfo> {
    match_nodes_bt_exact_name(idx, context, Some(name), NodeType::ContractDefinition)
}

// Matches all events and errors.

#[inline]
pub fn get_all_events(idx: usize, context: &WorkspaceContext) -> Vec<NodeInfo> {
    match_nodes_bt_exact_name(idx, context, None, NodeType::EventDefinition)
}

#[inline]
pub fn get_all_errors(idx: usize, context: &WorkspaceContext) -> Vec<NodeInfo> {
    match_nodes_bt_exact_name(idx, context, None, NodeType::ErrorDefinition)
}

// Matches functions, modifiers and state variables whose code snippets match a given regex

#[inline]
pub fn grep_functions(idx: usize, context: &WorkspaceContext, term: &str) -> Vec<NodeInfo> {
    todo!()
}

#[inline]
pub fn grep_modifiers(idx: usize, context: &WorkspaceContext, term: &str) -> Vec<NodeInfo> {
    todo!()
}

#[inline]
pub fn grep_state_variables(idx: usize, context: &WorkspaceContext, term: &str) -> Vec<NodeInfo> {
    todo!()
}

// Helper functions

fn match_nodes_bt_exact_name(
    compilation_unit_index: usize,
    context: &WorkspaceContext,
    search_term: Option<&str>,
    node_ty: NodeType,
) -> Vec<NodeInfo> {
    let mut matching_nodes = vec![];

    let mut add_node = |name: &str, id: NodeID| {
        if let Ok(node_info) = NodeInfoBuilder::default()
            .name(name.to_string())
            .node_id(id)
            .compilation_unit_index(compilation_unit_index)
            .build()
        {
            matching_nodes.push(node_info);
        }
    };

    match node_ty {
        NodeType::ContractDefinition => {
            context
                .contract_definitions()
                .iter()
                .filter(|m| search_term.is_none_or(|t| t == m.name))
                .for_each(|m| add_node(&m.name, m.id));
        }
        NodeType::FunctionDefinition => {
            context
                .function_definitions()
                .iter()
                .filter(|m| search_term.is_none_or(|t| t == m.name))
                .for_each(|m| add_node(&m.name, m.id));
        }
        NodeType::ModifierDefinition => {
            context
                .modifier_definitions()
                .iter()
                .filter(|m| search_term.is_none_or(|t| t == m.name))
                .for_each(|m| add_node(&m.name, m.id));
        }
        _ => {}
    };
    matching_nodes
}
