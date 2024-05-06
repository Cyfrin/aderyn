use crate::{
    ast::NodeID,
    context::{browser::GetPreviousSibling, workspace_context::WorkspaceContext},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub trait PeekOver {
    /// Peek for text outside the node and just above until previous sibling is reached
    fn peek_over(&self, context: &WorkspaceContext) -> Option<String>;
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

impl<T: Node + ?Sized> PeekOver for T {
    fn peek_over(&self, context: &WorkspaceContext) -> Option<String> {
        // Setup a Node ID receiver
        let mut node_id_receiver = NodeIDReceiver::default();

        // Find the ID of the node this method is called upon
        self.accept_id(&mut node_id_receiver).ok()?;
        let current_node_id = node_id_receiver.id?;

        let source_unit =
            context.get_source_unit_from_child_node(context.nodes.get(&current_node_id)?)?;

        let content = source_unit.source.as_ref()?;
        let (curr_offset, _) = context.get_offset_and_length_of_node(current_node_id)?;

        if let Some(previous_sibling) = self.previous_sibling(context) {
            let (prev_offset, prev_len) =
                context.get_offset_and_length_of_node(previous_sibling.id()?)?;
            if prev_offset + prev_len < curr_offset && curr_offset < content.len() {
                let requried_content = &content[prev_offset + prev_len..curr_offset];
                return Some(requried_content.to_string());
            } else {
                return None;
            }
        }

        // If there is no previous sibling we must return content from the top of the file

        if curr_offset < content.len() {
            let requried_content = &content[0..curr_offset];
            return Some(requried_content.to_string());
        }

        None
    }
}
