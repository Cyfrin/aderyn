use std::cmp::Ordering;

use crate::context::workspace_context::{ASTNode, WorkspaceContext};

pub trait AppearsAfterASTNodeLocation {
    fn appears_after(&self, context: &WorkspaceContext, other: &ASTNode) -> Option<bool>;
}

impl AppearsAfterASTNodeLocation for ASTNode {
    fn appears_after(&self, context: &WorkspaceContext, other: &ASTNode) -> Option<bool> {
        match context.get_relative_location_of_nodes(self.id()?, other.id()?)? {
            Ordering::Less => Some(false),
            Ordering::Greater => Some(true),
            Ordering::Equal => Some(false),
        }
    }
}

pub trait AppearsBeforeASTNodeLocation {
    fn appears_before(&self, context: &WorkspaceContext, other: &ASTNode) -> Option<bool>;
}

impl AppearsBeforeASTNodeLocation for ASTNode {
    fn appears_before(&self, context: &WorkspaceContext, other: &ASTNode) -> Option<bool> {
        match context.get_relative_location_of_nodes(self.id()?, other.id()?)? {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => Some(false),
        }
    }
}
