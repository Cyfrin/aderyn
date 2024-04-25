use crate::{
    ast::NodeID,
    context::{browser::GetNextSibling, workspace_context::WorkspaceContext},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub trait PeekUnder {
    /// Peek for text outside the node and just above until previous sibling is reached
    fn peek_under(&self, context: &WorkspaceContext) -> Option<String>;
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

impl<T: Node + ?Sized> PeekUnder for T {
    fn peek_under(&self, context: &WorkspaceContext) -> Option<String> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        let source_unit =
            context.get_source_unit_from_child_node(context.nodes.get(&current_node_id)?)?;

        let content = source_unit.source.as_ref()?;
        let (curr_offset, curr_len) = context.get_offset_and_length_of_node(current_node_id)?;

        if let Some(next_sibling) = self.next_sibling(context) {
            let (next_offset, _) = context.get_offset_and_length_of_node(next_sibling.id()?)?;
            if curr_offset + curr_len < next_offset && next_offset < content.len() {
                let requried_content = &content[curr_offset + curr_len..next_offset];
                return Some(requried_content.to_string());
            } else {
                return None;
            }
        }

        // If there is no next sibling we must content til the bottom of the file
        let requried_content = &content[curr_offset + curr_len..];
        Some(requried_content.to_string())
    }
}
