use crate::ast::Block;

use super::{AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId};

// Control flow graph definitions nodes
#[derive(Debug)]
pub struct CfgBlock {
    pub block: AstNodeId,
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
