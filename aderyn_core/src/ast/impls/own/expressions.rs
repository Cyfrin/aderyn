use crate::ast::*;
use eyre::Result;

impl Expression {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            Expression::Literal(literal) => Some(literal.id),
            Expression::Identifier(identifier) => Some(identifier.id),
            Expression::UnaryOperation(unary_operation) => Some(unary_operation.id),
            Expression::BinaryOperation(binary_operation) => Some(binary_operation.id),
            Expression::Conditional(conditional) => Some(conditional.id),
            Expression::Assignment(assignment) => Some(assignment.id),
            Expression::FunctionCall(function_call) => Some(function_call.id),
            Expression::FunctionCallOptions(function_call_options) => {
                Some(function_call_options.id)
            }
            Expression::IndexAccess(index_access) => Some(index_access.id),
            Expression::IndexRangeAccess(index_range_access) => Some(index_range_access.id),
            Expression::MemberAccess(member_access) => Some(member_access.id),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                Some(elementary_type_name_expression.id)
            }

            Expression::TupleExpression(tuple_expression) => Some(tuple_expression.id),
            Expression::NewExpression(new_expression) => Some(new_expression.id),
        }
    }

    pub fn root_expression(&self) -> Option<&Expression> {
        match self {
            Expression::Identifier(_) => Some(self),
            Expression::TupleExpression(_) => Some(self),
            Expression::Assignment(assignment) => assignment.left_hand_side.root_expression(),
            Expression::IndexAccess(index_access) => index_access.base_expression.root_expression(),
            Expression::IndexRangeAccess(index_range_access) => {
                index_range_access.base_expression.root_expression()
            }
            Expression::MemberAccess(_) => Some(self),
            _ => None,
        }
    }

    pub fn referenced_declarations(&self) -> Vec<NodeID> {
        let mut result = vec![];

        match self {
            Expression::Identifier(identifier) => {
                if let Some(reference_id) = identifier.referenced_declaration {
                    result.push(reference_id);
                }
            }

            Expression::Assignment(assignment) => {
                result.extend(assignment.left_hand_side.referenced_declarations());
                result.extend(assignment.right_hand_side.referenced_declarations());
            }

            Expression::IndexAccess(index_access) => {
                result.extend(index_access.base_expression.referenced_declarations());
            }

            Expression::IndexRangeAccess(index_range_access) => {
                result.extend(index_range_access.base_expression.referenced_declarations());
            }

            Expression::MemberAccess(member_access) => {
                result.extend(member_access.expression.referenced_declarations());

                if let Some(referenced_declaration) = member_access.referenced_declaration {
                    result.push(referenced_declaration);
                }
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter().flatten() {
                    result.extend(component.referenced_declarations());
                }
            }

            Expression::FunctionCall(function_call) => {
                result.extend(function_call.expression.referenced_declarations());

                for argument in function_call.arguments.iter() {
                    result.extend(argument.referenced_declarations());
                }
            }
            Expression::Literal(_) => {} // Literal by definition, does not "reference" any declaration!
            Expression::UnaryOperation(unary_op) => {
                result.extend(unary_op.sub_expression.referenced_declarations());
            }
            Expression::BinaryOperation(binary_op) => {
                result.extend(binary_op.left_expression.referenced_declarations());
                result.extend(binary_op.right_expression.referenced_declarations());
            }
            Expression::Conditional(conditional) => {
                result.extend(conditional.true_expression.referenced_declarations());
                result.extend(conditional.false_expression.referenced_declarations());
                result.extend(conditional.condition.referenced_declarations());
            }
            Expression::FunctionCallOptions(function_call_ops) => {
                result.extend(
                    function_call_ops
                        .options
                        .iter()
                        .flat_map(|opt| opt.referenced_declarations()),
                );

                if let Some(arguments) = function_call_ops.arguments.as_ref() {
                    result.extend(
                        arguments
                            .iter()
                            .flat_map(|opt| opt.referenced_declarations()),
                    );
                }

                result.extend(&function_call_ops.expression.referenced_declarations());
            }
            Expression::ElementaryTypeNameExpression(_) => (), // TODO: Ignore `TypeName` references for now
            Expression::NewExpression(_) => (), // TODO: Ignore `TypeName` references for now
        }

        result
    }

    pub fn contains_operation(&self, operator: &str) -> bool {
        match self {
            Expression::UnaryOperation(unary_operation) => {
                unary_operation.contains_operation(operator)
            }
            Expression::BinaryOperation(binary_operation) => {
                binary_operation.contains_operation(operator)
            }
            Expression::Conditional(conditional) => conditional.contains_operation(operator),
            Expression::Assignment(assignment) => assignment.contains_operation(operator),
            Expression::FunctionCall(function_call) => function_call.contains_operation(operator),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.contains_operation(operator)
            }
            Expression::IndexAccess(index_access) => index_access.contains_operation(operator),
            Expression::IndexRangeAccess(index_range_access) => {
                index_range_access.contains_operation(operator)
            }
            Expression::MemberAccess(member_access) => member_access.contains_operation(operator),
            Expression::TupleExpression(tuple_expression) => {
                tuple_expression.contains_operation(operator)
            }
            _ => false,
        }
    }

    pub fn type_descriptions(&self) -> Option<&TypeDescriptions> {
        match self {
            Expression::Literal(Literal {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Identifier(Identifier {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::UnaryOperation(UnaryOperation {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::BinaryOperation(BinaryOperation {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Conditional(Conditional {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Assignment(Assignment {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::FunctionCall(FunctionCall {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::FunctionCallOptions(FunctionCallOptions {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::IndexAccess(IndexAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::IndexRangeAccess(IndexRangeAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::MemberAccess(MemberAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_descriptions,
                ..
            }) => Some(type_descriptions),
            Expression::TupleExpression(TupleExpression {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::NewExpression(NewExpression {
                type_descriptions, ..
            }) => Some(type_descriptions),
        }
    }

    pub fn source_line(&self, source_unit: &SourceUnit) -> Result<usize> {
        source_unit.source_line(match self {
            Expression::Literal(Literal { src, .. }) => src.as_str(),
            Expression::Identifier(Identifier { src, .. }) => src.as_str(),
            Expression::UnaryOperation(UnaryOperation { src, .. }) => src.as_str(),
            Expression::BinaryOperation(BinaryOperation { src, .. }) => src.as_str(),
            Expression::Conditional(Conditional { src, .. }) => src.as_str(),
            Expression::Assignment(Assignment { src, .. }) => src.as_str(),
            Expression::FunctionCall(FunctionCall { src, .. }) => src.as_str(),
            Expression::FunctionCallOptions(FunctionCallOptions { src, .. }) => src.as_str(),
            Expression::IndexAccess(IndexAccess { src, .. }) => src.as_str(),
            Expression::IndexRangeAccess(IndexRangeAccess { src, .. }) => src.as_str(),
            Expression::MemberAccess(MemberAccess { src, .. }) => src.as_str(),
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                src, ..
            }) => src.as_str(),
            Expression::TupleExpression(TupleExpression { src, .. }) => src.as_str(),
            Expression::NewExpression(NewExpression { src, .. }) => src.as_str(),
        })
    }
}

impl UnaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator || self.sub_expression.contains_operation(operator)
    }
}

impl BinaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator
            || self.left_expression.contains_operation(operator)
            || self.right_expression.contains_operation(operator)
    }
}

impl Conditional {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.condition.contains_operation(operator)
            || self.true_expression.contains_operation(operator)
            || self.false_expression.contains_operation(operator)
    }
}

impl Assignment {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator || self.right_hand_side.contains_operation(operator)
    }
}

impl FunctionCall {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for argument in self.arguments.iter() {
            if argument.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl FunctionCallOptions {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for option in self.options.iter() {
            if option.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl IndexAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        if let Some(index_expression) = &self.index_expression {
            index_expression.contains_operation(operator);
        }
        false
    }
}

impl IndexRangeAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.start_expression
            .as_ref()
            .map(|expr| expr.contains_operation(operator))
            .unwrap_or(false)
            || self
                .end_expression
                .as_ref()
                .map(|expr| expr.contains_operation(operator))
                .unwrap_or(false)
    }
}

impl MemberAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.expression.contains_operation(operator)
    }
}

impl TupleExpression {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for component in self.components.iter() {
            if component
                .as_ref()
                .map(|expr| expr.contains_operation(operator))
                .unwrap_or(false)
            {
                return true;
            }
        }

        false
    }
}
