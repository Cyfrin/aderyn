use crate::{ast::Block, context::workspace_context::WorkspaceContext};

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

        let Some(ASTNode::IfStatement(if_ast_node)) = context.nodes.get(&self.if_statement) else {
            cfg.add_flow_edge(start_id, end_id);
            return (start_id, end_id);
        };

        // Condition node
        let start_cond = cfg.add_start_if_cond_node();
        let end_cond = cfg.add_end_if_cond_node();
        let condition = cfg.add_if_statement_condition(&if_ast_node.condition);

        cfg.add_flow_edge(start_id, start_cond);
        cfg.add_flow_edge(start_cond, condition);
        cfg.add_flow_edge(condition, end_cond);

        // True branch
        let start_true_branch = cfg.add_start_if_true_branch_node();
        let end_true_branch = cfg.add_end_if_true_branch_node();

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

            cfg.add_flow_edge(end_cond, start_false_branch);
            cfg.add_flow_edge(start_false_branch, end_false_branch);

            cfg.add_flow_edge(end_false_branch, end_id);
        }

        (start_id, end_id)
    }
}

impl CfgIfStatement {
    pub fn from(if_stmt: &IfStatement) -> Self {
        Self {
            if_statement: if_stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_if_statement(&mut self, if_stmt: &IfStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::IfStatement(Box::new(
            CfgIfStatement::from(if_stmt),
        )))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgWhileStatement {
    while_statement: AstNodeId,
}

impl CfgReduce for CfgWhileStatement {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_while_node(self.while_statement);
        let end_id = cfg.add_end_while_node(self.while_statement);

        let Some(ASTNode::WhileStatement(ast_while_stmt)) =
            context.nodes.get(&self.while_statement)
        else {
            cfg.add_flow_edge(start_id, end_id);
            return (start_id, end_id);
        };

        let start_cond = cfg.add_start_while_cond_node();
        let end_cond = cfg.add_end_while_cond_node();
        let condition = cfg.add_while_statement_condition(&ast_while_stmt.condition);

        cfg.add_flow_edge(start_id, start_cond);
        cfg.add_flow_edge(start_cond, condition);
        cfg.add_flow_edge(condition, end_cond);

        // Exit happens from the condition node
        cfg.add_flow_edge(end_cond, end_id);

        // Loop arcs around the condition
        let start_body = cfg.add_start_while_body_node();
        let end_body = cfg.add_end_while_body_node();
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
        Self {
            while_statement: while_stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_while_statement(&mut self, while_stmt: &WhileStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::WhileStatement(Box::new(
            CfgWhileStatement::from(while_stmt),
        )))
    }
}
