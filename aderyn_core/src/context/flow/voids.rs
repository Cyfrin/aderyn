// Void definition nodes
use super::{AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId};

#[derive(Debug, Clone)]
pub enum CfgStartNode {
    Start,
    StartBlock(AstNodeId),
}

#[derive(Debug, Clone)]
pub enum CfgEndNode {
    End,
    EndBlock(AstNodeId),
}

/// Helper functions
impl Cfg {
    pub fn add_start_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::Start)))
    }
    pub fn add_end_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::End)))
    }
    pub fn add_start_block_node(&mut self, block: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(
            CfgStartNode::StartBlock(block),
        )))
    }
    pub fn add_end_block_node(&mut self, block: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndBlock(
            block,
        ))))
    }
}
