use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for UsingForDirective {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_using_for_directive(self)? {
            // TODO there is a deviation. Missing FunctionsOrLibrary
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
            if let Some(function_list) = &self.function_list {
                for func_item in function_list {
                    match func_item {
                        UsingForFunctionItem::Function(function) => {
                            function.function.accept(visitor)?;
                        }
                        UsingForFunctionItem::OverloadedOperator(overloaded_operator) => {
                            overloaded_operator.definition.accept(visitor)?;
                        }
                    }
                }
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
    macros::accept_id!();
}
