mod load_source_unit;

use crate::{
    ast::{ContractDefinition, FunctionDefinition, FunctionKind, ModifierDefinition, NodeID},
    context::{browser::ExtractVariableDeclarations, workspace::WorkspaceContext},
};

// Using `solc` to read AST given a source unit (i.e Solidity file)
pub use load_source_unit::{
    load_multiple_solidity_source_units_into_single_context, load_playground_solidity_source_units,
    load_solidity_source_unit,
};

impl WorkspaceContext {
    pub fn find_contract_by_name(&self, name: &str) -> &ContractDefinition {
        self.contract_definitions().into_iter().find(|c| c.name.as_str() == name).unwrap()
    }
    pub fn find_free_function_by_name(&self, name: &str) -> &FunctionDefinition {
        self.function_definitions()
            .iter()
            .filter(|func| *func.kind() == FunctionKind::FreeFunction)
            .find(|func| func.name == name)
            .unwrap()
    }
}

impl ContractDefinition {
    pub fn find_function_by_name(&self, name: &str) -> &FunctionDefinition {
        self.function_definitions().iter().find(|func| func.name == name).unwrap()
    }

    pub fn find_receive_function(&self) -> &FunctionDefinition {
        self.function_definitions()
            .iter()
            .find(|func| *func.kind() == FunctionKind::Receive)
            .unwrap()
    }

    pub fn find_fallback_function(&self) -> &FunctionDefinition {
        self.function_definitions()
            .iter()
            .find(|func| *func.kind() == FunctionKind::Fallback)
            .unwrap()
    }

    pub fn find_modifier_by_name(&self, name: &str) -> &ModifierDefinition {
        self.modifier_definitions().iter().find(|modifier| modifier.name == name).unwrap()
    }

    pub fn find_state_variable_node_id_by_name(&self, name: &str) -> NodeID {
        let variable_declarations = ExtractVariableDeclarations::from(self).extracted;
        let variable = variable_declarations
            .into_iter()
            .filter(|v| v.state_variable && v.name == name)
            .collect::<Vec<_>>();
        variable.first().unwrap().id
    }
}
