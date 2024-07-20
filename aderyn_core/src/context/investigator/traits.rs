//! Trackers can implement the following traits to interact with investigators

use crate::{
    ast::{FunctionDefinition, ModifierDefinition},
    context::workspace_context::ASTNode,
};

/// Use with [`super::StandardInvestigator`]
pub trait StandardInvestigatorVisitor {
    /// Shift all logic to tracker otherwise, you would track state at 2 different places
    /// One at the tracker level, and other at the application level. Instead, we must
    /// contain all of the tracking logic in the tracker. Therefore, visit entry point
    /// is essential because the tracker can get to take a look at not just the
    /// downstream functions and modifiers, but also the entry points that have invoked it.
    fn visit_entry_point(&mut self, node: &ASTNode) -> eyre::Result<()>;

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::forward_callgraph`]
    fn visit_downstream_function_definition(
        &mut self,
        _node: &FunctionDefinition,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::reverse_callgraph`]
    fn visit_upstream_function_definition(
        &mut self,
        _node: &FunctionDefinition,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::forward_callgraph`]
    fn visit_downstream_modifier_definition(
        &mut self,
        _node: &ModifierDefinition,
    ) -> eyre::Result<()> {
        Ok(())
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::reverse_callgraph`]
    fn visit_upstream_modifier_definition(
        &mut self,
        _node: &ModifierDefinition,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
