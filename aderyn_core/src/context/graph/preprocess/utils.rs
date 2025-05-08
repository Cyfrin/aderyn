use crate::{
    ast::{ASTNode, NodeID, NodeType},
    context::{browser::GetClosestAncestorOfTypeX, workspace::WorkspaceContext},
};

impl WorkspaceContext {
    pub fn find_potential_base_contracts(&self, nodes: &[&ASTNode]) -> Vec<NodeID> {
        let mut potential: Vec<NodeID> = vec![];
        for &node in nodes {
            let Some(func_id) = containing_function_or_modifier(node, self) else {
                continue;
            };
            let cg = self.callgraphs.as_ref().expect("callgraph not found");
            for (contract_id, graph) in &cg.raw_callgraphs {
                if graph.contains_key(&func_id) {
                    potential.push(*contract_id);
                }
            }
        }
        potential
    }
}

#[inline]
fn containing_function_or_modifier(node: &ASTNode, context: &WorkspaceContext) -> Option<NodeID> {
    if matches!(node.node_type(), NodeType::FunctionDefinition | NodeType::ModifierDefinition) {
        return node.id();
    }
    node.closest_ancestor_of_type(context, NodeType::FunctionDefinition)
        .or_else(|| node.closest_ancestor_of_type(context, NodeType::ModifierDefinition))?
        .id()
}
