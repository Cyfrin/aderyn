use crate::{
    ast::*,
    context::browser::ExtractImmediateChildrenIDs,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetImmediateChildren {
    /// Get the immediate children of an ASTNode
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
}

impl GetImmediateChildren for ASTNode {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id()?)?)
            .extracted
            .into_iter()
            .map(|x| context.nodes.get(&x))
            .flatten()
            .collect::<Vec<_>>();
        Some(children)
    }
}
