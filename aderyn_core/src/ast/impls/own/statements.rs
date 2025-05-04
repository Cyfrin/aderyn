use crate::ast::*;

impl Statement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                Some(variable_declaration_statement.id)
            }
            Statement::IfStatement(if_statement) => Some(if_statement.id),
            Statement::ForStatement(for_statement) => Some(for_statement.id),
            Statement::WhileStatement(while_statement) => Some(while_statement.id),
            Statement::EmitStatement(emit_statement) => Some(emit_statement.id),
            Statement::UncheckedBlock(unchecked_statement) => Some(unchecked_statement.id),
            Statement::Return(return_statement) => Some(return_statement.id),
            Statement::RevertStatement(revert_statement) => Some(revert_statement.error_call.id),
            Statement::ExpressionStatement(expression_statement) => Some(expression_statement.id),
            Statement::InlineAssembly(inline_assembly) => Some(inline_assembly.id),
            Statement::TryStatement(try_statement) => Some(try_statement.id),
            Statement::Block(block) => Some(block.id),
            Statement::Break(break_statement) => Some(break_statement.id),
            Statement::Continue(continue_statement) => Some(continue_statement.id),
            Statement::DoWhileStatement(do_while_statement) => Some(do_while_statement.id),
            Statement::PlaceholderStatement(placeholder) => Some(placeholder.id),
        }
    }
}

impl BlockOrStatement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            BlockOrStatement::Block(block) => Some(block.id),
            BlockOrStatement::Statement(statement) => statement.get_node_id(),
        }
    }
}
