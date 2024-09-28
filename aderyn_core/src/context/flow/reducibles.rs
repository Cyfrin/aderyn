use crate::{ast::Block, context::workspace_context::WorkspaceContext};

use super::{ASTNode, AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId, CfgReduce, Statement};

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
                match statement {
                    Statement::IfStatement(_)
                    | Statement::ForStatement(_)
                    | Statement::WhileStatement(_)
                    | Statement::EmitStatement(_)
                    | Statement::TryStatement(_)
                    | Statement::UncheckedBlock(_)
                    | Statement::Return(_)
                    | Statement::RevertStatement(_)
                    | Statement::InlineAssembly(_)
                    | Statement::Break(_)
                    | Statement::Continue(_)
                    | Statement::DoWhileStatement(_)
                    | Statement::PlaceholderStatement(_) => unimplemented!(),
                    Statement::Block(n) => {
                        let node_id = cfg.add_block_node(n);
                        cfg.add_flow_edge(last_link, node_id);
                        last_link = node_id;
                    }
                    Statement::VariableDeclarationStatement(n) => {
                        let node_id = cfg.add_variable_declaration_statement(n);
                        cfg.add_flow_edge(last_link, node_id);
                        last_link = node_id;
                    }
                    Statement::ExpressionStatement(n) => {
                        let node_id = cfg.add_expression_statement(n);
                        cfg.add_flow_edge(last_link, node_id);
                        last_link = node_id;
                    }
                }
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
