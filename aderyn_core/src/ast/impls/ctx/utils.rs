use crate::{ast::*, context::workspace_context::WorkspaceContext};

impl ContractDefinition {
    /// Returns sequence of all inherited contracts including itself in C3 linearized hirearchy
    pub fn c3<'a>(
        &'a self,
        context: &'a WorkspaceContext,
    ) -> impl Iterator<Item = &'a ContractDefinition> {
        self.linearized_base_contracts.iter().flat_map(|c_id| context.nodes.get(c_id)).flat_map(
            |n| {
                if let ASTNode::ContractDefinition(c) = n {
                    return Some(c);
                }
                return None;
            },
        )
    }
}
