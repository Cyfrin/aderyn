use super::{Cfg, CfgNodeDescriptor, CfgNodeId};

/// Helper functions
impl Cfg {
    pub fn add_start_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start)
    }
    pub fn add_end_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End)
    }
}
