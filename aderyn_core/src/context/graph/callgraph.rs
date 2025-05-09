//! This module helps with strategies on performing different types of investigations.
//!
//! Our first kind of callgraph is [`CallGraph`] it comes bundled with actions to help
//! application modules "hook in" and consume the graphs.

mod legacy;
mod new;
mod tests;
mod utils;
mod visit;

use super::{traits::CallGraphVisitor, Error, Result};
use crate::{
    ast::NodeID,
    context::workspace::{ASTNode, WorkspaceContext},
};

#[derive(Clone, PartialEq)]
pub enum CallGraphDirection {
    /// Deeper into the callgraph
    Inward,

    /// Opposite of Inward
    Outward,

    /// Both inward and outward (If outward side effects also need to be tracked)
    BothWays,
}

pub struct CallGraphConsumer {
    /// Ad-hoc Nodes that we would like to explore inward from.
    pub entry_points: Vec<NodeID>,

    /// Surface points are calculated based on the entry points (input)
    /// and only consists of [`crate::ast::FunctionDefinition`] and
    /// [`crate::ast::ModifierDefinition`] These are nodes that are the *actual* starting
    /// points for traversal in the graph
    pub inward_surface_points: Vec<NodeID>,

    /// Same as the inward one, but acts on reverse graph.
    pub outward_surface_points: Vec<NodeID>,

    /// Decides what graph type to chose.
    pub direction: CallGraphDirection,

    /// Decides what graph to chose from [`WorkspaceContext::callgraphs`].
    pub base_contract: Option<NodeID>,
}

#[derive(PartialEq, Clone, Copy)]
enum CurrentDFSVector {
    Inward,
    Outward,
    OutwardSideEffect,
}

impl CallGraphConsumer {
    pub fn get_legacy(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<CallGraphConsumer> {
        Self::from_nodes(context, nodes, direction)
    }

    pub fn get(
        context: &WorkspaceContext,
        nodes: &[&ASTNode],
        direction: CallGraphDirection,
    ) -> super::Result<Vec<CallGraphConsumer>> {
        Self::many_from_nodes(context, nodes, direction)
    }

    /// Visit the entry points and all the plausible function definitions and modifier definitions
    /// that EVM may encounter during execution.
    pub fn accept<T>(&self, context: &WorkspaceContext, visitor: &mut T) -> super::Result<()>
    where
        T: CallGraphVisitor,
    {
        self._accept(context, visitor)
    }

    #[inline]
    pub fn is_legacy(&self) -> bool {
        self.base_contract.is_none()
    }
}
