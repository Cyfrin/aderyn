use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use std::fmt::Display;

impl Node for ModifierDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_modifier_definition(self)? {
            // TODO: should we implement a string based visitor?
            // self.name.accept(visitor)?;
            self.body.accept(visitor)?;
            if let Some(override_specifier) = &self.overrides {
                override_specifier.accept(visitor)?;
            }
            self.parameters.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_modifier_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_immediate_children(self.id, vec![self.body.id])?;
        if let Some(override_specifier) = &self.overrides {
            visitor.visit_immediate_children(self.id, vec![override_specifier.id])?;
        }
        visitor.visit_immediate_children(self.id, vec![self.parameters.id])?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl IdentifierOrIdentifierPath {
    pub fn get_node_id(&self) -> NodeID {
        match self {
            IdentifierOrIdentifierPath::Identifier(n) => n.id,
            IdentifierOrIdentifierPath::IdentifierPath(n) => n.id,
        }
    }

    pub fn name(&self) -> String {
        match self {
            IdentifierOrIdentifierPath::Identifier(identifier) => identifier.name.clone(),
            IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                identifier_path.name.clone()
            }
        }
    }

    pub fn referenced_declaration(&self) -> Option<NodeID> {
        match self {
            IdentifierOrIdentifierPath::Identifier(identifier) => identifier.referenced_declaration,
            IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                identifier_path.referenced_declaration
            }
        }
    }
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
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
