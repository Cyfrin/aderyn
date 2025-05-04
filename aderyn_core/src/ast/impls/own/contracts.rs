use crate::ast::*;

impl ContractDefinitionNode {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                Some(using_for_directive.id)
            }
            ContractDefinitionNode::StructDefinition(struct_definition) => {
                Some(struct_definition.id)
            }
            ContractDefinitionNode::EnumDefinition(enum_definition) => Some(enum_definition.id),
            ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                Some(variable_declaration.id)
            }
            ContractDefinitionNode::EventDefinition(event_definition) => Some(event_definition.id),
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                Some(function_definition.id)
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                Some(modifier_definition.id)
            }
            ContractDefinitionNode::ErrorDefinition(error_definition) => Some(error_definition.id),
            ContractDefinitionNode::UserDefinedValueTypeDefinition(
                user_defined_value_type_definition,
            ) => Some(user_defined_value_type_definition.id),
        }
    }
}

impl ContractDefinition {
    pub fn function_definitions(&self) -> Vec<&FunctionDefinition> {
        let mut result = vec![];
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                result.push(function_definition);
            }
        }
        result
    }
}
