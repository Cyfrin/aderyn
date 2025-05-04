use crate::ast::*;

impl Expression {
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
