use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for ModifierDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_modifier_definition(self)? {
            // TODO: should we implement a string based visitor?
            // self.name.accept(visitor)?;
            if let Some(body) = &self.body {
                body.accept(visitor)?;
            }
            if let Some(override_specifier) = &self.overrides {
                override_specifier.accept(visitor)?;
            }
            self.parameters.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_modifier_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(body) = &self.body {
            visitor.visit_immediate_children(self.id, vec![body.id])?;
        }
        if let Some(override_specifier) = &self.overrides {
            visitor.visit_immediate_children(self.id, vec![override_specifier.id])?;
        }
        visitor.visit_immediate_children(self.id, vec![self.parameters.id])?;
        Ok(())
    }
    macros::accept_id!();
}

impl Node for ModifierInvocation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_modifier_invocation(self)? {
            match &self.modifier_name {
                IdentifierOrIdentifierPath::Identifier(identifier) => identifier.accept(visitor)?,
                IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                    identifier_path.accept(visitor)?
                }
            };
            if self.arguments.is_some() {
                list_accept(self.arguments.as_ref().unwrap(), visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_modifier_invocation(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_immediate_children(self.id, vec![self.modifier_name.get_node_id()])?;
        if let Some(arguments) = &self.arguments {
            let mut argument_ids = vec![];
            for arg in arguments {
                if let Some(arg_id) = arg.get_node_id() {
                    argument_ids.push(arg_id);
                }
            }
            visitor.visit_immediate_children(self.id, argument_ids)?;
        }
        Ok(())
    }
    macros::accept_id!();
}
