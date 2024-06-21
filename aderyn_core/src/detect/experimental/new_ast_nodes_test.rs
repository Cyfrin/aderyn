use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct NewASTNodesDemonstrator {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for NewASTNodesDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for do_while_statement in context.do_while_statements() {
            capture!(self, context, do_while_statement);
        }

        for break_statement in context.break_statements() {
            capture!(self, context, break_statement);
        }

        for continue_statements in context.continue_statements() {
            capture!(self, context, continue_statements);
        }

        for placeholder_statement in context.placeholder_statements() {
            capture!(self, context, placeholder_statement);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("New AST Nodes Demo")
    }

    fn description(&self) -> String {
        String::from("New AST Nodes Demo")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod new_ast_nodes_demonstrator_tests {
    use crate::detect::{
        detector::IssueDetector, experimental::new_ast_nodes_test::NewASTNodesDemonstrator,
    };

    use serial_test::serial;

    #[test]
    #[serial]
    fn test_new_ast_nodes() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/adhoc-sol-files/DemoASTNodes.sol",
        );

        let mut detector = NewASTNodesDemonstrator::default();
        let _ = detector.detect(&context).unwrap();

        let instances = detector.instances();
        println!("{:?}", instances);

        assert!(instances.len() == 4);
    }
}
