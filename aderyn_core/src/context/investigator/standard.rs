//! This module helps with strategies on performing different types of investigations.
//!
//! Our first kind of investigator is [`CallGraph`] it comes bundled with actions to help
//! application modules "hook in" and consume the graphs.
//!
//!

use std::collections::HashSet;

use crate::{
    ast::{NodeID, NodeType},
    context::{
        browser::{ExtractReferencedDeclarations, GetClosestAncestorOfTypeX},
        graph::WorkspaceCallGraph,
        workspace_context::{ASTNode, WorkspaceContext},
    },
};

use super::CallGraphVisitor;

#[derive(PartialEq)]
pub enum CallGraphDirection {
    /// Depper into the callgraph
    Inward,

    /// Opposite of Inward
    Outward,

    /// Both inward and outward (If outward side effects also need to be tracked)
    BothWays,
}

pub struct CallGraph {
    /// Ad-hoc Nodes that we would like to explore inward from.
    pub entry_points: Vec<NodeID>,

    /// Surface points are calculated based on the entry points (input)
    /// and only consists of [`crate::ast::FunctionDefinition`] and [`crate::ast::ModifierDefinition`]
    /// These are nodes that are the *actual* starting points for traversal in the graph
    pub inward_surface_points: Vec<NodeID>,

    /// Same as the inward one, but acts on reverse graph.
    pub outward_surface_points: Vec<NodeID>,

    /// Decides what graph type to chose from [`WorkspaceContext`]
    pub direction: CallGraphDirection,
}

#[derive(PartialEq, Clone, Copy)]
enum CurrentDFSVector {
    Inward,
    Outward,
    OutwardSideEffect,
}

impl CallGraph {
    /// Creates a [`CallGraph`] by exploring paths from given nodes. This is the starting point.
    pub fn for_specific_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<CallGraph> {
        let mut entry_points = vec![];
        let mut inward_surface_points = vec![];
        let mut outward_surface_points = vec![];

        // Construct entry points
        for &node in nodes {
            let node_id = node
                .id()
                .ok_or_else(|| super::Error::UnidentifiedEntryPointNode(node.clone()))?;
            entry_points.push(node_id);
        }

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

        // Construct outward surface points
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
                    .or_else(|| {
                        node.closest_ancestor_of_type(context, NodeType::ModifierDefinition)
                    });
                if let Some(parent_surface_point) = parent_surface_point {
                    if let Some(parent_surface_point_id) = parent_surface_point.id() {
                        outward_surface_points.push(parent_surface_point_id);
                    }
                }
            }
        }

        Ok(CallGraph {
            entry_points,
            inward_surface_points,
            outward_surface_points,
            direction,
        })
    }

    pub fn new(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<CallGraph> {
        Self::for_specific_nodes(context, nodes, direction)
    }

    /// Visit the entry points and all the plausible function definitions and modifier definitions that
    /// EVM may encounter during execution.
    pub fn investigate<T>(&self, context: &WorkspaceContext, visitor: &mut T) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        self._investigate(
            context,
            context
                .inward_callgraph
                .as_ref()
                .ok_or(super::Error::InwardCallgraphNotAvailable)?,
            context
                .outward_callgraph
                .as_ref()
                .ok_or(super::Error::OutwardCallgraphNotAvailable)?,
            visitor,
        )
    }

    /// Responsible for informing the trackers.
    /// First, we visit the entry points. Then, we derive the subgraph from the [`WorkspaceCallGraph`]
    /// which consists of all the nodes that can be reached by traversing the edges starting
    /// from the surface points.
    fn _investigate<T>(
        &self,
        context: &WorkspaceContext,
        inward_callgraph: &WorkspaceCallGraph,
        outward_callgraph: &WorkspaceCallGraph,
        visitor: &mut T,
    ) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        // Visit entry point nodes (so that trackers can track the state across all code regions in 1 place)
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
                    inward_callgraph,
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
                    outward_callgraph,
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
                    inward_callgraph,
                    visitor,
                    CurrentDFSVector::OutwardSideEffect,
                    Some(&blacklisted),
                )?;
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs_and_visit_subgraph<T>(
        &self,
        node_id: NodeID,
        visited: &mut HashSet<NodeID>,
        context: &WorkspaceContext,
        callgraph: &WorkspaceCallGraph,
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

        if let Some(pointing_to) = callgraph.graph.get(&node_id) {
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

    fn make_relevant_visit_call<T>(
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
                        visitor
                            .visit_outward_side_effect_function_definition(function)
                            .map_err(|_| {
                                super::Error::OutwardSideEffectFunctionDefinitionVisitError
                            })?;
                    }
                    if let ASTNode::ModifierDefinition(modifier) = node {
                        visitor
                            .visit_outward_side_effect_modifier_definition(modifier)
                            .map_err(|_| {
                                super::Error::OutwardSideEffectModifierDefinitionVisitError
                            })?;
                    }
                }
            }
        }

        Ok(())
    }

    fn make_entry_point_visit_call<T>(
        &self,
        context: &WorkspaceContext,
        node_id: NodeID,
        visitor: &mut T,
    ) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        let node = context
            .nodes
            .get(&node_id)
            .ok_or(super::Error::InvalidEntryPointId(node_id))?;
        visitor
            .visit_entry_point(node)
            .map_err(|_| super::Error::EntryPointVisitError)?;
        Ok(())
    }
}
