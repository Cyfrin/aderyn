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
        browser::ExtractReferencedDeclarations,
        callgraph::WorkspaceCallGraph,
        workspace_context::{ASTNode, WorkspaceContext},
    },
};

use super::StandardInvestigatorVisitor;

pub struct StandardInvestigator {
    /// Ad-hoc Nodes that we would like to explore from.
    pub entry_points: Vec<NodeID>,

    /// Surface points are calculated based on the entry points (input)
    /// and only consists of [`crate::ast::FunctionDefinition`] and [`crate::ast::ModifierDefinition`]
    /// These are nodes that are the *actual* starting points for traversal in the graph
    pub surface_points: Vec<NodeID>,
}

impl StandardInvestigator {
    /// Creates a [`StandardInvestigator`] by exploring paths from given nodes. This is the starting point.
    pub fn for_specific_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
    ) -> super::Result<StandardInvestigator> {
        let mut entry_points = vec![];
        let mut surface_points = vec![];

        // Construct entry points
        for &node in nodes {
            let node_id = node
                .id()
                .ok_or_else(|| super::Error::UnidentifiedEntryPointNode(node.clone()))?;
            entry_points.push(node_id);
        }

        // Construct surface points
        for &node in nodes {
            let referenced_declarations = ExtractReferencedDeclarations::from(node).extracted;

            for declared_id in referenced_declarations {
                if let Some(node) = context.nodes.get(&declared_id) {
                    if node.node_type() == NodeType::ModifierDefinition {
                        surface_points.push(declared_id);
                    } else if let ASTNode::FunctionDefinition(function_definition) = node {
                        if function_definition.implemented {
                            surface_points.push(declared_id);
                        }
                    }
                }
            }
        }

        Ok(StandardInvestigator {
            entry_points,
            surface_points,
        })
    }

    pub fn new(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
    ) -> super::Result<StandardInvestigator> {
        Self::for_specific_nodes(context, nodes)
    }

    /// Visit the entry points and all the plausible function definitions and modifier definitions that
    /// EVM may encounter during execution.
    pub fn investigate<T>(&self, context: &WorkspaceContext, visitor: &mut T) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        self._investigate(
            context,
            context
                .callgraph
                .as_ref()
                .ok_or(super::Error::CallgraphNotAvailable)?,
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
        callgraph: &WorkspaceCallGraph,
        visitor: &mut T,
    ) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        // Visit entry point nodes (so that trackers can track the state across all code regions in 1 place)
        for entry_point_id in &self.entry_points {
            self.make_entry_point_visit_call(context, *entry_point_id, visitor)?;
        }

        // Keep track of visited node IDs during DFS from surface nodes
        let mut visited = HashSet::new();

        // Visit the subgraph starting from surface points
        for surface_point_id in &self.surface_points {
            self.dfs_and_visit_subgraph(
                *surface_point_id,
                &mut visited,
                context,
                callgraph,
                visitor,
                None,
            )?;
        }

        // Collect already visited nodes so that we don't repeat visit calls on them
        // while traversing through side effect nodes.
        let mut blacklisted: HashSet<i64> = HashSet::new();
        blacklisted.extend(visited.iter());
        blacklisted.extend(self.entry_points.iter());

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
        blacklist: Option<&HashSet<NodeID>>,
    ) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        if visited.contains(&node_id) {
            return Ok(());
        }

        visited.insert(node_id);

        if let Some(blacklist) = blacklist {
            if !blacklist.contains(&node_id) {
                self.make_relevant_visit_call(context, node_id, visitor)?;
            }
        } else {
            self.make_relevant_visit_call(context, node_id, visitor)?;
        }

        if let Some(pointing_to) = callgraph.graph.get(&node_id) {
            for destination in pointing_to {
                self.dfs_and_visit_subgraph(
                    *destination,
                    visited,
                    context,
                    callgraph,
                    visitor,
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
    ) -> super::Result<()>
    where
        T: StandardInvestigatorVisitor,
    {
        if let Some(node) = context.nodes.get(&node_id) {
            if node.node_type() != NodeType::FunctionDefinition
                && node.node_type() != NodeType::ModifierDefinition
            {
                return Ok(());
            }

            if let ASTNode::FunctionDefinition(function) = node {
                visitor
                    .visit_function_definition(function)
                    .map_err(|_| super::Error::FunctionDefinitionVisitError)?;
            }
            if let ASTNode::ModifierDefinition(modifier) = node {
                visitor
                    .visit_modifier_definition(modifier)
                    .map_err(|_| super::Error::ModifierDefinitionVisitError)?;
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
