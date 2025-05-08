use std::collections::{hash_map::*, HashMap, HashSet};

use crate::{
    ast::{NodeID, NodeType},
    context::{
        browser::{
            ExtractFunctionCalls, ExtractModifierInvocations, ExtractReferencedDeclarations,
            GetClosestAncestorOfTypeX,
        },
        workspace::{ASTNode, WorkspaceContext},
    },
};

use super::*;

impl CallGraphConsumer {
    /// Legacy method
    ///
    /// Creates a [`CallGraphConsumer`] that can explore paths from given nodes.
    pub fn from_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<CallGraphConsumer> {
        Ok(CallGraphConsumer {
            entry_points: derive_entry_points(nodes)?,
            inward_surface_points: derive_inward_surface_points_legacy(context, nodes),
            outward_surface_points: derive_outward_surface_points(context, nodes),
            direction,
            base_contract: None,
        })
    }

    /// New method
    ///
    /// Creates a [`CallGraphConsumer`] that can explore paths from given nodes.
    pub fn many_from_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<Vec<CallGraphConsumer>> {
        let mut consumers = vec![];

        let entry_points = derive_entry_points(nodes)?;
        let outward_surface_points = derive_outward_surface_points(context, nodes);

        let inward_surface_pointss = derive_inward_surface_points(context, nodes);
        for (contract_id, inward_surface_points) in inward_surface_pointss {
            consumers.push(CallGraphConsumer {
                entry_points: entry_points.clone(),
                inward_surface_points: inward_surface_points.into_iter().collect(),
                outward_surface_points: outward_surface_points.clone(),
                direction: direction.clone(),
                base_contract: Some(contract_id),
            });
        }

        Ok(consumers)
    }
}

pub(super) fn derive_outward_surface_points(
    context: &WorkspaceContext,
    nodes: &[&ASTNode],
) -> Vec<NodeID> {
    let mut outward_surface_points = vec![];
    for &node in nodes {
        if node.node_type() == NodeType::FunctionDefinition
            || node.node_type() == NodeType::ModifierDefinition
        {
            if let Some(id) = node.id() {
                outward_surface_points.push(id);
            }
        } else {
            let parent_surface_point = node
                .closest_ancestor_of_type(context, NodeType::FunctionDefinition)
                .or_else(|| node.closest_ancestor_of_type(context, NodeType::ModifierDefinition));
            if let Some(parent_surface_point) = parent_surface_point {
                if let Some(parent_surface_point_id) = parent_surface_point.id() {
                    outward_surface_points.push(parent_surface_point_id);
                }
            }
        }
    }
    outward_surface_points
}

pub(super) fn derive_entry_points(nodes: &[&ASTNode]) -> super::Result<Vec<NodeID>> {
    let mut entry_points = vec![];
    for &node in nodes {
        let node_id =
            node.id().ok_or_else(|| super::Error::UnidentifiedEntryPointNode(node.clone()))?;
        entry_points.push(node_id);
    }
    Ok(entry_points)
}

pub(super) fn derive_inward_surface_points_legacy(
    context: &WorkspaceContext,
    nodes: &[&ASTNode],
) -> Vec<NodeID> {
    let mut inward_surface_points = vec![];

    // Construct inward surface points
    for &node in nodes {
        let referenced_declarations = ExtractReferencedDeclarations::from(node).extracted;

        for declared_id in referenced_declarations {
            if let Some(node) = context.nodes.get(&declared_id) {
                if node.node_type() == NodeType::ModifierDefinition {
                    inward_surface_points.push(declared_id);
                } else if let ASTNode::FunctionDefinition(function_definition) = node {
                    if function_definition.implemented {
                        inward_surface_points.push(declared_id);
                    }
                }
            }
        }
    }

    inward_surface_points
}

pub(super) fn derive_inward_surface_points(
    context: &WorkspaceContext,
    nodes: &[&ASTNode],
) -> HashMap<NodeID, HashSet<NodeID>> {
    // key => base ontract IDs
    // value => set of callgraph entrypoints
    let mut potential: HashMap<NodeID, HashSet<NodeID>> = Default::default();

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
            let mut insert = |contract_id: NodeID, dest: NodeID| {
                match potential.entry(contract_id) {
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
                            insert(*contract_id, f.id);
                        }
                    }
                    for modifier_call in modifier_calls.iter() {
                        if let Some(m) = context.resolve_modifier_call(contract, modifier_call) {
                            insert(*contract_id, m.id);
                        }
                    }
                }
            }
        }
    }
    potential
}
