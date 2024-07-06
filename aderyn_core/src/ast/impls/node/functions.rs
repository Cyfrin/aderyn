use crate::ast::*;
use crate::visitor::ast_visitor::*;
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
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
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
        let overrides_ids = &self
            .overrides
            .iter()
            .filter_map(|x| x.get_node_id())
            .collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, overrides_ids.clone())?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
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
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl FunctionDefinition {
    /// The kind of function this node defines.
    pub fn kind(&self) -> &FunctionKind {
        if let Some(kind) = &self.kind {
            kind
        } else if self.is_constructor {
            &FunctionKind::Constructor
        } else {
            &FunctionKind::Function
        }
    }

    /// The state mutability of the function.
    ///
    /// Note: Before Solidity 0.5.x, this is an approximation, as there was no distinction between
    /// `view` and `pure`.
    pub fn state_mutability(&self) -> &StateMutability {
        if let Some(state_mutability) = &self.state_mutability {
            state_mutability
        } else if self.is_declared_const {
            &StateMutability::View
        } else if self.is_payable {
            &StateMutability::Payable
        } else {
            &StateMutability::NonPayable
        }
    }

    pub fn get_assigned_return_variables(&self, expression: &Expression) -> Vec<NodeID> {
        let mut ids = vec![];

        match expression {
            Expression::Identifier(identifier) => {
                if let Some(reference_id) = identifier.referenced_declaration {
                    if self
                        .return_parameters
                        .parameters
                        .iter()
                        .any(|p| p.id == reference_id)
                    {
                        ids.push(reference_id);
                    }
                }
            }

            Expression::Assignment(assignment) => {
                ids.extend(self.get_assigned_return_variables(assignment.left_hand_side.as_ref()));
            }

            Expression::IndexAccess(index_access) => {
                ids.extend(
                    self.get_assigned_return_variables(index_access.base_expression.as_ref()),
                );
            }

            Expression::IndexRangeAccess(index_range_access) => {
                ids.extend(
                    self.get_assigned_return_variables(index_range_access.base_expression.as_ref()),
                );
            }

            Expression::MemberAccess(member_access) => {
                ids.extend(self.get_assigned_return_variables(member_access.expression.as_ref()));
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter().flatten() {
                    ids.extend(self.get_assigned_return_variables(component));
                }
            }

            _ => (),
        }

        ids
    }
}
