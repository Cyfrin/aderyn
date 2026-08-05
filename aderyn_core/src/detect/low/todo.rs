use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
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

                // `contract_code` is only a slice of the file (starting at the
                // contract's `src` offset), so token positions coming out of
                // the tokenizer are *relative* to it. Resolve the contract's
                // absolute (file, line, byte-offset) so we can translate each
                // TODO token back into an absolute file location instead of
                // always pointing at the contract itself.
                let (file_path, contract_start_line, contract_src_location) =
                    context.get_node_sort_key_pure(&contract_as_ast);
                let Some((contract_offset_str, _)) = contract_src_location.split_once(':') else {
                    continue;
                };
                let Ok(contract_offset) = contract_offset_str.parse::<usize>() else {
                    continue;
                };

                let line_start_offsets = line_start_byte_offsets(&contract_code);

                let tokens = stats::token::tokenize(&contract_code);
                for token in tokens {
                    match token.token_type {
                        stats::token::TokenType::MultilineComment
                        | stats::token::TokenType::SinglelineComment => {
                            if token.content.to_lowercase().contains("todo") {
                                // Line number of the comment, relative to the contract, is
                                // 1-indexed; translate it to an absolute line in the file.
                                let relative_line = token.start_line;
                                let absolute_line =
                                    contract_start_line + relative_line.saturating_sub(1);

                                // Locate the byte offset (within the file) where the
                                // comment starts, so the reported snippet/range points
                                // at the TODO comment rather than the contract.
                                let line_start = line_start_offsets
                                    .get(relative_line.saturating_sub(1))
                                    .copied()
                                    .unwrap_or(0);
                                let line_end = line_start_offsets
                                    .get(relative_line)
                                    .copied()
                                    .unwrap_or(contract_code.len());
                                let line_slice =
                                    contract_code.get(line_start..line_end).unwrap_or("");
                                let leading_whitespace =
                                    line_slice.len() - line_slice.trim_start().len();
                                let token_offset_in_contract = line_start + leading_whitespace;
                                let absolute_offset = contract_offset + token_offset_in_contract;

                                let key = (
                                    file_path.clone(),
                                    absolute_line,
                                    format!("{absolute_offset}:{}", token.content.len()),
                                );
                                self.found_instances.insert(key, contract.id);
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

/// Returns the byte offset (within `content`) at which each line starts.
/// `offsets[i]` is the byte offset of line `i + 1` (1-indexed lines).
fn line_start_byte_offsets(content: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, c) in content.char_indices() {
        if c == '\n' {
            offsets.push(i + 1);
        }
    }
    offsets
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

        // ContractWithTodo.sol has 4 separate TODO comments (lines 8, 9, 14, 15).
        // Each one should be reported individually, on its own line, instead of
        // being collapsed into a single instance pointing at the contract
        // definition (line 4).
        let instances = detector.instances();

        println!("Found {} TODO instances:", instances.len());
        for (key, value) in instances.iter() {
            println!("{:?} => {:?}", key, value);
        }
        assert_eq!(instances.len(), 4);

        let reported_lines: std::collections::BTreeSet<usize> =
            instances.keys().map(|(_, line, _)| *line).collect();
        assert_eq!(
            reported_lines,
            std::collections::BTreeSet::from([8, 9, 14, 15]),
            "expected the TODO detector to flag the exact lines of each comment"
        );
    }
}
