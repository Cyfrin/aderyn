use crate::ast::{ASTNode, FunctionDefinition, ModifierDefinition};

/// Trait to support reversing of callgraph. (Because, direct impl is not allowed on Foreign Types)
pub trait Transpose {
    fn reverse(&self) -> Self;
}

/// Use with [`super::CallGraph`]
pub trait CallGraphVisitor {
    /// Shift all logic to tracker otherwise, you would track state at 2 different places
    /// One at the tracker level, and other at the application level. Instead, we must
    /// contain all of the tracking logic in the tracker. Therefore, visit entry point
    /// is essential because the tracker can get to take a look at not just the
    /// inward functions and modifiers, but also the entry points that have invoked it.
    fn visit_entry_point(&mut self, node: &ASTNode) -> eyre::Result<()> {
        self.visit_any(node)
    }

    /// Meant to be invoked while traversing
    /// [`crate::context::workspace_context::WorkspaceContext::inward_callgraph`]
    fn visit_inward_function_definition(&mut self, node: &FunctionDefinition) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Meant to be invoked while traversing
    /// [`crate::context::workspace_context::WorkspaceContext::outward_callgraph`]
    fn visit_outward_function_definition(&mut self, node: &FunctionDefinition) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Meant to be invoked while traversing
    /// [`crate::context::workspace_context::WorkspaceContext::inward_callgraph`]
    fn visit_inward_modifier_definition(&mut self, node: &ModifierDefinition) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Meant to be invoked while traversing
    /// [`crate::context::workspace_context::WorkspaceContext::outward_callgraph`]
    fn visit_outward_modifier_definition(&mut self, node: &ModifierDefinition) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Read as "outward's inward-side-effect" function definition
    /// These are function definitions that are inward from the outward nodes
    /// but are themselves neither outward nor inward to the entry points
    fn visit_outward_side_effect_function_definition(
        &mut self,
        node: &FunctionDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    /// Read as "outward's inward-side-effect" modifier definition
    /// These are modifier definitions that are inward from the outward nodes
    /// but are themselves neither outward nor inward to the entry points
    fn visit_outward_side_effect_modifier_definition(
        &mut self,
        node: &ModifierDefinition,
    ) -> eyre::Result<()> {
        self.visit_any(&(node.into()))
    }

    fn visit_any(&mut self, _node: &ASTNode) -> eyre::Result<()> {
        Ok(())
    }
}
