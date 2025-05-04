use crate::ast::*;

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

    pub fn type_descriptions(&self) -> Option<&TypeDescriptions> {
        match self {
            Expression::Literal(Literal { type_descriptions, .. }) => Some(type_descriptions),
            Expression::Identifier(Identifier { type_descriptions, .. }) => Some(type_descriptions),
            Expression::UnaryOperation(UnaryOperation { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::BinaryOperation(BinaryOperation { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::Conditional(Conditional { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::Assignment(Assignment { type_descriptions, .. }) => Some(type_descriptions),
            Expression::FunctionCall(FunctionCall { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::FunctionCallOptions(FunctionCallOptions { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::IndexAccess(IndexAccess { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::IndexRangeAccess(IndexRangeAccess { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::MemberAccess(MemberAccess { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_descriptions,
                ..
            }) => Some(type_descriptions),
            Expression::TupleExpression(TupleExpression { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::NewExpression(NewExpression { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
        }
    }
}
