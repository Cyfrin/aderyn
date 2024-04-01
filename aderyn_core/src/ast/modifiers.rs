use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub body: Block,
    pub overrides: Option<OverrideSpecifier>,
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub r#virtual: Option<bool>,
    pub visibility: Visibility,
    pub src: String,
    pub id: NodeID,
}

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

impl Display for ModifierDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("modifier")?;

        if !self.name.is_empty() {
            f.write_fmt(format_args!(" {}", self.name))?;
        }

        f.write_fmt(format_args!("{}", self.parameters))?;

        if self.visibility != Visibility::Internal {
            f.write_fmt(format_args!("{} {}", self.parameters, self.visibility))?;
        }

        if let Some(true) = self.r#virtual {
            f.write_fmt(format_args!(" virtual"))?;
        }

        if let Some(overrides) = self.overrides.as_ref() {
            f.write_fmt(format_args!(" {overrides}"))?;
        }

        f.write_fmt(format_args!(" {}", self.body))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    ModifierInvocation,
    BaseConstructorSpecifier,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub arguments: Option<Vec<Expression>>,
    pub modifier_name: IdentifierPath,
    pub src: String,
    pub id: NodeID,
    pub kind: Option<ModifierInvocationKind>,
}

impl Node for ModifierInvocation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_modifier_invocation(self)? {
            self.modifier_name.accept(visitor)?;
            if self.arguments.is_some() {
                list_accept(self.arguments.as_ref().unwrap(), visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_modifier_invocation(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_immediate_children(self.id, vec![self.modifier_name.id])?;
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

impl Display for ModifierInvocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.modifier_name))?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_str("(")?;

            for (i, argument) in arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                f.write_fmt(format_args!("{argument}"))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}
