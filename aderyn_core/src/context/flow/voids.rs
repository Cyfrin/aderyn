// Void definition nodes
use super::{AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId};

#[derive(Debug, Clone)]
pub enum CfgStartNode {
    Start,
    StartBlock(AstNodeId),
    StartIf(AstNodeId),
    StartIfCond,
    StartIfTrue,
    StartIfFalse,
    StartWhile(AstNodeId),
    StartWhileCond,
    StartWhileBody,
}

#[derive(Debug, Clone)]
pub enum CfgEndNode {
    End,
    EndBlock(AstNodeId),
    EndIf(AstNodeId),
    EndIfCond,
    EndIfTrue,
    EndIfFalse,
    EndWhile(AstNodeId),
    EndWhileCond,
    EndWhileBody,
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
    pub fn add_start_if_node(&mut self, if_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartIf(
            if_stmt,
        ))))
    }
    pub fn add_start_if_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(
            CfgStartNode::StartIfCond,
        )))
    }
    pub fn add_start_if_true_branch_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(
            CfgStartNode::StartIfTrue,
        )))
    }
    pub fn add_start_if_false_branch_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(
            CfgStartNode::StartIfFalse,
        )))
    }
    pub fn add_end_if_node(&mut self, if_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndIf(if_stmt))))
    }
    pub fn add_end_if_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndIfCond)))
    }
    pub fn add_end_if_true_branch_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndIfTrue)))
    }
    pub fn add_end_if_false_branch_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndIfFalse)))
    }
    pub fn add_start_while_node(&mut self, while_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(
            CfgStartNode::StartWhile(while_stmt),
        )))
    }
    pub fn add_start_while_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(
            CfgStartNode::StartWhileCond,
        )))
    }
    pub fn add_start_while_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(
            CfgStartNode::StartWhileBody,
        )))
    }
    pub fn add_end_while_node(&mut self, while_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndWhile(
            while_stmt,
        ))))
    }
    pub fn add_end_while_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndWhileCond)))
    }
    pub fn add_end_while_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndWhileBody)))
    }
}
