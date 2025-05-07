//! This module helps with strategies on performing different types of investigations.
//!
//! Our first kind of callgraph is [`CallGraph`] it comes bundled with actions to help
//! application modules "hook in" and consume the graphs.

mod tests;
mod utils;

use super::{traits::CallGraphVisitor, Error, Result};
use crate::{
    ast::NodeID,
    context::workspace::{ASTNode, WorkspaceContext},
};

#[derive(PartialEq)]
pub enum CallGraphDirection {
    /// Deeper into the callgraph
    Inward,

    /// Opposite of Inward
    Outward,

    /// Both inward and outward (If outward side effects also need to be tracked)
    BothWays,
}

pub struct CallGraphConsumerV1 {
    /// Ad-hoc Nodes that we would like to explore inward from.
    pub entry_points: Vec<NodeID>,

    /// Surface points are calculated based on the entry points (input)
    /// and only consists of [`crate::ast::FunctionDefinition`] and
    /// [`crate::ast::ModifierDefinition`] These are nodes that are the *actual* starting
    /// points for traversal in the graph
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

impl CallGraphConsumerV1 {
    pub fn new(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<CallGraphConsumerV1> {
        Self::from_nodes(context, nodes, direction)
    }

    /// Visit the entry points and all the plausible function definitions and modifier definitions
    /// that EVM may encounter during execution.
    pub fn accept<T>(&self, context: &WorkspaceContext, visitor: &mut T) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        self._accept(
            context,
            context.inward_callgraph.as_ref().ok_or(super::Error::InwardCallgraphNotAvailable)?,
            context.outward_callgraph.as_ref().ok_or(super::Error::OutwardCallgraphNotAvailable)?,
            visitor,
        )
    }
}
