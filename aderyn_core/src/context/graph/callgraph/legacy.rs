use crate::{
    ast::{NodeID, NodeType},
    context::{
        browser::{ExtractReferencedDeclarations, GetClosestAncestorOfTypeX},
        workspace::{ASTNode, WorkspaceContext},
    },
};

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
