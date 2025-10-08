use crate::{
    ast::{ASTNode, NodeID, NodeType},
    context::{
        mcp::node_summarizer::render::{NodeInfo, NodeInfoBuilder},
        workspace::WorkspaceContext,
    },
};

pub fn get_code_snippet(context: &WorkspaceContext, node: &ASTNode) -> String {
    let (filepath, _, src_location) = context.get_node_sort_key_pure(node);
    let source_unit = context
        .source_units()
        .into_iter()
        .find(|s| s.absolute_path.as_ref().is_some_and(|p| *p == filepath))
        .expect("node not found");

    let source_content = source_unit.source.as_ref().expect("source not found");

    let (byte_offset_str, byte_len_str) = src_location.split_once(':').unwrap();
    let byte_offset: usize = byte_offset_str.parse().unwrap();
    let byte_length: usize = byte_len_str.parse().unwrap();

    let code_snippet = &source_content[byte_offset..byte_offset + byte_length];
    code_snippet.to_owned()
}

pub fn get_containing_contract(context: &WorkspaceContext, node: &ASTNode) -> Option<NodeInfo> {
    if let ASTNode::ContractDefinition(_) = node {
        return None;
    }
    let Some(ASTNode::ContractDefinition(parent_contract)) = context.get_closest_ancestor(
        node.id().expect("node found without an ID"),
        NodeType::ContractDefinition,
    ) else {
        return None;
    };
    Some((parent_contract.id, parent_contract.name.clone()).into())
}

pub fn get_containing_modifier(context: &WorkspaceContext, node: &ASTNode) -> Option<NodeInfo> {
    if let ASTNode::ModifierDefinition(_) = node {
        return None;
    }
    let Some(ASTNode::ModifierDefinition(parent_modifier)) = context.get_closest_ancestor(
        node.id().expect("node found without an ID"),
        NodeType::ModifierDefinition,
    ) else {
        return None;
    };
    Some((parent_modifier.id, parent_modifier.name.clone()).into())
}

pub fn get_containing_function(context: &WorkspaceContext, node: &ASTNode) -> Option<NodeInfo> {
    if let ASTNode::FunctionDefinition(_) = node {
        return None;
    }
    let Some(ASTNode::FunctionDefinition(parent_function)) = context.get_closest_ancestor(
        node.id().expect("node found without an ID"),
        NodeType::FunctionDefinition,
    ) else {
        return None;
    };
    Some((parent_function.id, parent_function.name.clone()).into())
}

impl From<(NodeID, String)> for NodeInfo {
    fn from(value: (NodeID, String)) -> Self {
        NodeInfoBuilder::default()
            .name(value.1)
            .node_id(value.0)
            .build()
            .expect("failed to build node info")
    }
}
