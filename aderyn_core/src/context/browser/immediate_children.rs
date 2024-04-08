use crate::{
    ast::*,
    context::{
        browser::ExtractImmediateChildrenIDs,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub trait GetImmediateChildren {
    /// Get the immediate children of an ASTNode
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
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

impl<T: Node + ?Sized> GetImmediateChildren for T {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&current_node_id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
