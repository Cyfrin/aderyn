use crate::{ast::Block, context::workspace::WorkspaceContext};

use super::{ASTNode, AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId, CfgReduce};

use crate::ast::*;

// Control flow graph definitions nodes
#[derive(Debug, Clone)]
pub struct CfgBlock {
    pub block: AstNodeId,
}

impl CfgReduce for CfgBlock {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_block_node(self.block);
        let end_id = cfg.add_end_block_node(self.block);
        cfg.start_end_pairs.insert(start_id, end_id);

        let mut last_link = start_id;

        if let Some(ASTNode::Block(block)) = context.nodes.get(&self.block) {
            for statement in &block.statements {
                let node_id = cfg.add_statement_node(statement);
                cfg.add_flow_edge(last_link, node_id);
                last_link = node_id;
            }
        }

        cfg.add_flow_edge(last_link, end_id);

        (start_id, end_id)
    }
}

impl CfgBlock {
    pub fn from(block: &Block) -> Self {
        Self { block: block.id }
    }
}

/// Helper functions
impl Cfg {
    pub fn add_block_node(&mut self, block: &Block) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Block(Box::new(CfgBlock::from(block))))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgIfStatement {
    pub if_statement: AstNodeId,
}

impl CfgReduce for CfgIfStatement {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_if_node(self.if_statement);
        let end_id = cfg.add_end_if_node(self.if_statement);
        cfg.start_end_pairs.insert(start_id, end_id);

        let Some(ASTNode::IfStatement(if_ast_node)) = context.nodes.get(&self.if_statement) else {
            cfg.add_flow_edge(start_id, end_id);
            return (start_id, end_id);
        };

        // Condition node
        let start_cond = cfg.add_start_if_cond_node();
        let end_cond = cfg.add_end_if_cond_node();
        cfg.start_end_pairs.insert(start_id, end_id);

        let condition = cfg.add_if_statement_condition(&if_ast_node.condition);

        cfg.add_flow_edge(start_id, start_cond);
        cfg.add_flow_edge(start_cond, condition);
        cfg.add_flow_edge(condition, end_cond);

        // True branch
        let start_true_branch = cfg.add_start_if_true_branch_node();
        let end_true_branch = cfg.add_end_if_true_branch_node();
        cfg.start_end_pairs.insert(start_true_branch, end_true_branch);

        let true_block = match &if_ast_node.true_body {
            super::BlockOrStatement::Block(block) => cfg.add_block_node(block.as_ref()),
            super::BlockOrStatement::Statement(stmt) => cfg.add_statement_node(stmt.as_ref()),
        };

        cfg.add_flow_edge(end_cond, start_true_branch);
        cfg.add_flow_edge(start_true_branch, true_block);
        cfg.add_flow_edge(true_block, end_true_branch);

        cfg.add_flow_edge(end_true_branch, end_id);

        // False branch
        if let Some(false_body) = if_ast_node.false_body.as_ref() {
            let start_false_branch = cfg.add_start_if_false_branch_node();
            let end_false_branch = cfg.add_end_if_false_branch_node();
            cfg.start_end_pairs.insert(start_false_branch, end_false_branch);

            let false_block = match false_body {
                super::BlockOrStatement::Block(block) => cfg.add_block_node(block.as_ref()),
                super::BlockOrStatement::Statement(stmt) => cfg.add_statement_node(stmt.as_ref()),
            };

            cfg.add_flow_edge(end_cond, start_false_branch);
            cfg.add_flow_edge(start_false_branch, false_block);
            cfg.add_flow_edge(false_block, end_false_branch);

            cfg.add_flow_edge(end_false_branch, end_id);
        } else {
            // It's possible to skip the true branch if the false branch doesn't exist
            let start_false_branch = cfg.add_start_if_false_branch_node();
            let end_false_branch = cfg.add_end_if_false_branch_node();
            cfg.start_end_pairs.insert(start_false_branch, end_false_branch);

            cfg.add_flow_edge(end_cond, start_false_branch);
            cfg.add_flow_edge(start_false_branch, end_false_branch);

            cfg.add_flow_edge(end_false_branch, end_id);
        }

        (start_id, end_id)
    }
}

impl CfgIfStatement {
    pub fn from(if_stmt: &IfStatement) -> Self {
        Self { if_statement: if_stmt.id }
    }
}

impl Cfg {
    pub fn add_if_statement(&mut self, if_stmt: &IfStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::IfStatement(Box::new(CfgIfStatement::from(if_stmt))))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgWhileStatement {
    pub while_statement: AstNodeId,
}

impl CfgReduce for CfgWhileStatement {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_while_node(self.while_statement);
        let end_id = cfg.add_end_while_node(self.while_statement);
        cfg.start_end_pairs.insert(start_id, end_id);

        let Some(ASTNode::WhileStatement(ast_while_stmt)) =
            context.nodes.get(&self.while_statement)
        else {
            cfg.add_flow_edge(start_id, end_id);
            return (start_id, end_id);
        };

        let start_cond = cfg.add_start_while_cond_node();
        let end_cond = cfg.add_end_while_cond_node();
        cfg.start_end_pairs.insert(start_cond, end_cond);
        cfg.start_cond_pairs.insert(start_id, start_cond);
        cfg.cond_start_pairs.insert(end_cond, start_id);

        let condition = cfg.add_while_statement_condition(&ast_while_stmt.condition);

        cfg.add_flow_edge(start_id, start_cond);
        cfg.add_flow_edge(start_cond, condition);
        cfg.add_flow_edge(condition, end_cond);

        // Exit happens from the condition node
        cfg.add_flow_edge(end_cond, end_id);

        // Loop arcs around the condition
        let start_body = cfg.add_start_while_body_node();
        let end_body = cfg.add_end_while_body_node();
        cfg.start_end_pairs.insert(start_body, end_body);

        let body = match &ast_while_stmt.body {
            BlockOrStatement::Block(block) => cfg.add_block_node(block),
            BlockOrStatement::Statement(stmt) => cfg.add_statement_node(stmt),
        };

        cfg.add_flow_edge(end_cond, start_body);
        cfg.add_flow_edge(start_body, body);
        cfg.add_flow_edge(body, end_body);
        cfg.add_flow_edge(end_body, start_cond);

        (start_id, end_id)
    }
}

impl CfgWhileStatement {
    pub fn from(while_stmt: &WhileStatement) -> Self {
        Self { while_statement: while_stmt.id }
    }
}

impl Cfg {
    pub fn add_while_statement(&mut self, while_stmt: &WhileStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::WhileStatement(Box::new(CfgWhileStatement::from(
            while_stmt,
        ))))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgForStatement {
    pub for_statement: AstNodeId,
}

impl CfgReduce for CfgForStatement {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_for_node(self.for_statement);
        let end_id = cfg.add_end_for_node(self.for_statement);
        cfg.start_end_pairs.insert(start_id, end_id);

        let Some(ASTNode::ForStatement(ast_for_stmt)) = context.nodes.get(&self.for_statement)
        else {
            cfg.add_flow_edge(start_id, end_id);
            return (start_id, end_id);
        };

        // First we prepare the loop initialization expression
        let start_loop_init = cfg.add_start_for_init_exp_node();
        let end_loop_init = cfg.add_end_for_init_exp_node();
        cfg.start_end_pairs.insert(start_loop_init, end_loop_init);

        if let Some(ast_loop_init) = ast_for_stmt.initialization_expression.as_ref() {
            let loop_init = match ast_loop_init.as_ref() {
                ExpressionOrVariableDeclarationStatement::ExpressionStatement(exp_stmt) => {
                    cfg.add_expression_statement(exp_stmt)
                }
                ExpressionOrVariableDeclarationStatement::VariableDeclarationStatement(stmt) => {
                    cfg.add_variable_declaration_statement(stmt)
                }
            };
            cfg.add_flow_edge(start_loop_init, loop_init);
            cfg.add_flow_edge(loop_init, end_loop_init);
        } else {
            // If there is not loop initialization expression, leave it blank
            cfg.add_flow_edge(start_loop_init, end_loop_init);
        }

        // Prepare the loop condition expression
        let start_loop_cond = cfg.add_start_for_cond_node();
        let end_loop_cond = cfg.add_end_for_cond_node();
        cfg.start_end_pairs.insert(start_loop_cond, end_loop_cond);
        cfg.start_cond_pairs.insert(start_id, start_loop_cond);

        if let Some(ast_loop_cond) = ast_for_stmt.condition.as_ref() {
            let loop_cond = cfg.add_for_statement_condition(ast_loop_cond);
            cfg.add_flow_edge(start_loop_cond, loop_cond);
            cfg.add_flow_edge(loop_cond, end_loop_cond);
        } else {
            cfg.add_flow_edge(start_loop_cond, end_loop_cond);
        }

        // Prepare the loop body
        let start_loop_body = cfg.add_start_for_body_node();
        let end_loop_body = cfg.add_end_for_body_node();
        cfg.start_end_pairs.insert(start_loop_body, end_loop_body);

        let loop_body = match &ast_for_stmt.body {
            BlockOrStatement::Block(block) => cfg.add_block_node(block),
            BlockOrStatement::Statement(stmt) => cfg.add_statement_node(stmt),
        };

        cfg.add_flow_edge(start_loop_body, loop_body);
        cfg.add_flow_edge(loop_body, end_loop_body);

        // Prepare the loop expression
        let start_loop_exp = cfg.add_start_for_exp_node();
        let end_loop_exp = cfg.add_end_for_exp_node();
        cfg.start_end_pairs.insert(start_loop_exp, end_loop_exp);
        cfg.start_loop_expr.insert(start_id, start_loop_exp);
        cfg.loop_expr_start.insert(end_loop_exp, start_id);

        if let Some(ast_loop_exp) = ast_for_stmt.loop_expression.as_ref() {
            let loop_exp = cfg.add_expression_statement(ast_loop_exp.as_ref());
            cfg.add_flow_edge(start_loop_exp, loop_exp);
            cfg.add_flow_edge(loop_exp, end_loop_exp);
        } else {
            cfg.add_flow_edge(start_loop_exp, end_loop_exp);
        }

        // Connect all the above components
        cfg.add_flow_edge(start_id, start_loop_init);
        cfg.add_flow_edge(end_loop_init, start_loop_cond);
        cfg.add_flow_edge(end_loop_cond, start_loop_body);
        cfg.add_flow_edge(end_loop_body, start_loop_exp);
        cfg.add_flow_edge(end_loop_exp, start_loop_cond);
        cfg.add_flow_edge(end_loop_cond, end_id);

        (start_id, end_id)
    }
}

impl CfgForStatement {
    pub fn from(for_stmt: &ForStatement) -> Self {
        Self { for_statement: for_stmt.id }
    }
}

impl Cfg {
    pub fn add_for_statement(&mut self, for_stmt: &ForStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::ForStatement(Box::new(CfgForStatement::from(for_stmt))))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgDoWhileStatement {
    pub do_while_statement: AstNodeId,
}

impl CfgReduce for CfgDoWhileStatement {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_do_while_node(self.do_while_statement);
        let end_id = cfg.add_end_do_while_node(self.do_while_statement);
        cfg.start_end_pairs.insert(start_id, end_id);

        let Some(ASTNode::DoWhileStatement(ast_do_while_stmt)) =
            context.nodes.get(&self.do_while_statement)
        else {
            cfg.add_flow_edge(start_id, end_id);
            return (start_id, end_id);
        };

        // Loop body
        let start_loop_body = cfg.add_start_do_while_body_node();
        let end_loop_body = cfg.add_end_do_while_body_node();
        cfg.start_end_pairs.insert(start_loop_body, end_loop_body);

        let loop_body = cfg.add_block_node(&ast_do_while_stmt.body);

        cfg.add_flow_edge(start_id, start_loop_body);
        cfg.add_flow_edge(start_loop_body, loop_body);
        cfg.add_flow_edge(loop_body, end_loop_body);

        // Loop condition
        let start_loop_cond = cfg.add_start_do_while_cond_node();
        let end_loop_cond = cfg.add_end_do_while_cond_node();
        cfg.start_end_pairs.insert(start_loop_cond, end_loop_cond);
        cfg.start_cond_pairs.insert(start_id, start_loop_cond);
        cfg.cond_start_pairs.insert(end_loop_cond, start_id);

        let loop_cond = cfg.add_do_while_statement_condition(&ast_do_while_stmt.condition);

        cfg.add_flow_edge(end_loop_body, start_loop_cond);
        cfg.add_flow_edge(start_loop_cond, loop_cond);
        cfg.add_flow_edge(loop_cond, end_loop_cond);

        // Loop link
        cfg.add_flow_edge(end_loop_cond, start_loop_body);

        // Exit link
        cfg.add_flow_edge(end_loop_cond, end_id);

        (start_id, end_id)
    }
}

impl CfgDoWhileStatement {
    pub fn from(do_while_stmt: &DoWhileStatement) -> Self {
        Self { do_while_statement: do_while_stmt.id }
    }
}

impl Cfg {
    pub fn add_do_while_statement(&mut self, do_while_stmt: &DoWhileStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::DoWhileStatement(Box::new(CfgDoWhileStatement::from(
            do_while_stmt,
        ))))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgUncheckedBlock {
    pub unchecked_block: AstNodeId,
}

impl CfgReduce for CfgUncheckedBlock {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_unchecked_block_node(self.unchecked_block);
        let end_id = cfg.add_end_unchecked_block_node(self.unchecked_block);
        cfg.start_end_pairs.insert(start_id, end_id);

        let mut last_link = start_id;

        if let Some(ASTNode::UncheckedBlock(block)) = context.nodes.get(&self.unchecked_block) {
            for statement in &block.statements {
                let node_id = cfg.add_statement_node(statement);
                cfg.add_flow_edge(last_link, node_id);
                last_link = node_id;
            }
        }

        cfg.add_flow_edge(last_link, end_id);

        (start_id, end_id)
    }
}

impl CfgUncheckedBlock {
    pub fn from(unchecked_block: &UncheckedBlock) -> Self {
        Self { unchecked_block: unchecked_block.id }
    }
}

/// Helper functions
impl Cfg {
    pub fn add_unchecked_block_node(&mut self, unchecked_block: &UncheckedBlock) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::UncheckedBlock(Box::new(CfgUncheckedBlock::from(
            unchecked_block,
        ))))
    }
}
