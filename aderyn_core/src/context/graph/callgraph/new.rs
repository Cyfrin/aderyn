use std::collections::{hash_map::*, HashMap, HashSet};

use crate::{
    ast::{NodeID, NodeType},
    context::{
        browser::{
            ExtractFunctionCalls, ExtractModifierInvocations,
            GetClosestAncestorOfTypeX,
        },
        workspace::{ASTNode, WorkspaceContext},
    },
};

pub(super) fn derive_inward_surface_points(
    context: &WorkspaceContext,
    nodes: &[&ASTNode],
) -> HashMap<NodeID, HashSet<NodeID>> {
    // key => base contract IDs
    // value => set of callgraph entrypoints
    let mut potential: HashMap<NodeID, HashSet<NodeID>> = Default::default();

    // TODO:
    // Key => Node ID
    // Value => Set of contracts where it's valid to visit CallgraphConsumer::entry_points

    let containing_function_or_modifier = |node: &ASTNode| -> Option<NodeID> {
        if matches!(node.node_type(), NodeType::FunctionDefinition | NodeType::ModifierDefinition) {
            return node.id();
        }
        node.closest_ancestor_of_type(context, NodeType::FunctionDefinition)
            .or_else(|| node.closest_ancestor_of_type(context, NodeType::ModifierDefinition))?
            .id()
    };

    for &node in nodes {
        let Some(containing_fm) = containing_function_or_modifier(node) else {
            continue;
        };
        let function_calls = ExtractFunctionCalls::from(node).extracted;
        let modifier_calls = ExtractModifierInvocations::from(node).extracted;

        let cg = context.callgraphs.as_ref().expect("callgraph not found");
        for (contract_id, graph) in &cg.inward_callgraphs {
            let mut insert = |dest: NodeID| {
                match potential.entry(*contract_id) {
                    Entry::Occupied(mut o) => {
                        o.get_mut().insert(dest);
                    }
                    Entry::Vacant(v) => {
                        let mut points = HashSet::new();
                        points.insert(dest);
                        v.insert(points);
                    }
                };
            };
            if let Some(ASTNode::ContractDefinition(contract)) = context.nodes.get(contract_id) {
                if graph.contains_key(&containing_fm) {
                    for function_call in function_calls.iter() {
                        if let Some(f) = context.resolve_internal_call(contract, function_call) {
                            insert(f.id);
                        }
                    }
                    for modifier_call in modifier_calls.iter() {
                        if let Some(m) = context.resolve_modifier_call(contract, modifier_call) {
                            insert(m.id);
                        }
                    }
                }
            }
        }
    }
    potential
}
