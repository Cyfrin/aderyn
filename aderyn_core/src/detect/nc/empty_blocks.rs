use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{FunctionKind, NodeID},
    capture,
    context::{
        browser::GetImmediateParent,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct EmptyBlockDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for EmptyBlockDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for empty_block in context.blocks().iter().filter(|b| b.statements.is_empty()) {
            if let Some(ASTNode::FunctionDefinition(f)) = &empty_block.parent(context) {
                // It's okay to have empty block if it's a constructor, receive, or fallback
                if f.kind == FunctionKind::Function {
                    capture!(self, context, empty_block);
                }
            } else {
                capture!(self, context, empty_block);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Empty Block - No statements found!")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
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
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::EmptyBlockDetector;

    #[test]
    fn test_empty_block() {
        let context =
            load_contract("../tests/contract-playground/out/AdminContract.sol/AdminContract.json");

        let mut detector = EmptyBlockDetector::default();
        // assert that the detector finds something
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Empty Block - No statements found!")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
