//! Trackers can implement the following traits to interact with investigators
//!
//! NOTE
//! Upstream and downstream here is relative to [`super::StandardInvestigator::entry_points`]
//! which is initialized with [`super::StandardInvestigator::new`] function.

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
    fn visit_entry_point(&mut self, node: &ASTNode) -> eyre::Result<()> {
        self.visit_any(node)
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::forward_callgraph`]
    fn visit_downstream_function_definition(
        &mut self,
        node: &FunctionDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::reverse_callgraph`]
    fn visit_upstream_function_definition(
        &mut self,
        node: &FunctionDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::forward_callgraph`]
    fn visit_downstream_modifier_definition(
        &mut self,
        node: &ModifierDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Meant to be invoked while traversing [`crate::context::workspace_context::WorkspaceContext::reverse_callgraph`]
    fn visit_upstream_modifier_definition(
        &mut self,
        node: &ModifierDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Read as "upstream's downstream-side-effect" function definition
    /// These are function definitions that are downstream from the upstream nodes
    /// but are themselves neither upstream nor downstream to the entry points
    fn visit_upstream_side_effect_function_definition(
        &mut self,
        node: &FunctionDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Read as "upstream's downstream-side-effect" modifier definition
    /// These are modifier definitions that are downstream from the upstream nodes
    /// but are themselves neither upstream nor downstream to the entry points
    fn visit_upstream_side_effect_modifier_definition(
        &mut self,
        node: &ModifierDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    fn visit_any(&mut self, _node: &ASTNode) -> eyre::Result<()> {
        Ok(())
    }
}
