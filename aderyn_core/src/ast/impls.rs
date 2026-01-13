mod ctx;
mod disp;
mod node;
mod own;

use crate::ast::*;

impl From<&Expression> for ASTNode {
    fn from(value: &Expression) -> Self {
        match value {
            Expression::Literal(literal) => ASTNode::Literal(literal.clone()),
            Expression::Identifier(identifier) => ASTNode::Identifier(identifier.clone()),
            Expression::UnaryOperation(unary_operation) => {
                ASTNode::UnaryOperation(unary_operation.clone())
            }
            Expression::BinaryOperation(binary_operation) => {
                ASTNode::BinaryOperation(binary_operation.clone())
            }
            Expression::Conditional(conditional) => ASTNode::Conditional(conditional.clone()),
            Expression::Assignment(assignment) => ASTNode::Assignment(assignment.clone()),
            Expression::FunctionCall(function_call) => ASTNode::FunctionCall(function_call.clone()),
            Expression::FunctionCallOptions(function_call_ops) => {
                ASTNode::FunctionCallOptions(function_call_ops.clone())
            }
            Expression::IndexAccess(index_access) => ASTNode::IndexAccess(index_access.clone()),
            Expression::IndexRangeAccess(index_range_access) => {
                ASTNode::IndexRangeAccess(index_range_access.clone())
            }
            Expression::MemberAccess(member_access) => ASTNode::MemberAccess(member_access.clone()),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                ASTNode::ElementaryTypeNameExpression(elementary_type_name_expression.clone())
            }
            Expression::TupleExpression(tuple_expression) => {
                ASTNode::TupleExpression(tuple_expression.clone())
            }
            Expression::NewExpression(new_expression) => {
                ASTNode::NewExpression(new_expression.clone())
            }
        }
    }
}

impl From<Expression> for ASTNode {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Literal(literal) => ASTNode::Literal(literal),
            Expression::Identifier(identifier) => ASTNode::Identifier(identifier),
            Expression::UnaryOperation(unary_operation) => ASTNode::UnaryOperation(unary_operation),
            Expression::BinaryOperation(binary_operation) => {
                ASTNode::BinaryOperation(binary_operation)
            }
            Expression::Conditional(conditional) => ASTNode::Conditional(conditional),
            Expression::Assignment(assignment) => ASTNode::Assignment(assignment),
            Expression::FunctionCall(function_call) => ASTNode::FunctionCall(function_call),
            Expression::FunctionCallOptions(function_call_ops) => {
                ASTNode::FunctionCallOptions(function_call_ops)
            }
            Expression::IndexAccess(index_access) => ASTNode::IndexAccess(index_access),
            Expression::IndexRangeAccess(index_range_access) => {
                ASTNode::IndexRangeAccess(index_range_access)
            }
            Expression::MemberAccess(member_access) => ASTNode::MemberAccess(member_access),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                ASTNode::ElementaryTypeNameExpression(elementary_type_name_expression)
            }
            Expression::TupleExpression(tuple_expression) => {
                ASTNode::TupleExpression(tuple_expression)
            }
            Expression::NewExpression(new_expression) => ASTNode::NewExpression(new_expression),
        }
    }
}

impl From<Statement> for ASTNode {
    fn from(value: Statement) -> Self {
        match value {
            Statement::Block(node) => node.into(),
            Statement::Break(node) => node.into(),
            Statement::Continue(node) => node.into(),
            Statement::DoWhileStatement(node) => node.into(),
            Statement::PlaceholderStatement(node) => node.into(),
            Statement::VariableDeclarationStatement(node) => node.into(),
            Statement::IfStatement(node) => node.into(),
            Statement::ForStatement(node) => node.into(),
            Statement::WhileStatement(node) => node.into(),
            Statement::EmitStatement(node) => node.into(),
            Statement::TryStatement(node) => node.into(),
            Statement::UncheckedBlock(node) => node.into(),
            Statement::Return(node) => node.into(),
            Statement::RevertStatement(node) => node.into(),
            Statement::ExpressionStatement(node) => node.into(),
            Statement::InlineAssembly(node) => node.into(),
        }
    }
}

impl From<&Statement> for ASTNode {
    fn from(value: &Statement) -> Self {
        match value {
            Statement::Block(node) => node.into(),
            Statement::Break(node) => node.into(),
            Statement::Continue(node) => node.into(),
            Statement::DoWhileStatement(node) => node.into(),
            Statement::PlaceholderStatement(node) => node.into(),
            Statement::VariableDeclarationStatement(node) => node.into(),
            Statement::IfStatement(node) => node.into(),
            Statement::ForStatement(node) => node.into(),
            Statement::WhileStatement(node) => node.into(),
            Statement::EmitStatement(node) => node.into(),
            Statement::TryStatement(node) => node.into(),
            Statement::UncheckedBlock(node) => node.into(),
            Statement::Return(node) => node.into(),
            Statement::RevertStatement(node) => node.into(),
            Statement::ExpressionStatement(node) => node.into(),
            Statement::InlineAssembly(node) => node.into(),
        }
    }
}
