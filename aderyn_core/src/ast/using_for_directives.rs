use super::*;
use crate::visitor::ast_visitor::*;
// use cyfrin_foundry_compilers::artifacts::serde_helpers;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub function_list: Option<Vec<UsingForFunctionItem>>,
    #[serde(default)]
    pub global: bool,
    pub library_name: Option<UserDefinedTypeNameOrIdentifierPath>,
    pub type_name: Option<TypeName>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum UserDefinedTypeNameOrIdentifierPath {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum UsingForFunctionItem {
    Function(FunctionIdentifierPath),
    OverloadedOperator(OverloadedOperator),
}

/// A wrapper around [IdentifierPath] for the [UsingForDirective].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct FunctionIdentifierPath {
    pub function: IdentifierPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct OverloadedOperator {
    pub definition: IdentifierPath,
    pub operator: String,
}

impl Node for UsingForDirective {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_using_for_directive(self)? {
            // TODO there is a deviation. Missing FuntionsOrLibrary
            if self.library_name.is_some() {
                match self.library_name.as_ref().unwrap() {
                    UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(_) => {}
                    UserDefinedTypeNameOrIdentifierPath::IdentifierPath(identifier_path) => {
                        identifier_path.accept(visitor)?;
                    }
                };
            }
            if self.type_name.is_some() {
                self.type_name.as_ref().unwrap().accept(visitor)?;
            }
            self.accept_metadata(visitor)?;
        }
        visitor.end_visit_using_for_directive(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if self.library_name.is_some() {
            match self.library_name.as_ref().unwrap() {
                UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(_) => {}
                UserDefinedTypeNameOrIdentifierPath::IdentifierPath(identifier_path) => {
                    visitor.visit_immediate_children(self.id, vec![identifier_path.id])?;
                }
            };
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for UsingForDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "using {:?} for {}",
            self.library_name,
            match self.type_name.as_ref() {
                Some(type_name) => format!("{type_name}"),
                None => "_".to_string(),
            }
        ))
    }
}
