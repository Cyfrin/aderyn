// Void definition nodes
use super::{AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId};

#[derive(Debug, Clone)]
pub enum CfgStartNode {
    Start,
    StartFunctionBody(AstNodeId),   // Function Definition ID
    StartModifierBody(AstNodeId),   // Modifier Definition ID
    StartBlock(AstNodeId),          // Block Node ID
    StartUncheckedBlock(AstNodeId), // Unchecked Block ID
    StartIf(AstNodeId),             // If Statemtnt ID
    StartIfCond,
    StartIfTrue,
    StartIfFalse,
    StartWhile(AstNodeId), // While Statement ID
    StartWhileCond,
    StartWhileBody,
    StartFor(AstNodeId), // For Statement ID
    StartForInitExp,
    StartForCond,
    StartForLoopExp,
    StartForBody,
    StartDoWhile(AstNodeId), // Do While Statement ID
    StartDoWhileCond,
    StartDoWhileBody,
}

#[derive(Debug, Clone)]
pub enum CfgEndNode {
    End,
    EndFunctionBody(AstNodeId),
    EndModifierBody(AstNodeId),
    EndBlock(AstNodeId),
    EndUncheckedBlock(AstNodeId),
    EndIf(AstNodeId),
    EndIfCond,
    EndIfTrue,
    EndIfFalse,
    EndWhile(AstNodeId),
    EndWhileCond,
    EndWhileBody,
    EndFor(AstNodeId),
    EndForInitExp,
    EndForCond,
    EndForLoopExp,
    EndForBody,
    EndDoWhile(AstNodeId),
    EndDoWhileCond,
    EndDoWhileBody,
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
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartBlock(block))))
    }
    pub fn add_end_block_node(&mut self, block: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndBlock(block))))
    }
    pub fn add_start_if_node(&mut self, if_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartIf(if_stmt))))
    }
    pub fn add_start_if_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartIfCond)))
    }
    pub fn add_start_if_true_branch_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartIfTrue)))
    }
    pub fn add_start_if_false_branch_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartIfFalse)))
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
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartWhile(while_stmt))))
    }
    pub fn add_start_while_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartWhileCond)))
    }
    pub fn add_start_while_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartWhileBody)))
    }
    pub fn add_end_while_node(&mut self, while_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndWhile(while_stmt))))
    }
    pub fn add_end_while_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndWhileCond)))
    }
    pub fn add_end_while_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndWhileBody)))
    }
    pub fn add_start_for_node(&mut self, for_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartFor(for_stmt))))
    }
    pub fn add_start_for_init_exp_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartForInitExp)))
    }
    pub fn add_start_for_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartForCond)))
    }
    pub fn add_start_for_exp_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartForLoopExp)))
    }
    pub fn add_start_for_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartForBody)))
    }
    pub fn add_end_for_node(&mut self, for_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndFor(for_stmt))))
    }
    pub fn add_end_for_init_exp_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndForInitExp)))
    }
    pub fn add_end_for_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndForCond)))
    }
    pub fn add_end_for_exp_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndForLoopExp)))
    }
    pub fn add_end_for_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndForBody)))
    }
    pub fn add_start_do_while_node(&mut self, do_while_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartDoWhile(do_while_stmt))))
    }
    pub fn add_start_do_while_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartDoWhileCond)))
    }
    pub fn add_start_do_while_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartDoWhileBody)))
    }
    pub fn add_end_do_while_node(&mut self, do_while_stmt: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndDoWhile(do_while_stmt))))
    }
    pub fn add_end_do_while_cond_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndDoWhileCond)))
    }
    pub fn add_end_do_while_body_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndDoWhileBody)))
    }
    pub fn add_start_unchecked_block_node(&mut self, unchecked_block: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartUncheckedBlock(
            unchecked_block,
        ))))
    }
    pub fn add_end_unchecked_block_node(&mut self, unchecked_block: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndUncheckedBlock(
            unchecked_block,
        ))))
    }
    pub fn add_start_function_body_node(&mut self, function_definition_id: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartFunctionBody(
            function_definition_id,
        ))))
    }
    pub fn add_end_function_body_node(&mut self, function_definition_id: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndFunctionBody(
            function_definition_id,
        ))))
    }
    pub fn add_start_modifier_body_node(&mut self, modifier_definition_id: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start(Box::new(CfgStartNode::StartModifierBody(
            modifier_definition_id,
        ))))
    }
    pub fn add_end_modifier_body_node(&mut self, modifier_definition_id: AstNodeId) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End(Box::new(CfgEndNode::EndModifierBody(
            modifier_definition_id,
        ))))
    }
}
