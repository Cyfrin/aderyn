use std::collections::HashMap;

use eyre::Result;

use solc_ast::{ast::*, visitor::ast_visitor::*};

#[derive(Debug)]
pub enum ASTNode {
    SourceUnit(SourceUnit),
    Contract(ContractDefinition),
    StateVariable(VariableDeclaration),
    ConstantVariable(VariableDeclaration),
    ImmutableVariable(VariableDeclaration),
    NonStateVariable(VariableDeclaration),
    Function(FunctionDefinition),
    Modifier(ModifierDefinition),
    FunctionCall(FunctionCall),
    MemberAccess(MemberAccess)
}

#[derive(Default, Debug)]
pub struct ContractContext {
    pub nodes: HashMap<i64, ASTNode>,
}

impl ContractContext {
    pub fn get(&self, id: i64) -> Option<&ASTNode> {
        self.nodes.get(&id)
    }
}

impl ASTConstVisitor for ContractContext {

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        println!("FIRST PASS");
        self.nodes.insert(node.id, ASTNode::SourceUnit(node.clone()));
        Ok(true)
    }

    fn end_visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<()> {
        self.nodes.insert(node.id, ASTNode::Contract(node.clone()));

        // Iterate through every node in the contract definition
        for child in &node.nodes {
            // If the node is a variable declaration, then add to to either:
            // - state_variables
            // - constant_variables
            // - immutable_variables
            if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = child {
                let v = variable_declaration.clone();
                if v.constant == false && v.mutability == Some(Mutability::Mutable) {
                    self.nodes.insert(v.id, ASTNode::StateVariable(v));
                }
                else if v.mutability == Some(Mutability::Immutable) {
                    self.nodes.insert(v.id, ASTNode::ImmutableVariable(v));
                }
                else if v.constant == true {
                    self.nodes.insert(v.id, ASTNode::ConstantVariable(v));
                }
            }
        }

        Ok(())
    }

    fn end_visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<()> {
        if !node.state_variable {
            self.nodes.insert(node.id, ASTNode::NonStateVariable(node.clone()));
        }
        Ok(())
    }

    fn end_visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<()> {
        self.nodes.insert(node.id, ASTNode::Function(node.clone()));
        Ok(())
    }

    fn end_visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<()> {
        self.nodes.insert(node.id, ASTNode::Modifier(node.clone()));
        Ok(())
    }

    fn end_visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
        self.nodes.insert(node.id, ASTNode::FunctionCall(node.clone()));
        Ok(())
    }

    fn end_visit_member_access(&mut self, node: &MemberAccess) -> Result<()> {
        self.nodes.insert(node.id, ASTNode::MemberAccess(node.clone()));
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
            // std::fs::File::open("tests/ast-json/StateVariables.ast.json")?,
            std::fs::File::open("tests/ast-json/AbiEncodePacked.json")?,
        ))?)
    }

    #[test]
    fn test_contract_context() -> Result<()> {
        let source_unit = read_abi_encode_packed()?;
        let mut context = ContractContext::default();
        source_unit.accept(&mut context)?;
        context.nodes.into_iter().for_each(|(id, entry_type)| {
            println!("{}: {:?}", id, entry_type);
        });
        Ok(())
    }
}