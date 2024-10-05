use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;
use macros::accept_id;

impl Node for TypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            TypeName::FunctionTypeName(function_type_name) => function_type_name.accept(visitor),
            TypeName::ArrayTypeName(array_type_name) => array_type_name.accept(visitor),
            TypeName::Mapping(mapping) => mapping.accept(visitor),
            TypeName::UserDefinedTypeName(user_defined_type_name) => {
                user_defined_type_name.accept(visitor)
            }
            TypeName::ElementaryTypeName(elementary_type_name) => {
                elementary_type_name.accept(visitor)
            }
            TypeName::Raw(_) => Ok(()),
        }
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl Node for ElementaryTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_elementary_type_name(self)?;
        visitor.end_visit_elementary_type_name(self)
    }
    accept_id!();
}

impl Node for UserDefinedTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_user_defined_type_name(self)? && self.path_node.is_some() {
            self.path_node.as_ref().unwrap().accept(visitor)?;
        }
        visitor.end_visit_user_defined_type_name(self)
    }
    accept_id!();
}

impl Node for FunctionTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_function_type_name(self)? {
            self.parameter_types.accept(visitor)?;
            self.return_parameter_types.accept(visitor)?;
        }
        visitor.end_visit_function_type_name(self)
    }
    accept_id!();
}

impl Node for ArrayTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_array_type_name(self)? {
            self.base_type.accept(visitor)?;
            if let Some(length) = self.length.as_ref() {
                length.accept(visitor)?;
            }
        }
        visitor.end_visit_array_type_name(self)
    }
    accept_id!();
}

impl Node for Mapping {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_mapping(self)? {
            self.key_type.accept(visitor)?;
            self.value_type.accept(visitor)?;
        }
        visitor.end_visit_mapping(self)
    }
    accept_id!();
}
