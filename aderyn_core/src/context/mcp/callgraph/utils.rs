use super::render::*;
use crate::{
    ast::{ASTNode, ContractDefinition, FunctionDefinition, NodeID},
    context::{graph::RawCallGraph, workspace::WorkspaceContext},
};
use rmcp::ErrorData as McpError;
use std::collections::{HashSet, hash_map::Entry};

pub fn build_post_order_nodes(
    context: &WorkspaceContext,
    subgraph: &RawCallGraph,
    entrypoint: &FunctionDefinition,
) -> Result<Vec<NodeData>, McpError> {
    let mut visited: HashSet<NodeID> = Default::default();
    let mut post_order_nodes = vec![];

    fn post_order_traverse(
        context: &WorkspaceContext,
        node_id: NodeID,
        post_order_nodes: &mut Vec<NodeData>,
        visited: &mut HashSet<NodeID>,
        subgraph: &RawCallGraph,
    ) {
        if visited.contains(&node_id) {
            return;
        }
        visited.insert(node_id);

        if let Some(to_node_ids) = subgraph.get(&node_id) {
            for to_id in to_node_ids {
                post_order_traverse(context, *to_id, post_order_nodes, visited, subgraph);
            }
        }

        let name = match context.nodes.get(&node_id) {
            Some(ASTNode::FunctionDefinition(func)) => Some(func.name.clone()),
            Some(ASTNode::ModifierDefinition(modifier)) => Some(modifier.name.clone()),
            _ => None,
        };

        let Some(name) = name else {
            return;
        };

        let mut called_nodes = vec![];
        if let Some(called) = subgraph.get(&node_id) {
            for c in called {
                let c_name = match context.nodes.get(c) {
                    Some(ASTNode::FunctionDefinition(func)) => Some(func.name.clone()),
                    Some(ASTNode::ModifierDefinition(modifier)) => Some(modifier.name.clone()),
                    _ => None,
                };

                let Some(c_name) = c_name else {
                    continue;
                };

                if let Ok(c_data) = NodeDataBuilder::default()
                    .node_id(*c)
                    .name(c_name)
                    .called_nodes(vec![]) // When displaying we only show one level deep.
                    .build()
                {
                    called_nodes.push(c_data);
                }
            }
        }

        if let Ok(node_data) = NodeDataBuilder::default()
            .node_id(node_id)
            .name(name)
            .called_nodes(called_nodes)
            .build()
        {
            post_order_nodes.push(node_data);
        }
    }

    post_order_traverse(context, entrypoint.id, &mut post_order_nodes, &mut visited, subgraph);

    Ok(post_order_nodes)
}

pub fn build_raw_callgraph_for_entrypoint(
    context: &WorkspaceContext,
    contract: &ContractDefinition,
    entrypoint: &FunctionDefinition,
) -> Result<RawCallGraph, McpError> {
    let Some(callgraphs) = &context.callgraphs else {
        return Err(McpError::internal_error("callgraphs could not be formed", None));
    };
    let Some(callgraph) = callgraphs.inward_callgraphs.get(&contract.id) else {
        return Err(McpError::internal_error(
            format!("callgraph for {} doesn't exist.", contract.id),
            None,
        ));
    };

    let mut visited: HashSet<NodeID> = Default::default();
    let mut subgraph = RawCallGraph::default();

    let mut worklist = vec![entrypoint.id];

    while let Some(node_id) = worklist.pop() {
        if visited.contains(&node_id) {
            continue;
        }
        visited.insert(node_id);
        create_node_if_not_exists(node_id, &mut subgraph);

        if let Some(to_nodes) = callgraph.get(&node_id) {
            for to_id in to_nodes {
                create_node_if_not_exists(*to_id, &mut subgraph);
                create_connection_if_not_exists(node_id, *to_id, &mut subgraph);
                // Only explore further if it hasn't already been explored.
                if !visited.contains(to_id) {
                    worklist.push(*to_id);
                }
            }
        }
    }
    Ok(subgraph)
}

#[inline(always)]
fn create_node_if_not_exists(node_id: NodeID, raw_callgraph: &mut RawCallGraph) {
    if let Entry::Vacant(v) = raw_callgraph.entry(node_id) {
        v.insert(vec![]);
    }
}

#[inline]
fn create_connection_if_not_exists(
    from_id: NodeID,
    to_id: NodeID,
    raw_callgraph: &mut RawCallGraph,
) {
    match raw_callgraph.entry(from_id) {
        Entry::Occupied(mut o) => {
            if !o.get().contains(&to_id) {
                o.get_mut().push(to_id);
            }
        }
        Entry::Vacant(v) => {
            v.insert(vec![to_id]);
        }
    }
}
