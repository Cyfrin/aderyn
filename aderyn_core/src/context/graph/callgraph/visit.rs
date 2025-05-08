use std::collections::HashSet;

use crate::{
    ast::{NodeID, NodeType},
    context::{
        graph::RawCallGraph,
        workspace::{ASTNode, WorkspaceContext},
    },
};

use super::*;

impl CallGraphConsumer {
    /// Responsible for informing the trackers.
    /// First, we visit the entry points. Then, we derive the subgraph from the
    /// [`WorkspaceCallGraph`] which consists of all the nodes that can be reached by traversing
    /// the edges starting from the surface points.
    pub(super) fn _accept<T>(
        &self,
        context: &WorkspaceContext,
        visitor: &mut T,
    ) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        let callgraphs = context.callgraphs.as_ref().expect("callgraphs not attached to context");

        let inward_callgraph = {
            if self.is_legacy() {
                context
                    .inward_callgraph
                    .as_ref()
                    .ok_or(super::Error::InwardCallgraphNotAvailable)?
                    .raw_callgraph
                    .clone()
            } else {
                callgraphs
                    .inward_callgraphs
                    .get(&self.base_contract.expect("base contract not set"))
                    .ok_or(super::Error::InwardCallgraphNotAvailable)?
                    .clone()
            }
        };
        let outward_callgraph = {
            if self.is_legacy() {
                context
                    .outward_callgraph
                    .as_ref()
                    .ok_or(super::Error::OutwardCallgraphNotAvailable)?
                    .raw_callgraph
                    .clone()
            } else {
                callgraphs
                    .outward_callgraphs
                    .get(&self.base_contract.expect("base contract not set"))
                    .ok_or(super::Error::OutwardCallgraphNotAvailable)?
                    .clone()
            }
        };

        // Visit entry point nodes (so that trackers can track the state across all code regions in
        // 1 place)
        for entry_point_id in &self.entry_points {
            self.make_entry_point_visit_call(context, *entry_point_id, visitor)?;
        }

        // Keep track of visited node IDs during DFS from surface nodes
        let mut visited_inward = HashSet::new();
        let mut visited_outward = HashSet::new();
        let mut visited_outward_side_effects = HashSet::new();

        // Now decide, which points to visit outward or inward
        if self.direction == CallGraphDirection::BothWays
            || self.direction == CallGraphDirection::Inward
        {
            // Visit the subgraph starting from surface points
            for surface_point_id in &self.inward_surface_points {
                self.dfs_and_visit_subgraph(
                    *surface_point_id,
                    &mut visited_inward,
                    context,
                    &inward_callgraph,
                    visitor,
                    CurrentDFSVector::Inward,
                    None,
                )?;
            }
        }

        if self.direction == CallGraphDirection::BothWays
            || self.direction == CallGraphDirection::Outward
        {
            // Visit the subgraph starting from surface points
            for surface_point_id in &self.outward_surface_points {
                self.dfs_and_visit_subgraph(
                    *surface_point_id,
                    &mut visited_outward,
                    context,
                    &outward_callgraph,
                    visitor,
                    CurrentDFSVector::Outward,
                    None,
                )?;
            }
        }

        // Collect already visited nodes so that we don't repeat visit calls on them
        // while traversing through side effect nodes.
        let mut blacklisted = HashSet::new();
        blacklisted.extend(visited_inward.iter());
        blacklisted.extend(visited_outward.iter());
        blacklisted.extend(self.entry_points.iter());

        if self.direction == CallGraphDirection::BothWays {
            // Visit the subgraph from the outward points (go inward in inward graph)
            // but do not re-visit the outward nodes or the inward nodes again

            for surface_point_id in &visited_outward {
                self.dfs_and_visit_subgraph(
                    *surface_point_id,
                    &mut visited_outward_side_effects,
                    context,
                    &inward_callgraph,
                    visitor,
                    CurrentDFSVector::OutwardSideEffect,
                    Some(&blacklisted),
                )?;
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn dfs_and_visit_subgraph<T>(
        &self,
        node_id: NodeID,
        visited: &mut HashSet<NodeID>,
        context: &WorkspaceContext,
        callgraph: &RawCallGraph,
        visitor: &mut T,
        current_investigation_direction: CurrentDFSVector,
        blacklist: Option<&HashSet<NodeID>>,
    ) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        if visited.contains(&node_id) {
            return Ok(());
        }

        visited.insert(node_id);

        if let Some(blacklist) = blacklist {
            if !blacklist.contains(&node_id) {
                self.make_relevant_visit_call(
                    context,
                    node_id,
                    visitor,
                    current_investigation_direction,
                )?;
            }
        } else {
            self.make_relevant_visit_call(
                context,
                node_id,
                visitor,
                current_investigation_direction,
            )?;
        }

        if let Some(pointing_to) = callgraph.get(&node_id) {
            for destination in pointing_to {
                self.dfs_and_visit_subgraph(
                    *destination,
                    visited,
                    context,
                    callgraph,
                    visitor,
                    current_investigation_direction,
                    blacklist,
                )?;
            }
        }
        Ok(())
    }

    pub(super) fn make_relevant_visit_call<T>(
        &self,
        context: &WorkspaceContext,
        node_id: NodeID,
        visitor: &mut T,
        current_investigation_direction: CurrentDFSVector,
    ) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        if let Some(node) = context.nodes.get(&node_id) {
            if node.node_type() != NodeType::FunctionDefinition
                && node.node_type() != NodeType::ModifierDefinition
            {
                return Ok(());
            }

            match current_investigation_direction {
                CurrentDFSVector::Inward => {
                    if let ASTNode::FunctionDefinition(function) = node {
                        visitor
                            .visit_inward_function_definition(function)
                            .map_err(|_| super::Error::InwardFunctionDefinitionVisitError)?;
                    }
                    if let ASTNode::ModifierDefinition(modifier) = node {
                        visitor
                            .visit_inward_modifier_definition(modifier)
                            .map_err(|_| super::Error::InwardModifierDefinitionVisitError)?;
                    }
                }
                CurrentDFSVector::Outward => {
                    if let ASTNode::FunctionDefinition(function) = node {
                        visitor
                            .visit_outward_function_definition(function)
                            .map_err(|_| super::Error::OutwardFunctionDefinitionVisitError)?;
                    }
                    if let ASTNode::ModifierDefinition(modifier) = node {
                        visitor
                            .visit_outward_modifier_definition(modifier)
                            .map_err(|_| super::Error::OutwardModifierDefinitionVisitError)?;
                    }
                }
                CurrentDFSVector::OutwardSideEffect => {
                    if let ASTNode::FunctionDefinition(function) = node {
                        visitor.visit_outward_side_effect_function_definition(function).map_err(
                            |_| super::Error::OutwardSideEffectFunctionDefinitionVisitError,
                        )?;
                    }
                    if let ASTNode::ModifierDefinition(modifier) = node {
                        visitor.visit_outward_side_effect_modifier_definition(modifier).map_err(
                            |_| super::Error::OutwardSideEffectModifierDefinitionVisitError,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    pub(super) fn make_entry_point_visit_call<T>(
        &self,
        context: &WorkspaceContext,
        node_id: NodeID,
        visitor: &mut T,
    ) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        let node = context.nodes.get(&node_id).ok_or(super::Error::InvalidEntryPointId(node_id))?;
        visitor.visit_entry_point(node).map_err(|_| super::Error::EntryPointVisitError)?;
        Ok(())
    }
}
