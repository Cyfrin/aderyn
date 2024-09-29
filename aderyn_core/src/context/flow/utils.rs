use super::{Cfg, CfgNodeId, Statement};

impl Cfg {
    pub fn add_statement_node(&mut self, stmt: &Statement) -> CfgNodeId {
        match stmt {
            Statement::TryStatement(_) => unimplemented!(),
            Statement::UncheckedBlock(n) => self.add_unchecked_block_node(n),
            Statement::DoWhileStatement(n) => self.add_do_while_statement(n),
            Statement::ForStatement(n) => self.add_for_statement(n),
            Statement::EmitStatement(n) => self.add_emit_statement(n),
            Statement::WhileStatement(n) => self.add_while_statement(n),
            Statement::IfStatement(n) => self.add_if_statement(n),
            Statement::PlaceholderStatement(n) => self.add_placeholder_statement(n),
            Statement::RevertStatement(n) => self.add_revert_statement(n),
            Statement::Return(n) => self.add_return_statement(n),
            Statement::InlineAssembly(n) => self.add_inline_assembly_statement(n),
            Statement::Continue(n) => self.add_continue_statement(n),
            Statement::Break(n) => self.add_break_statement(n),
            Statement::Block(n) => self.add_block_node(n),
            Statement::VariableDeclarationStatement(n) => {
                self.add_variable_declaration_statement(n)
            }
            Statement::ExpressionStatement(n) => self.add_expression_statement(n),
        }
    }
}
