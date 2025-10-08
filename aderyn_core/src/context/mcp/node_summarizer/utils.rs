use crate::{
    ast::{ASTNode, NodeID, NodeType},
    context::{
        graph::RawCallGraph,
        mcp::node_summarizer::render::{
            EntrypointCallgraphInfo, EntrypointCallgraphInfoBuilder, NodeInfo, NodeInfoBuilder,
        },
        workspace::WorkspaceContext,
    },
};
use std::collections::HashSet;

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

pub fn get_containing_callgraphs(
    compilation_unit_index: usize,
    context: &WorkspaceContext,
    node: &ASTNode,
) -> Vec<EntrypointCallgraphInfo> {
    // Given node, we want to locate it in the callgraph
    let node_id = node.id().expect("node found without an ID");

    let parent_graph_node = match (
        context.get_closest_ancestor_including_self(node_id, NodeType::FunctionDefinition),
        context.get_closest_ancestor_including_self(node_id, NodeType::ModifierDefinition),
    ) {
        (Some(ASTNode::FunctionDefinition(func)), _) => Some(func.id),
        (_, Some(ASTNode::ModifierDefinition(modifier))) => Some(modifier.id),
        (_, _) => None,
    };

    let Some(parent_graph_node) = parent_graph_node else {
        return vec![];
    };

    let mut entrypoint_callgraph_info = vec![];

    for contract in context.deployable_contracts() {
        let Some(contract_callgraph) =
            context.callgraphs.as_ref().and_then(|c| c.outward_callgraphs.get(&contract.id))
        else {
            continue;
        };
        let Some(entrypoint_ids) = contract
            .entrypoint_functions(context)
            .and_then(|funcs| Some(funcs.into_iter().map(|f| f.id).collect::<HashSet<_>>()))
        else {
            continue;
        };
        let reachable_entrypoints = traverse_cg_and_get_reachable_entrypoints(
            parent_graph_node,
            contract_callgraph,
            &entrypoint_ids,
        );

        for entrypoint_id in reachable_entrypoints {
            let e = EntrypointCallgraphInfoBuilder::default()
                .compilation_unit_index(compilation_unit_index)
                .deployable_contract_id(contract.id)
                .entrypoint_function_id(entrypoint_id)
                .build()
                .expect("failed to build node info");
            entrypoint_callgraph_info.push(e);
        }
    }
    entrypoint_callgraph_info
}

fn traverse_cg_and_get_reachable_entrypoints(
    node_id: NodeID,
    outward_cg: &RawCallGraph,
    entrypoint_ids: &HashSet<NodeID>,
) -> HashSet<NodeID> {
    // Visit all possible nodes starting from node_id in the outward callgraph. Then collect all the
    // nodes which can be potential starting points that lead to node_id in the (real) inward
    // callgraph.
    let mut worklist = vec![node_id];
    let mut visited: HashSet<NodeID> = Default::default();

    while let Some(node) = worklist.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        if let Some(connections) = outward_cg.get(&node) {
            for conn in connections {
                worklist.push(*conn);
            }
        }
    }

    visited.into_iter().filter(|f| entrypoint_ids.contains(f)).collect()
}
