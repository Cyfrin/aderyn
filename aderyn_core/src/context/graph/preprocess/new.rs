use crate::{
    ast::ContractDefinition,
    context::{
        browser::{ExtractFunctionCalls, ExtractModifierInvocations},
        graph::*,
        workspace::WorkspaceContext,
    },
};
use std::collections::{hash_map::*, HashSet};

impl WorkspaceCallGraphs {
    pub fn build(context: &WorkspaceContext) -> WorkspaceCallGraphs {
        let mut workspace_cg: WorkspaceCallGraphs = Default::default();
        for contract in context.deployable_contracts() {
            if let Some(raw_callgraph) = _create_raw_callgraph(context, contract) {
                workspace_cg.inward_callgraphs.insert(contract.id, raw_callgraph.clone());
                workspace_cg.outward_callgraphs.insert(contract.id, raw_callgraph.reverse());
            }
        }
        workspace_cg
    }
}

pub fn _create_raw_callgraph(
    context: &WorkspaceContext,
    contract: &ContractDefinition,
) -> Option<RawCallGraph> {
    let mut raw_callgraph = Default::default();
    let mut visited: HashSet<NodeID> = Default::default();
    for entrypoint in context.entrypoint_functions(contract)? {
        let mut current = vec![entrypoint.id];
        while let Some(node_id) = current.pop() {
            if visited.contains(&node_id) {
                continue;
            }
            visited.insert(node_id);
            create_node_if_not_exists(node_id, &mut raw_callgraph);
            let Some(node) = context.nodes.get(&node_id) else {
                continue;
            };
            for function_call in ExtractFunctionCalls::from(node).extracted {
                if let Some(f) = context.resolve_internal_call(contract, &function_call) {
                    create_connection_if_not_exists(node_id, f.id, &mut raw_callgraph);
                    current.push(f.id);
                }
            }
            for modifier_call in ExtractModifierInvocations::from(node).extracted {
                if let Some(m) = context.resolve_modifier_call(contract, &modifier_call) {
                    create_connection_if_not_exists(node_id, m.id, &mut raw_callgraph);
                    current.push(m.id);
                }
            }
        }
    }
    Some(raw_callgraph)
}

fn create_node_if_not_exists(node_id: NodeID, raw_callgraph: &mut RawCallGraph) {
    if let Entry::Vacant(v) = raw_callgraph.entry(node_id) {
        v.insert(vec![]);
    }
}

fn create_connection_if_not_exists(
    from_id: NodeID,
    to_id: NodeID,
    raw_callgraph: &mut RawCallGraph,
) {
    match raw_callgraph.entry(from_id) {
        Entry::Occupied(mut o) => {
            // Performance Tip: Maybe later use binary search (it requires keeping ascending order
            // while inserting tho)
            if !o.get().contains(&to_id) {
                o.get_mut().push(to_id);
            }
        }
        Entry::Vacant(v) => {
            v.insert(vec![to_id]);
        }
    }
}
