use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub trait GetClosestAncestorOfTypeX {
    /// Get the parent Chain of an ASTNode
    fn closest_ancestor_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode>;
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

impl<T: Node + ?Sized> GetClosestAncestorOfTypeX for T {
    fn closest_ancestor_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;
        context.get_closest_ancestor(current_node_id, node_type)
    }
}
