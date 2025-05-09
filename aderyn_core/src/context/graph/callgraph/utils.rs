
use crate::context::workspace::{ASTNode, WorkspaceContext};

use super::*;

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
        let mut consumers = vec![];

        let entry_points = legacy::derive_entry_points(nodes)?;
        let outward_surface_points = legacy::derive_outward_surface_points(context, nodes);
        let inward_surface_pointss = new::derive_inward_surface_points(context, nodes);

        if inward_surface_pointss.is_empty() {
        } else {
            for (contract_id, inward_surface_points) in inward_surface_pointss {
                consumers.push(CallGraphConsumer {
                    entry_points: entry_points.clone(), // maybe not valid
                    inward_surface_points: inward_surface_points.into_iter().collect(),
                    outward_surface_points: outward_surface_points.clone(),
                    direction: direction.clone(),
                    base_contract: Some(contract_id),
                });
            }
        }

        Ok(consumers)
    }
}
