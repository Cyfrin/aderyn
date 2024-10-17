use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for Expression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            Expression::Literal(literal) => literal.accept(visitor),
            Expression::Identifier(identifier) => identifier.accept(visitor),
            Expression::UnaryOperation(unary_operation) => unary_operation.accept(visitor),
            Expression::BinaryOperation(binary_operation) => binary_operation.accept(visitor),
            Expression::Conditional(conditional) => conditional.accept(visitor),
            Expression::Assignment(assignment) => assignment.accept(visitor),
            Expression::FunctionCall(function_call) => function_call.accept(visitor),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.accept(visitor)
            }
            Expression::IndexAccess(index_access) => index_access.accept(visitor),
            Expression::IndexRangeAccess(index_range_access) => index_range_access.accept(visitor),
            Expression::MemberAccess(member_access) => member_access.accept(visitor),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                elementary_type_name_expression.accept(visitor)
            }
            Expression::TupleExpression(tuple_expression) => tuple_expression.accept(visitor),
            Expression::NewExpression(new_expression) => new_expression.accept(visitor),
        }
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl Node for UnaryOperation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_unary_operation(self)? {
            self.sub_expression.accept(visitor)?
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_unary_operation(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(child_id) = self.sub_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![child_id])?;
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for BinaryOperation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_binary_operation(self)? {
            self.left_expression.accept(visitor)?;
            self.right_expression.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_binary_operation(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(left_node_id) = self.left_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![left_node_id])?;
        }
        if let Some(right_node) = self.right_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![right_node])?;
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for Conditional {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_conditional(self)? {
            self.condition.accept(visitor)?;
            self.true_expression.accept(visitor)?;
            self.false_expression.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_conditional(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(condition_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![condition_id])?;
        }
        if let Some(true_expression_id) = self.true_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![true_expression_id])?;
        }
        if let Some(false_expression_id) = self.false_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![false_expression_id])?;
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for Assignment {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_assignment(self)? {
            self.left_hand_side.accept(visitor)?;
            self.right_hand_side.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_assignment(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(left_hand_id) = self.left_hand_side.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![left_hand_id])?;
        }
        if let Some(right_hand_id) = self.right_hand_side.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![right_hand_id])?;
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for FunctionCall {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_function_call(self)? {
            self.expression.accept(visitor)?;
            list_accept(&self.arguments, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_function_call(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr_id) = self.expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![expr_id])?;
        }
        let mut argument_ids = vec![];
        for arg in &self.arguments {
            if let Some(arg_id) = arg.get_node_id() {
                argument_ids.push(arg_id);
            }
        }
        visitor.visit_immediate_children(self.id, argument_ids)?;
        Ok(())
    }
    macros::accept_id!();
}

impl Node for FunctionCallOptions {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_function_call_options(self)? {
            self.expression.accept(visitor)?;
            if self.arguments.is_some() {
                list_accept(self.arguments.as_ref().unwrap(), visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_function_call_options(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr_id) = self.expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![expr_id])?;
        }
        let mut argument_ids = vec![];
        if let Some(arguments) = &self.arguments {
            for arg in arguments {
                if let Some(arg_id) = arg.get_node_id() {
                    argument_ids.push(arg_id);
                }
            }
            visitor.visit_immediate_children(self.id, argument_ids)?;
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for IndexAccess {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_index_access(self)? {
            self.base_expression.accept(visitor)?;
            if let Some(index_expression) = &self.index_expression {
                index_expression.accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_index_access(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(base_expr_id) = self.base_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![base_expr_id])?;
        }
        if let Some(index_expression) = &self.index_expression {
            if let Some(index_expr_id) = index_expression.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![index_expr_id])?;
            }
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for IndexRangeAccess {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_index_range_access(self)? {
            self.base_expression.accept(visitor)?;
            if self.start_expression.is_some() {
                self.start_expression.as_ref().unwrap().accept(visitor)?;
            }
            if self.end_expression.is_some() {
                self.end_expression.as_ref().unwrap().accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_index_range_access(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(start_expression) = &self.start_expression {
            if let Some(start_expr_id) = start_expression.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![start_expr_id])?;
            }
        }
        if let Some(end_expression) = &self.end_expression {
            if let Some(end_expr_id) = end_expression.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![end_expr_id])?;
            }
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for MemberAccess {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_member_access(self)? {
            self.expression.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_member_access(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr_id) = self.expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![expr_id])?;
        }
        Ok(())
    }
    macros::accept_id!();
}

impl Node for ElementaryTypeNameExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_elementary_type_name_expression(self)?;
        visitor.end_visit_elementary_type_name_expression(self)
    }
    macros::accept_id!();
}

impl Node for TupleExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_tuple_expression(self)? {
            for elem in &self.components {
                if elem.is_some() {
                    elem.as_ref().unwrap().accept(visitor)?;
                }
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_tuple_expression(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let mut comp_ids = vec![];
        for expr in self.components.iter().flatten() {
            if let Some(id) = expr.get_node_id() {
                comp_ids.push(id)
            }
        }
        visitor.visit_immediate_children(self.id, comp_ids)?;
        Ok(())
    }
    macros::accept_id!();
}

impl Node for NewExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_new_expression(self)? {
            self.type_name.accept(visitor)?;
        }
        visitor.end_visit_new_expression(self)
    }
    macros::accept_id!();
}
