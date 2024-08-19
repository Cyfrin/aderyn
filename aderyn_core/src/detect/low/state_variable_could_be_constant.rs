use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::context::graph::{CallGraph, CallGraphDirection};
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct StateVariableCouldBeConstantDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StateVariableCouldBeConstantDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // 1. Collect all state variables that are not marked constant (Set A)
        // 2. Investigate every function and collect all the state variables that could change (Set B)
        // 3. Result = Set A - Set B

        let mut set_a = HashSet::new();

        for variable in context.variable_declarations() {
            if variable.state_variable && !variable.constant {
                set_a.insert(variable.id);
            }
        }

        // let mut set_b = HashSet::new();

        for func in helpers::get_implemented_external_and_public_functions(context) {
            let investigator =
                CallGraph::new(context, &[&(func.into())], CallGraphDirection::Inward)?;
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State variable could be declared constant")
    }

    fn description(&self) -> String {
        String::from("Description of the low issue.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("low-issue-template")
    }
}

mod function_state_changes_finder_helper {
    use crate::{ast::FunctionDefinition, context::workspace_context::WorkspaceContext};


    impl FunctionDefinition {

        /// Investigates the function with the help callgraph and accumulates all the state variables
        /// that have been changed.
        pub fn state_variable_changes<'a>(
            &self,
            context: &'a WorkspaceContext,
        ) -> Option<ApproximateStorageChangeFinder<'a>> {
            let mut tracker = StateVariableChangeTracker {
                changes: None,
                context,
            };

            let investigator =
                CallGraph::new(context, &[&(self.into())], CallGraphDirection::Inward).ok()?;

            investigator.accept(context, &mut tracker).ok()?;

            tracker.changes.take()
        }

        struct StateVariableChangeTracker<'a> {
            context: &'a WorkspaceContext,
            changes: Option<ApproximateStorageChangeFinder<'a>>,
        }
    
        impl<'a> CallGraphVisitor for StateVariableChangeTracker<'a> {
            fn visit_any(&mut self, node: &ASTNode) -> eyre::Result<()> {
                let changes = ApproximateStorageChangeFinder::from(self.context, node);
                if self.changes.is_none() {
                    self.changes = Some(changes);
                } else if let Some(existing_changes) = self.changes.take() {
                    let new_changes = existing_changes + changes;
                    self.changes = Some(new_changes);
                }
                Ok(())
            }
        }
    }

}

#[cfg(test)]
mod state_variable_could_be_constant_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::state_variable_could_be_constant::StateVariableCouldBeConstantDetector,
    };

    #[test]
    #[serial]
    fn test_state_variable_could_be_declared_constant() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariableCouldBeDeclaredConstant.sol",
        );

        let mut detector = StateVariableCouldBeConstantDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
