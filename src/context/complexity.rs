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
        Ok(true)
    }

    fn visit_modifier_invocation(&mut self, _node: &ModifierInvocation) -> Result<bool> {
        self.complexity += 1;
        Ok(true)
    }

    fn visit_function_call(&mut self, _node: &FunctionCall) -> Result<bool> {
        self.complexity += 1;
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
        Ok(true)
    }

    fn visit_new_expression(&mut self, _node: &NewExpression) -> Result<bool> {
        self.complexity += 10;
        Ok(true)
    }

    fn visit_for_statement(&mut self, _node: &ForStatement) -> Result<bool> {
        self.complexity += 5;
        Ok(true)
    }

    fn visit_while_statement(&mut self, _node: &WhileStatement) -> Result<bool> {
        self.complexity += 1;
        Ok(true)
    }

    fn visit_inline_assembly(&mut self, _node: &InlineAssembly) -> Result<bool> {
        self.complexity += 2;
        Ok(true)
    }

    fn visit_conditional(&mut self, _node: &Conditional) -> Result<bool> {
        self.complexity += 1;
        Ok(true)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        let mut comp = 1;
        while let Some(_base_contract) = Some(&node.base_contracts) {
            comp += 2;
        }
        self.complexity += comp;
        Ok(true)
    }
}

#[cfg(test)]
mod complexity_tests {
    use crate::detect::detector::detector_test_helpers::load_foundry_output;

    use super::ComplexityLoader;

    #[test]
    fn test_complexity() {
        let foundry_output =
            load_foundry_output("./tests/contract-playground/out/Counter.sol/Counter.json");
        let mut complexity_loader = ComplexityLoader::default();
        let complexity = complexity_loader.visit(&foundry_output.ast).unwrap();
        assert_eq!(complexity, 14);
        println!("Complexity: {}", complexity);
    }
}
