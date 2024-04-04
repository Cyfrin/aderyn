use crate::{
    ast::NodeID,
    context::workspace_context::WorkspaceContext,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub trait Peek {
    /// Peek into the source code of a node
    fn peek(&self, context: &WorkspaceContext) -> Option<String>;
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

impl<T: Node + ?Sized> Peek for T {
    fn peek(&self, context: &WorkspaceContext) -> Option<String> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        context.get_source_code_of_node(current_node_id)
    }
}
