use super::*;
use crate::context::workspace::{ASTNode, WorkspaceContext};

impl CallGraphConsumer {
    /// Legacy method
    ///
    /// Creates a [`CallGraphConsumer`] that can explore paths from given nodes.
    pub(super) fn from_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<CallGraphConsumer> {
        Ok(CallGraphConsumer {
            entry_points: legacy::derive_entry_points(nodes)?,
            inward_surface_points: legacy::derive_inward_surface_points_legacy(context, nodes),
            outward_surface_points: legacy::derive_outward_surface_points(context, nodes),
            direction,
            base_contract: None,
        })
    }

    /// New method
    ///
    /// Creates a [`CallGraphConsumer`] that can explore paths from given nodes.
    pub(super) fn many_from_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<Vec<CallGraphConsumer>> {
        let mut cg_consumers = vec![];
        let cg_points = new::derive_surface_points(context, nodes);
        for (contract_id, points) in cg_points.points {
            cg_consumers.push(CallGraphConsumer {
                entry_points: points.entry.into_iter().collect(),
                inward_surface_points: points.inward.into_iter().collect(),
                outward_surface_points: points.outward.into_iter().collect(),
                direction: direction.clone(),
                base_contract: Some(contract_id),
            });
        }
        Ok(cg_consumers)
    }
}
