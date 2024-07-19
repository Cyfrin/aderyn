//! This module helps with strategies on performing different types of investigations.
//! Our first kind of investigator is [`SimpleInvestigator`] it comes bundled with actions to help
//! application modules like `AuditorDetector`s to "hook in" and take advantage to guage the
//! solidity codebase for anything of interest.
//!
//! As we get more sophisticated, we could introduce more types of investigators!
//!
use std::collections::{hash_map, HashMap, HashSet};

use crate::{
    ast::{Expression, FunctionDefinition, IdentifierOrIdentifierPath, NodeID},
    context::{
        browser::{ExtractFunctionCalls, ExtractModifierInvocations},
        workspace_context::{ASTNode, WorkspaceContext},
    },
    visitor::ast_visitor::ASTConstVisitor,
};

use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    SimpleInvestigatorDFSFailure,
    SimpleInvestigatorVisitorFailure,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

impl std::error::Error for Error {}

/// It creates simple call graph from a given [`FunctionDefinition`] and provides a hook to the
/// visitor for investigating in-the-path nodes that may be of interest.
///
/// SimpleInvestigator does a Depth First Search to recursively visit the following types of Nodes
/// - [`ModifierDefinition`]
/// - [`FunctionDefinition`]
///
/// NOTE: This is NOT a Control-Flow-Graph so that means it does not promise *any* kind of order
pub struct SimpleInvestigator {
    /// Dependents can either browse this graph directly or use the [`SimpleInvestigator::investigate`] method
    /// for easier user (but that comes at the cost of reduced transparency)
    pub graph: SIGraph,

    pub start_node_id: NodeID,
}

pub type SIGraph = HashMap<NodeID, Vec<NodeID>>;

impl SimpleInvestigator {
    fn dfs_to_create_graph(
        id: NodeID,
        graph: &mut SIGraph,
        visited: &mut HashSet<NodeID>,
        context: &WorkspaceContext,
    ) -> Result<()> {
        if visited.contains(&id) {
            return Ok(());
        }

        visited.insert(id);

        // Only deal with `id`s that are in scope right now
        if let Some(from_node) = context.nodes.get(&id) {
            // Make connections to [`FunctionDefinition`] that maybe invoked from the current node
            let function_calls = ExtractFunctionCalls::from(from_node).extracted;
            for function_call in function_calls {
                if let Expression::Identifier(identifier) = function_call.expression.as_ref() {
                    if let Some(referenced_function_id) = identifier.referenced_declaration {
                        Self::create_connection_if_not_exsits(id, referenced_function_id, graph);
                        Self::dfs_to_create_graph(referenced_function_id, graph, visited, context)?;
                    }
                }
            }

            // Make connections to [`ModifierDefinition`] that maybe invoked from the current node
            let modifier_invocations = ExtractModifierInvocations::from(from_node).extracted;
            for modifier_invocation in &modifier_invocations {
                match &modifier_invocation.modifier_name {
                    IdentifierOrIdentifierPath::Identifier(identifier) => {
                        if let Some(reference_modifier_id) = identifier.referenced_declaration {
                            Self::create_connection_if_not_exsits(id, reference_modifier_id, graph);
                            Self::dfs_to_create_graph(
                                reference_modifier_id,
                                graph,
                                visited,
                                context,
                            )?;
                        }
                    }
                    IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                        let referenced_modifier_id = identifier_path.referenced_declaration;
                        Self::create_connection_if_not_exsits(
                            id,
                            referenced_modifier_id as i64,
                            graph,
                        );
                        Self::dfs_to_create_graph(
                            referenced_modifier_id as i64,
                            graph,
                            visited,
                            context,
                        )?;
                    }
                }
            }
        }

        // Change the default return to error later in "strict mode" maybe, because if we
        // can't find the node that means, the file was not in scope and hence it is not
        // available in the context although references to it, exist.
        Ok(())
    }

    fn create_connection_if_not_exsits(from_id: NodeID, to_id: NodeID, graph: &mut SIGraph) {
        match graph.entry(from_id) {
            hash_map::Entry::Occupied(mut o) => {
                // Performance Tip: Maybe later use binary search (it requires keeping ascending order while inserting tho)
                if !o.get().contains(&to_id) {
                    o.get_mut().push(to_id);
                }
            }
            hash_map::Entry::Vacant(v) => {
                v.insert(vec![to_id]);
            }
        }
    }

    /// Creates a [`SimpleInvestigator`] by exploring paths from given node. This is the starting point
    /// for traversal. In this graph, we only go forward i.e, we don't track the upstream functions/modifiers.
    pub fn for_node(
        node: &FunctionDefinition,
        context: &WorkspaceContext,
    ) -> Result<SimpleInvestigator> {
        let mut graph = HashMap::new();
        let mut visited = HashSet::new();

        Self::dfs_to_create_graph(node.id, &mut graph, &mut visited, context)
            .map_err(|_| Error::SimpleInvestigatorDFSFailure)?;

        let investigator = SimpleInvestigator {
            graph,
            start_node_id: node.id,
        };
        Ok(investigator)
    }

    /// Visit all the possible function definitions and modifier definitions that we might
    /// encounter. Because, we created this graph in the beginning by starting from
    /// [`SimpleInvestigator::for_node`], it would mean our graph contains all the nodes reachable
    /// from that starting node only.
    pub fn investigate<T>(&self, context: &WorkspaceContext, visitor: &mut T) -> Result<()>
    where
        T: ASTConstVisitor,
    {
        let mut investigated = HashSet::new();
        for node_ids in self.graph.values() {
            for node_id in node_ids {
                if !investigated.contains(node_id) {
                    Self::make_relevant_visit_call(*node_id, context, visitor)?;
                    investigated.insert(node_id);
                }
            }
        }
        for node_id in self.graph.keys() {
            if !investigated.contains(node_id) {
                Self::make_relevant_visit_call(*node_id, context, visitor)?;
                investigated.insert(node_id);
            }
        }

        if !investigated.contains(&self.start_node_id) {
            Self::make_relevant_visit_call(self.start_node_id, context, visitor)?;
            investigated.insert(&self.start_node_id);
        }

        Ok(())
    }

    fn make_relevant_visit_call<T>(
        node_id: NodeID,
        context: &WorkspaceContext,
        visitor: &mut T,
    ) -> Result<()>
    where
        T: ASTConstVisitor,
    {
        if let Some(node) = context.nodes.get(&node_id) {
            match node {
                ASTNode::FunctionDefinition(function_definition) => visitor
                    .visit_function_definition(function_definition)
                    .map_err(|_| Error::SimpleInvestigatorVisitorFailure)?,
                ASTNode::ModifierDefinition(modifier_definition) => visitor
                    .visit_modifier_definition(modifier_definition)
                    .map_err(|_| Error::SimpleInvestigatorVisitorFailure)?,
                _ => true,
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod investigatory_tests {}
