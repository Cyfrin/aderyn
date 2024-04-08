use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

use super::GetImmediateChildren;
use super::SortNodeReferencesToSequence;

pub trait GetNextSibling {
    /// Get the next sibling an ASTNode
    fn next_sibling<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode>;
}

pub trait GetPreviousSibling {
    /// Get the previous sibling an ASTNode
    fn previous_sibling<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode>;
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

impl<T: Node + ?Sized> GetNextSibling for T {
    fn next_sibling<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        let parent = context.get_parent(current_node_id)?;
        let children = parent.children(context)?;
        let sorted_children = children.sort_by_src_position(context)?;

        for i in 0..sorted_children.len() - 1 {
            if sorted_children[i].id()? == current_node_id {
                return Some(sorted_children[i + 1]);
            }
        }

        None
    }
}

impl<T: Node + ?Sized> GetPreviousSibling for T {
    fn previous_sibling<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        let parent = context.get_parent(current_node_id)?;
        let children = parent.children(context)?;
        let sorted_children = children.sort_by_src_position(context)?;

        for i in (1..sorted_children.len()).rev() {
            if sorted_children[i].id()? == current_node_id {
                return Some(sorted_children[i - 1]);
            }
        }

        None
    }
}
