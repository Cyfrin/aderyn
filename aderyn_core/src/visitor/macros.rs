macro_rules! generate_visit_methods_for_workspace_context_with_insert_node {
    ($( $method_name:ident | $node:ident => $map_name:ident | ),* $(,)*) => {
        $(
            fn $method_name(&mut self, node: &$node) -> Result<bool> {
                self.nodes
                    .insert(node.id, ASTNode::$node(node.clone()));
                self.$map_name.insert(
                    node.clone(),
                    NodeContext {
                        source_unit_id: self.last_source_unit_id,
                        contract_definition_id: self.last_contract_definition_id,
                        function_definition_id: self.last_function_definition_id,
                        modifier_definition_id: self.last_modifier_definition_id,
                    },
                );
                Ok(true)
            }
        )*
    };
}

pub(crate) use generate_visit_methods_for_workspace_context_with_insert_node;
