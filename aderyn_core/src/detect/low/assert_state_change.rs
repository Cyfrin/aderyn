use std::{collections::BTreeMap, convert::identity, error::Error};

use crate::ast::{Expression, Identifier, NodeID};

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct AssertStateChangeDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for AssertStateChangeDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for function_call in context.function_calls() {
            if let Expression::Identifier(Identifier { name, .. }) =
                function_call.expression.as_ref()
            {
                if name == "assert"
                    && function_call.arguments_change_contract_state(context).is_some_and(identity)
                {
                    capture!(self, context, function_call);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State change in `assert()` statement")
    }

    fn description(&self) -> String {
        String::from("An argument to `assert()` modifies the state. Use `require` for invariants modifying state.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::AssertStateChange)
    }
}

mod assert_state_change_tracker {
    use crate::{
        ast::{ASTNode, FunctionCall},
        context::{
            browser::ApproximateStorageChangeFinder,
            graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
            workspace::WorkspaceContext,
        },
    };

    struct StateVariableChangeTracker<'a> {
        has_some_state_variable_changed: bool,
        context: &'a WorkspaceContext,
    }

    impl CallGraphVisitor for StateVariableChangeTracker<'_> {
        fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
            if self.has_some_state_variable_changed {
                return Ok(());
            }
            let finder = ApproximateStorageChangeFinder::from(self.context, node);
            if finder.state_variables_have_been_manipulated() {
                self.has_some_state_variable_changed = true;
            }
            Ok(())
        }
    }

    impl FunctionCall {
        pub fn arguments_change_contract_state(&self, context: &WorkspaceContext) -> Option<bool> {
            let arguments =
                self.arguments.clone().into_iter().map(|n| n.into()).collect::<Vec<ASTNode>>();
            let ast_nodes: &[&ASTNode] = &(arguments.iter().collect::<Vec<_>>());

            let callgraphs =
                CallGraphConsumer::get(context, ast_nodes, CallGraphDirection::Inward).ok()?;

            for callgraph in callgraphs {
                let mut tracker =
                    StateVariableChangeTracker { has_some_state_variable_changed: false, context };
                callgraph.accept(context, &mut tracker).ok()?;
                if tracker.has_some_state_variable_changed {
                    return Some(true);
                }
            }

            Some(false)
        }
    }
}

#[cfg(test)]
mod asert_state_changes_tests {

    use crate::detect::{
        detector::IssueDetector, low::assert_state_change::AssertStateChangeDetector,
    };

    #[test]

    fn test_assert_state_change() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/AssertStateChange.sol",
        );

        let mut detector = AssertStateChangeDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
