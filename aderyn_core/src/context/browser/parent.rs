use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub trait GetImmediateParent {
    /// Get the parent Chain of an ASTNode
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode>;
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

impl<T: Node + ?Sized> GetImmediateParent for T {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        context.get_parent(current_node_id)
    }
}
