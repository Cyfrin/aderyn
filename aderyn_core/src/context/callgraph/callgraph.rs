use std::collections::HashSet;

use crate::{
    ast::{ASTNode, FunctionDefinition, ModifierDefinition, NodeID, NodeType},
    context::{browser::ExtractReferencedDeclarations, workspace_context::WorkspaceContext},
};

use super::WorkspaceCallGraph;

/// Use with [`super::CallGraph`]
pub trait CallGraphVisitor {
    /// Shift all logic to tracker otherwise, you would track state at 2 different places
    /// One at the tracker level, and other at the application level. Instead, we must
    /// contain all of the tracking logic in the tracker. Therefore, visit entry point
    /// is essential because the tracker can get to take a look at not just the
    /// functions and modifiers, but also the entry points that have invoked it.
    fn visit_entry_point(&mut self, node: &ASTNode) -> eyre::Result<()> {
        self.visit_any(node)
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::callgraph`]
    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::callgraph`]
    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    fn visit_any(&mut self, _node: &ASTNode) -> eyre::Result<()> {
        Ok(())
    }
}

pub struct CallGraph {
    /// Ad-hoc Nodes that we would like to explore from.
    pub entry_points: Vec<NodeID>,

    /// Surface points are calculated based on the entry points (input)
    /// and only consists of [`crate::ast::FunctionDefinition`] and [`crate::ast::ModifierDefinition`]
    /// These are nodes that are the *actual* starting points for traversal in the graph
    pub surface_points: Vec<NodeID>,
}

impl CallGraph {
    /// Creates a [`CallGraph`] by exploring paths from given nodes. This is the starting point.
    pub fn for_specific_nodes(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
    ) -> super::Result<CallGraph> {
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

        Ok(CallGraph {
            entry_points,
            surface_points,
        })
    }

    pub fn new(context: &WorkspaceContext, nodes: &[&ASTNode]) -> super::Result<CallGraph> {
        Self::for_specific_nodes(context, nodes)
    }

    /// Visit the entry points and all the plausible function definitions and modifier definitions that
    /// EVM may encounter during execution.
    pub fn accept<T>(&self, context: &WorkspaceContext, visitor: &mut T) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        self._accept(
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
    fn _accept<T>(
        &self,
        context: &WorkspaceContext,
        callgraph: &WorkspaceCallGraph,
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
        T: CallGraphVisitor,
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

        if let Some(pointing_to) = callgraph.raw_callgraph.get(&node_id) {
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
        T: CallGraphVisitor,
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
