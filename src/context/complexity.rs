use eyre::Result;

use crate::{
    ast::*,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

#[derive(Default, Debug)]
pub struct ComplexityLoader {
    pub complexity: i32,
}

impl ComplexityLoader {
    pub fn visit(&mut self, node: &SourceUnit) -> Result<i32> {
        self.complexity = 0;

        node.accept(self)?;

        Ok(self.complexity)
    }
}

impl ASTConstVisitor for ComplexityLoader {
    fn visit_if_statement(&mut self, _node: &IfStatement) -> Result<bool> {
        self.complexity += 1;
        println!("After IfStatement: {}", self.complexity);
        Ok(true)
    }

    fn visit_modifier_invocation(&mut self, _node: &ModifierInvocation) -> Result<bool> {
        self.complexity += 1;
        println!("After ModifierInvocation: {}", self.complexity);
        Ok(true)
    }

    fn visit_function_call(&mut self, _node: &FunctionCall) -> Result<bool> {
        self.complexity += 1;
        println!("After FunctionCall: {}", self.complexity);
        Ok(true)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        let mut comp = match Some(node.visibility) {
            Some(Visibility::Public) => 2,
            Some(Visibility::External) => 2,
            _ => 0,
        };
        if Some(node.state_mutability) == Some(StateMutability::Payable) {
            comp += 1;
        }
        self.complexity += comp;
        println!("After FunctionDefinition: {}", self.complexity);
        Ok(true)
    }

    fn visit_new_expression(&mut self, _node: &NewExpression) -> Result<bool> {
        self.complexity += 10;
        println!("After NewExpression: {}", self.complexity);
        Ok(true)
    }

    fn visit_for_statement(&mut self, _node: &ForStatement) -> Result<bool> {
        self.complexity += 5;
        println!("After ForStatement: {}", self.complexity);
        Ok(true)
    }

    fn visit_while_statement(&mut self, _node: &WhileStatement) -> Result<bool> {
        self.complexity += 1;
        println!("After WhileStatement: {}", self.complexity);
        Ok(true)
    }

    fn visit_inline_assembly(&mut self, _node: &InlineAssembly) -> Result<bool> {
        self.complexity += 2;
        println!("After InlineAssembly: {}", self.complexity);
        Ok(true)
    }

    fn visit_conditional(&mut self, _node: &Conditional) -> Result<bool> {
        self.complexity += 1;
        println!("After Conditional: {}", self.complexity);
        Ok(true)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        let mut comp = 1;
        for _base_contract in &node.base_contracts {
            comp += 2;
        }
        self.complexity += comp;
        println!("After ContractDefinition: {}", self.complexity);
        Ok(true)
    }

    fn visit_yul_function_call(&mut self, _node: &YulFunctionCall) -> Result<bool> {
        self.complexity += 3;
        println!("After YulFunctionCall: {}", self.complexity);
        Ok(true)
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        if node.state_variable {
            self.complexity += 1;
        }

        println!("After VariableDeclaration: {}", self.complexity);
        Ok(true)
    }
}

#[cfg(test)]
mod complexity_tests {
    use crate::detect::detector::detector_test_helpers::load_foundry_output;

    use super::ComplexityLoader;

    #[test]
    fn test_complexity_counter() {
        let foundry_output =
            load_foundry_output("./tests/contract-playground/out/Counter.sol/Counter.json");
        let mut complexity_loader = ComplexityLoader::default();
        let complexity = complexity_loader.visit(&foundry_output.ast).unwrap();
        println!("Complexity: {}", complexity);
        assert_eq!(complexity, 14);
    }

    #[test]
    fn test_complexity_extended_inheritance() {
        let foundry_output = load_foundry_output(
            "./tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        );
        let mut complexity_loader = ComplexityLoader::default();
        let complexity = complexity_loader.visit(&foundry_output.ast).unwrap();
        println!("Complexity: {}", complexity);
        assert_eq!(complexity, 18);
    }

    #[test]
    fn test_complexity_uniswap_v3_swapper() {
        let foundry_output = load_foundry_output(
            "./tests/contract-playground/out/UniswapV3Swapper.sol/UniswapV3Swapper.json",
        );
        let mut complexity_loader = ComplexityLoader::default();
        let complexity = complexity_loader.visit(&foundry_output.ast).unwrap();
        println!("Complexity: {}", complexity);
        assert_eq!(complexity, 29);
    }

    #[test]
    fn test_complexity_erc_721() {
        let foundry_output =
            load_foundry_output("./tests/contract-playground/out/ERC721.sol/ERC721.json");
        let mut complexity_loader = ComplexityLoader::default();
        let complexity = complexity_loader.visit(&foundry_output.ast).unwrap();
        println!("Complexity: {}", complexity);
        assert_eq!(complexity, 142);
    }

    #[test]
    fn test_complexity_erc_20_votes() {
        let foundry_output =
            load_foundry_output("./tests/contract-playground/out/ERC20Votes.sol/ERC20Votes.json");
        let mut complexity_loader = ComplexityLoader::default();
        let complexity = complexity_loader.visit(&foundry_output.ast).unwrap();
        println!("Complexity: {}", complexity);
        assert_eq!(complexity, 142);
    }
}
