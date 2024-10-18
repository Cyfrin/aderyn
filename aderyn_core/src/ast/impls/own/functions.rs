use crate::ast::*;

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
                    if self.return_parameters.parameters.iter().any(|p| p.id == reference_id) {
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
