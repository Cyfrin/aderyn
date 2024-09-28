use crate::{ast::Block, context::workspace_context::WorkspaceContext};

use super::{ASTNode, AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId, CfgReduce};

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
