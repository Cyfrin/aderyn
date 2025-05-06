use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::Peek,
        workspace::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
    stats,
};
use eyre::Result;

#[derive(Default)]
pub struct TodoDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for TodoDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.contract_definitions() {
            let contract_as_ast: ASTNode = contract.into();
            if let Some(contract_code) = contract_as_ast.peek(context) {
                if contract_code.is_empty() {
                    continue;
                }
                let tokens = stats::token::tokenize(&contract_code);
                for token in tokens {
                    match token.token_type {
                        stats::token::TokenType::MultilineComment
                        | stats::token::TokenType::SinglelineComment => {
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

        Ok(!(self.found_instances.is_empty()))
    }

    fn title(&self) -> String {
        String::from("Contract has TODO Comments")
    }

    fn description(&self) -> String {
        String::from(
            "Contract contains comments with TODOS. Consider implementing or removing them.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::Todo)
    }
}

#[cfg(test)]
mod contracts_with_todos_tests {

    use crate::detect::detector::IssueDetector;

    use super::TodoDetector;

    #[test]

    fn test_contracts_with_todos_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ContractWithTodo.sol",
        );

        let mut detector = TodoDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
