use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::Peek,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
    fscloc,
};
use eyre::Result;

#[derive(Default)]
pub struct ContractsWithTodosDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ContractsWithTodosDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.contract_definitions() {
            let contract_as_ast: ASTNode = contract.into();
            if let Some(contract_code) = contract_as_ast.peek(context) {
                if contract_code.is_empty() {
                    continue;
                }
                let tokens = fscloc::token::tokenize(&contract_code);
                for token in tokens {
                    match token.token_type {
                        fscloc::token::TokenType::MultilineComment
                        | fscloc::token::TokenType::SinglelineComment => {
                            if token.content.to_lowercase().contains("todo") {
                                capture!(self, context, contract);
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Contract still has TODOs")
    }

    fn description(&self) -> String {
        String::from("Contract contains comments with TODOS")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ContractWithTodos)
    }
}

#[cfg(test)]
mod contracts_with_todos {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::ContractsWithTodosDetector;

    #[test]
    #[serial]
    fn test_contracts_with_todos_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ContractWithTodo.sol",
        );

        let mut detector = ContractsWithTodosDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
