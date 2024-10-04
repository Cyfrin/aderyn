use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for ParameterList {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_parameter_list(self)? {
            list_accept(&self.parameters, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_parameter_list(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let parameters_ids = &self.parameters.iter().map(|x| x.id).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, parameters_ids.clone())?;
        Ok(())
    }
    macros::accept_id!();
}

impl Node for OverrideSpecifier {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_override_specifier(self)? {
            for overrider in &self.overrides {
                match overrider {
                    UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(type_name) => {
                        type_name.accept(visitor)?
                    }
                    UserDefinedTypeNameOrIdentifierPath::IdentifierPath(identifier_path) => {
                        identifier_path.accept(visitor)?
                    }
                }
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_override_specifier(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let overrides_ids =
            &self.overrides.iter().filter_map(|x| x.get_node_id()).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, overrides_ids.clone())?;
        Ok(())
    }
    macros::accept_id!();
}

impl Node for FunctionDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_function_definition(self)? {
            if self.documentation.is_some() {
                self.documentation.as_ref().unwrap().accept(visitor)?;
            }
            if self.overrides.is_some() {
                self.overrides.as_ref().unwrap().accept(visitor)?;
            }
            self.parameters.accept(visitor)?;
            self.return_parameters.accept(visitor)?;
            list_accept(&self.modifiers, visitor)?;
            if self.body.is_some() {
                self.body.as_ref().unwrap().accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_function_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        // TODO: documentation
        if let Some(overrides) = &self.overrides {
            visitor.visit_immediate_children(self.id, vec![overrides.id])?;
        }
        visitor.visit_immediate_children(self.id, vec![self.parameters.id])?;
        visitor.visit_immediate_children(self.id, vec![self.return_parameters.id])?;
        let modifiers_ids = &self.modifiers.iter().map(|x| x.id).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, modifiers_ids.clone())?;
        if let Some(body) = &self.body {
            visitor.visit_immediate_children(self.id, vec![body.id])?;
        }
        Ok(())
    }
    macros::accept_id!();
}
