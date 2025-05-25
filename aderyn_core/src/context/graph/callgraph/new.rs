use crate::{
    ast::{NodeID, NodeType},
    context::{
        browser::{ExtractFunctionCalls, ExtractModifierInvocations, GetClosestAncestorOfTypeX},
        workspace::{ASTNode, WorkspaceContext},
    },
};
use std::collections::{hash_map::*, HashMap, HashSet};

#[derive(Debug, Default)]
pub struct CallgraphExplorationPoints {
    pub points: HashMap<NodeID, GraphPoints>,
}

#[derive(Debug, Default)]
pub struct GraphPoints {
    pub entry: HashSet<NodeID>,
    pub inward: HashSet<NodeID>,
    pub outward: HashSet<NodeID>,
}

pub enum GraphPointType {
    Entry,
    Inward,
    Outward,
}

impl CallgraphExplorationPoints {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add(&mut self, graph_type: GraphPointType, contract_id: NodeID, point: NodeID) {
        match self.points.entry(contract_id) {
            Entry::Occupied(mut o) => match graph_type {
                GraphPointType::Entry => {
                    o.get_mut().entry.insert(point);
                }
                GraphPointType::Inward => {
                    o.get_mut().inward.insert(point);
                }
                GraphPointType::Outward => {
                    o.get_mut().outward.insert(point);
                }
            },
            Entry::Vacant(v) => {
                v.insert(Default::default());
                self.add(graph_type, contract_id, point);
            }
        };
    }
}

pub(super) fn derive_surface_points(
    context: &WorkspaceContext,
    nodes: &[&ASTNode],
) -> CallgraphExplorationPoints {
    let mut cg_points = CallgraphExplorationPoints::new();

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
            let Some(ASTNode::ContractDefinition(contract)) = context.nodes.get(contract_id) else {
                continue;
            };
            if graph.contains_key(&containing_fm) {
                // Entry
                if let Some(node_id) = node.id() {
                    cg_points.add(GraphPointType::Entry, *contract_id, node_id);
                }
                // Inward
                for function_call in function_calls.iter() {
                    if let Some(f) = context.resolve_internal_call(contract, function_call) {
                        cg_points.add(GraphPointType::Inward, *contract_id, f.id);
                    }
                }
                for modifier_call in modifier_calls.iter() {
                    if let Some(m) = context.resolve_modifier_call(contract, modifier_call) {
                        cg_points.add(GraphPointType::Inward, *contract_id, m.id);
                    }
                }
                // Outward
                cg_points.add(GraphPointType::Outward, *contract_id, containing_fm);
            }
        }
    }
    cg_points
}
