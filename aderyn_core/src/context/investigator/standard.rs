//! This module helps with strategies on performing different types of investigations.
//!
//! Our first kind of investigator is [`StandardInvestigator`] it comes bundled with actions to help
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

use super::StandardInvestigatorVisitor;

#[derive(PartialEq)]
pub enum StandardInvestigationStyle {
    /// Picks the regular call graph (forward)
    Downstream,

    /// Picks the reverse call graph
    Upstream,

    /// Picks both the call graphs
    BothWays,
}

pub struct StandardInvestigator {
    /// Ad-hoc Nodes that we would like to explore downstream from.
    pub entry_points: Vec<NodeID>,

    /// Surface points are calculated based on the entry points (input)
    /// and only consists of [`crate::ast::FunctionDefinition`] and [`crate::ast::ModifierDefinition`]
    /// These are nodes that are the *actual* starting points for traversal in the graph
    pub forward_surface_points: Vec<NodeID>,

    /// Same as the forward one, but acts on reverse graph.
    pub backward_surface_points: Vec<NodeID>,

    /// Decides what graph type to chose from [`WorkspaceContext`]
    pub investigation_style: StandardInvestigationStyle,
}

#[derive(PartialEq, Clone, Copy)]
enum CurrentInvestigationDirection {
    Forward,  // Going downstream
    Backward, // Going upstream
}

impl StandardInvestigator {
    /// Creates a [`StandardInvestigator`] by exploring paths from given nodes. This is the starting point.
    pub fn for_specific_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        investigation_style: StandardInvestigationStyle,
    ) -> super::Result<StandardInvestigator> {
        let mut entry_points = vec![];
        let mut forward_surface_points = vec![];
        let mut backward_surface_points = vec![];

        // Construct entry points
        for &node in nodes {
            let node_id = node
                .id()
                .ok_or_else(|| super::Error::UnidentifiedEntryPointNode(node.clone()))?;
            entry_points.push(node_id);
        }

        // Construct forward surface points
        for &node in nodes {
            let referenced_declarations = ExtractReferencedDeclarations::from(node).extracted;

            for declared_id in referenced_declarations {
                if let Some(node) = context.nodes.get(&declared_id) {
                    if node.node_type() == NodeType::ModifierDefinition {
                        forward_surface_points.push(declared_id);
                    } else if let ASTNode::FunctionDefinition(function_definition) = node {
                        if function_definition.implemented {
                            forward_surface_points.push(declared_id);
                        }
                    }
                }
            }
        }

        // Construct backward surface points
        for &node in nodes {
            if node.node_type() == NodeType::FunctionDefinition
                || node.node_type() == NodeType::ModifierDefinition
            {
                if let Some(id) = node.id() {
                    backward_surface_points.push(id);
                }
            } else {
                let parent_surface_point = node
                    .closest_ancestor_of_type(context, NodeType::FunctionDefinition)
                    .or_else(|| {
                        node.closest_ancestor_of_type(context, NodeType::ModifierDefinition)
                    });
                if let Some(parent_surface_point) = parent_surface_point {
                    if let Some(parent_surface_point_id) = parent_surface_point.id() {
                        backward_surface_points.push(parent_surface_point_id);
                    }
                }
            }
        }

        Ok(StandardInvestigator {
            entry_points,
            forward_surface_points,
            backward_surface_points,
            investigation_style,
        })
    }

    pub fn new(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        investigation_style: StandardInvestigationStyle,
    ) -> super::Result<StandardInvestigator> {
        Self::for_specific_nodes(context, nodes, investigation_style)
    }

    /// Visit the entry points and all the plausible function definitions and modifier definitions that
    /// EVM may encounter during execution.
    pub fn investigate<T>(&self, context: &WorkspaceContext, visitor: &T) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        self._investigate(
            context,
            context
                .forward_callgraph
                .as_ref()
                .ok_or(super::Error::ForwardCallgraphNotAvailable)?,
            context
                .reverse_callgraph
                .as_ref()
                .ok_or(super::Error::BackwardCallgraphNotAvailable)?,
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
        forward_callgraph: &WorkspaceCallGraph,
        reverse_callgraph: &WorkspaceCallGraph,
        visitor: &T,
    ) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        // Visit entry point nodes (so that trackers can track the state across all code regions in 1 place)
        for entry_point_id in &self.entry_points {
            self.make_entry_point_visit_call(context, *entry_point_id, visitor)?;
        }

        // Now decide, which points to visit upstream or downstream
        if self.investigation_style == StandardInvestigationStyle::BothWays
            || self.investigation_style == StandardInvestigationStyle::Downstream
        {
            // Keep track of visited node IDs during DFS from surface nodes
            let mut visited = HashSet::new();

            // Visit the subgraph starting from surface points
            for surface_point_id in &self.forward_surface_points {
                self.dfs_and_visit_subgraph(
                    *surface_point_id,
                    &mut visited,
                    context,
                    forward_callgraph,
                    visitor,
                    CurrentInvestigationDirection::Forward,
                )?;
            }
        }

        if self.investigation_style == StandardInvestigationStyle::BothWays
            || self.investigation_style == StandardInvestigationStyle::Upstream
        {
            // Keep track of visited node IDs during DFS from surface nodes
            let mut visited = HashSet::new();

            // Visit the subgraph starting from surface points
            for surface_point_id in &self.backward_surface_points {
                self.dfs_and_visit_subgraph(
                    *surface_point_id,
                    &mut visited,
                    context,
                    reverse_callgraph,
                    visitor,
                    CurrentInvestigationDirection::Backward,
                )?;
            }
        }

        Ok(())
    }

    fn dfs_and_visit_subgraph<T>(
        &self,
        node_id: NodeID,
        visited: &mut HashSet<NodeID>,
        context: &WorkspaceContext,
        callgraph: &WorkspaceCallGraph,
        visitor: &T,
        current_investigation_direction: CurrentInvestigationDirection,
    ) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        if visited.contains(&node_id) {
            return Ok(());
        }

        visited.insert(node_id);
        self.make_relevant_visit_call(context, node_id, visitor, current_investigation_direction)?;

        if let Some(pointing_to) = callgraph.graph.get(&node_id) {
            for destination in pointing_to {
                self.dfs_and_visit_subgraph(
                    *destination,
                    visited,
                    context,
                    callgraph,
                    visitor,
                    current_investigation_direction,
                )?;
            }
        }
        Ok(())
    }

    fn make_relevant_visit_call<T>(
        &self,
        context: &WorkspaceContext,
        node_id: NodeID,
        visitor: &T,
        current_investigation_direction: CurrentInvestigationDirection,
    ) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        if let Some(node) = context.nodes.get(&node_id) {
            assert!(
                node.node_type() == NodeType::FunctionDefinition
                    || node.node_type() == NodeType::ModifierDefinition
            );

            match current_investigation_direction {
                CurrentInvestigationDirection::Forward => {
                    if let ASTNode::FunctionDefinition(function) = node {
                        visitor
                            .visit_downstream_function_definition(function)
                            .map_err(|_| super::Error::DownstreamFunctionDefinitionVisitError)?;
                    }
                    if let ASTNode::ModifierDefinition(modifier) = node {
                        visitor
                            .visit_downstream_modifier_definition(modifier)
                            .map_err(|_| super::Error::DownstreamModifierDefinitionVisitError)?;
                    }
                }
                CurrentInvestigationDirection::Backward => {
                    if let ASTNode::FunctionDefinition(function) = node {
                        visitor
                            .visit_upstream_function_definition(function)
                            .map_err(|_| super::Error::UpstreamFunctionDefinitionVisitError)?;
                    }
                    if let ASTNode::ModifierDefinition(modifier) = node {
                        visitor
                            .visit_upstream_modifier_definition(modifier)
                            .map_err(|_| super::Error::UpstreamModifierDefinitionVisitError)?;
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
        visitor: &T,
    ) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
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
