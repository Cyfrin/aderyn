use crate::{
    ast::{NodeID, NodeType},
    context::{
        mcp::node_finder::render::{NodeInfo, NodeInfoBuilder},
        workspace::WorkspaceContext,
    },
};

#[inline]
pub fn get_matching_functions(idx: usize, context: &WorkspaceContext, term: &str) -> Vec<NodeInfo> {
    _get_nodes(idx, context, Some(term), NodeType::FunctionDefinition)
}

#[inline]
pub fn get_matching_modifiers(idx: usize, context: &WorkspaceContext, term: &str) -> Vec<NodeInfo> {
    _get_nodes(idx, context, Some(term), NodeType::ModifierDefinition)
}

#[inline]
pub fn get_matching_contracts(idx: usize, context: &WorkspaceContext, term: &str) -> Vec<NodeInfo> {
    _get_nodes(idx, context, Some(term), NodeType::ContractDefinition)
}

#[inline]
pub fn get_all_events(compilation_unit_index: usize, context: &WorkspaceContext) -> Vec<NodeInfo> {
    _get_nodes(compilation_unit_index, context, None, NodeType::EventDefinition)
}

#[inline]
pub fn get_all_errors(compilation_unit_index: usize, context: &WorkspaceContext) -> Vec<NodeInfo> {
    _get_nodes(compilation_unit_index, context, None, NodeType::ErrorDefinition)
}

fn _get_nodes(
    compilation_unit_index: usize,
    context: &WorkspaceContext,
    search_term: Option<&str>,
    ty: NodeType,
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

    match ty {
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
