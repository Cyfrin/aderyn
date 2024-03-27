use std::cmp::Ordering;

use crate::{
    ast::NodeID,
    context::workspace_context::WorkspaceContext,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub trait AppearsAfterNodeLocation<T: Node + ?Sized> {
    fn appears_after(&self, context: &WorkspaceContext, other: &T) -> Option<bool>;
}

pub trait AppearsBeforeNodeLocation<T: Node + ?Sized> {
    fn appears_before(&self, context: &WorkspaceContext, other: &T) -> Option<bool>;
}

#[derive(Default)]
struct NodeIDReceiver {
    id: Option<NodeID>,
}

impl ASTConstVisitor for NodeIDReceiver {
    fn visit_node_id(&mut self, node_id: Option<NodeID>) -> eyre::Result<()> {
        self.id = node_id;
        Ok(())
    }
}

impl<T: Node + ?Sized, U: Node + ?Sized> AppearsBeforeNodeLocation<U> for T {
    fn appears_before(&self, context: &WorkspaceContext, other: &U) -> Option<bool> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        // FInd the ID of the target node
        other.accept_id(&mut node_id_receiver).ok()?;
        let target_node_id = node_id_receiver.id?;

        match context.get_relative_location_of_nodes(current_node_id, target_node_id)? {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => Some(false),
        }
    }
}

impl<T: Node + ?Sized, U: Node + ?Sized> AppearsAfterNodeLocation<U> for T {
    fn appears_after(&self, context: &WorkspaceContext, other: &U) -> Option<bool> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        // FInd the ID of the target node
        other.accept_id(&mut node_id_receiver).ok()?;
        let target_node_id = node_id_receiver.id?;

        match context.get_relative_location_of_nodes(current_node_id, target_node_id)? {
            Ordering::Less => Some(false),
            Ordering::Greater => Some(true),
            Ordering::Equal => Some(false),
        }
    }
}
