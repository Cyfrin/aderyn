use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{FunctionKind, NodeID, NodeType},
    capture,
    context::{
        browser::{GetAncestralLine, GetClosestAncestorOfTypeX},
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct EmptyBlockDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for EmptyBlockDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for empty_block in context.blocks().iter().filter(|b| b.statements.is_empty()) {
            if let Some(ASTNode::FunctionDefinition(f)) =
                &empty_block.closest_ancestor_of_type(context, NodeType::FunctionDefinition)
            {
                // It's okay to have empty block if it's a constructor, receive, or fallback
                if f.kind == FunctionKind::Function {
                    capture!(self, context, f);
                } else if f.kind == FunctionKind::Constructor
                    || f.kind == FunctionKind::Receive
                    || f.kind == FunctionKind::Fallback
                {
                    // It's not okay to have empty block nested somewhere inside constructor
                    if let Some(block_chain) = empty_block.ancestral_line(context) {
                        let function_definition_index = block_chain
                            .iter()
                            .position(|x| x.node_type() == NodeType::FunctionDefinition)
                            .unwrap(); // Remember, we know we are already inside a constructor Function

                        //We start from going up from first parent to the Function definition
                        if function_definition_index > 1 {
                            // 1 here, means the first parent.
                            // So if the constructor is NOT the immediate parent of this empty block
                            // capture it!
                            capture!(self, context, empty_block);
                        }
                    }
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Empty Block")
    }

    fn description(&self) -> String {
        String::from("Consider removing empty blocks.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::EmptyBlock)
    }
}

#[cfg(test)]
mod empty_block_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        test_utils::{load_solidity_source_unit, take_loader_lock},
    };

    use super::EmptyBlockDetector;

    #[test]
    #[serial]
    fn test_empty_block_by_loading_contract_directly() {
        let _lock = take_loader_lock();
        let context = load_solidity_source_unit("../tests/contract-playground/src/EmptyBlocks.sol");

        let mut detector = EmptyBlockDetector::default();
        // assert that the detector finds something
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 7);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(detector.title(), String::from("Empty Block"));
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            String::from("Consider removing empty blocks.")
        );
    }
}
