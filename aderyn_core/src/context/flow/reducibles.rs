use crate::ast::Block;

use super::{AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId, CfgReduce};

// Control flow graph definitions nodes
#[derive(Debug, Clone)]
pub struct CfgBlock {
    pub block: AstNodeId,
}

impl CfgReduce for CfgBlock {
    fn reduce(&self, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId) {
        let start_id = cfg.add_start_node();
        let end_id = cfg.add_end_node();
        cfg.add_flow_edge(start_id, end_id);

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
