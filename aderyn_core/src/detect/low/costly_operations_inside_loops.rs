use std::collections::BTreeMap;
use std::convert::identity;
use std::error::Error;

use crate::ast::{ASTNode, NodeID};

use crate::capture;
use crate::context::browser::ApproximateStorageChangeFinder;

use crate::context::graph::{CallGraph, CallGraphDirection, CallGraphVisitor};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct CostlyOperationsInsideLoopsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for CostlyOperationsInsideLoopsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Investigate for loops to check for storage writes
        for for_statement in context.for_statements() {
            if changes_state(context, &(for_statement.into())).is_some_and(identity) {
                capture!(self, context, for_statement);
            }
        }

        // Investigate while loops to check for storage writes
        for while_statement in context.while_statements() {
            if changes_state(context, &(while_statement.into())).is_some_and(identity) {
                capture!(self, context, while_statement);
            }
        }

        // Investigate the do while loops to check for storage writes
        for do_while_statement in context.do_while_statements() {
            if changes_state(context, &(do_while_statement.into())).is_some_and(identity) {
                capture!(self, context, do_while_statement);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Costly operations inside loops.")
    }

    fn description(&self) -> String {
        String::from("Invoking `SSTORE`operations in loops may lead to Out-of-gas errors. Use a local variable to hold the loop computation result.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::CostlyOperationsInsideLoops.to_string()
    }
}

fn changes_state(context: &WorkspaceContext, ast_node: &ASTNode) -> Option<bool> {
    // Now, investigate the function to see if there is scope for any state variable changes
    let mut tracker = StateVariableChangeTracker {
        state_var_has_changed: false,
        context,
    };
    let investigator = CallGraph::new(context, &[ast_node], CallGraphDirection::Inward).ok()?;
    investigator.accept(context, &mut tracker).ok()?;
    Some(tracker.state_var_has_changed)
}

struct StateVariableChangeTracker<'a> {
    state_var_has_changed: bool,
    context: &'a WorkspaceContext,
}

impl<'a> CallGraphVisitor for StateVariableChangeTracker<'a> {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        if self.state_var_has_changed {
            return Ok(());
        }
        // Check for state variable changes
        let finder = ApproximateStorageChangeFinder::from(self.context, node);
        if finder.state_variables_have_been_manipulated() {
            self.state_var_has_changed = true;
        }
        Ok(())
    }
}

#[cfg(test)]
mod costly_operations_inside_loops {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::costly_operations_inside_loops::CostlyOperationsInsideLoopsDetector,
    };

    #[test]
    #[serial]
    fn test_constly_operations_inside_loops() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CostlyOperationsInsideLoops.sol",
        );

        let mut detector = CostlyOperationsInsideLoopsDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
