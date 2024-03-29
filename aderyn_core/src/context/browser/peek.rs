use crate::context::workspace_context::{ASTNode, WorkspaceContext};

pub trait Peek {
    /// Peek into the source code of a node
    fn peek(&self, context: &WorkspaceContext) -> Option<String>;
}

impl Peek for ASTNode {
    fn peek(&self, context: &WorkspaceContext) -> Option<String> {
        context.get_source_code_of_node(self.id()?)
    }
}
