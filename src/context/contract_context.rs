use std::collections::HashMap;

use eyre::Result;

use solc_ast::{ast::*, visitor::ast_visitor::*};

#[derive(Debug)]
pub enum EntryType {
    SourceUnit,
    Contract,
    StateVariable,
    ConstantVariable,
    ImmutableVariable,
    Function,
    Modifier,
}

#[derive(Default, Debug)]
pub struct ContractContext {
    // ids is a mapping from node id to entry type, so we can look up the type of a node by id
    pub ids: HashMap<i64, EntryType>,
    pub source_units: HashMap<i64, SourceUnit>,
    pub contracts: HashMap<i64, ContractDefinition>,
    // state, constant and immtable variables
    pub state_variables: HashMap<i64, VariableDeclaration>,
    pub constant_variables: HashMap<i64, VariableDeclaration>,
    pub immutable_variables: HashMap<i64, VariableDeclaration>,
    // functions
    pub functions: HashMap<i64, FunctionDefinition>,
    // modifiers
    pub modifiers: HashMap<i64, ModifierDefinition>,
} 

impl ASTConstVisitor for ContractContext {

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        self.source_units.insert(node.id, node.clone());
        self.ids.insert(node.id, EntryType::SourceUnit);
        Ok(true)
    }

    fn end_visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<()> {
        self.contracts.insert(node.id, node.clone());
        self.ids.insert(node.id, EntryType::Contract);

        // Iterate through every node in the contract definition
        for child in &node.nodes {
            // If the node is a variable declaration, then add to to either:
            // - state_variables
            // - constant_variables
            // - immutable_variables
            if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = child {
                let v = variable_declaration.clone();
                if v.constant == false && v.mutability == Some(Mutability::Mutable) {
                    self.ids.insert(v.id, EntryType::StateVariable);
                    self.state_variables.insert(v.id, v);
                }
                else if v.mutability == Some(Mutability::Immutable) {
                    self.ids.insert(v.id, EntryType::ImmutableVariable);
                    self.immutable_variables.insert(v.id, v);
                }
                else if v.constant == true {
                    self.ids.insert(v.id, EntryType::ConstantVariable);
                    self.constant_variables.insert(v.id, v);
                }
            }
        }

        Ok(())
    }

    fn end_visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<()> {
        self.state_variables.insert(node.id, node.clone());
        self.ids.insert(node.id, EntryType::StateVariable);
        Ok(())
    }

    fn end_visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<()> {
        self.functions.insert(node.id, node.clone());
        self.ids.insert(node.id, EntryType::Function);
        Ok(())
    }

    fn end_visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<()> {
        self.modifiers.insert(node.id, node.clone());
        self.ids.insert(node.id, EntryType::Modifier);
        Ok(())
    }
}

#[cfg(test)]
mod contract_context_tests {
    use eyre::Result;
    use solc_ast::{ast::*, visitor::ast_visitor::*};
    use crate::context::contract_context::ContractContext;

    fn read_abi_encode_packed() -> Result<SourceUnit> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("tests/ast-json/StateVariables.ast.json")?,
            // std::fs::File::open("tests/ast-json/AbiEncodePacked.json")?,
        ))?)
    }

    #[test]
    fn test_contract_context() -> Result<()> {
        let source_unit = read_abi_encode_packed()?;
        let mut context = ContractContext::default();
        source_unit.accept(&mut context)?;
        context.ids.into_iter().for_each(|(id, entry_type)| {
            println!("{}: {:?}", id, entry_type);
        });
        Ok(())
    }
}