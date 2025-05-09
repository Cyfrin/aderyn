use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::ast::{FunctionCallKind, Mutability, NodeID};

use crate::{
    capture,
    context::{browser::ExtractFunctionCalls, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};

#[derive(Default)]
pub struct StateVariableCouldBeConstantDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StateVariableCouldBeConstantDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // 1. Collect all state variables that are not marked constant or immutable and are also not
        //    structs/mappings/contracts (collection A)
        // 2. Investigate every function and collect all the state variables that could change
        //    (collection B)
        // 3. Result = collection A - collection B

        let mut collection_a = Vec::new();

        for variable in context.variable_declarations() {
            // If we're not able to set the value upfront, then it cannot be constant
            if variable.value.is_none() {
                continue;
            }

            if let Some(rhs_value) = variable.value.as_ref() {
                let function_calls = ExtractFunctionCalls::from(rhs_value).extracted;
                if function_calls.iter().any(|f| f.kind == FunctionCallKind::FunctionCall) {
                    continue;
                }
            }

            if variable.mutability() == Some(&Mutability::Immutable) {
                continue;
            }

            // Do not report it if it's a struct / mapping
            if variable.type_descriptions.type_string.as_ref().is_some_and(|type_string| {
                type_string.starts_with("mapping") || type_string.starts_with("struct")
            }) {
                continue;
            }

            if variable.overrides.is_some() {
                continue;
            }

            if variable.state_variable && !variable.constant {
                collection_a.push(variable);
            }
        }

        let mut all_state_changes = None;
        for func in context.function_definitions() {
            if let Some(changes) = func.state_variable_changes(context) {
                if all_state_changes.is_none() {
                    all_state_changes = Some(changes);
                } else if let Some(existing_changes) = all_state_changes {
                    let new_changes = existing_changes + changes;
                    all_state_changes = Some(new_changes);
                }
            }
        }

        if let Some(all_state_changes) = all_state_changes {
            let collection_b = all_state_changes.fetch_non_exhaustive_manipulated_state_variables();
            let collection_b_ids: HashSet<_> = collection_b.into_iter().map(|v| v.id).collect();

            // RESULT =  collection A - collection B
            for variable in collection_a {
                if !collection_b_ids.contains(&variable.id) {
                    capture!(self, context, variable);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State Variable Could Be Constant")
    }

    fn description(&self) -> String {
        String::from("State variables that are not updated following deployment should be declared constant to save gas. Add the `constant` attribute to state variables that never change.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::StateVariableCouldBeConstant)
    }
}

mod function_state_changes_finder_helper {
    use crate::{
        ast::{ASTNode, FunctionDefinition},
        context::{
            browser::ApproximateStorageChangeFinder,
            graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
            workspace::WorkspaceContext,
        },
    };

    impl FunctionDefinition {
        /// Investigates the function with the help callgraph and accumulates all the state
        /// variables that have been changed.
        pub fn state_variable_changes<'a>(
            &self,
            context: &'a WorkspaceContext,
        ) -> Option<ApproximateStorageChangeFinder<'a>> {
            let mut all_changes = None;
            let callgraphs =
                CallGraphConsumer::get(context, &[&(self.into())], CallGraphDirection::Inward)
                    .ok()?;

            for callgraph in callgraphs {
                let mut tracker = StateVariableChangeTracker { changes: None, context };
                callgraph.accept(context, &mut tracker).ok()?;
                if let Some(changes) = tracker.changes.take() {
                    if all_changes.is_none() {
                        all_changes = Some(changes);
                    } else if let Some(existing_changes) = all_changes {
                        all_changes = Some(existing_changes + changes);
                    }
                }
            }

            all_changes
        }
    }

    struct StateVariableChangeTracker<'a> {
        context: &'a WorkspaceContext,
        changes: Option<ApproximateStorageChangeFinder<'a>>,
    }

    impl CallGraphVisitor for StateVariableChangeTracker<'_> {
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

#[cfg(test)]
mod state_variable_could_be_constant_tests {

    use crate::detect::{
        detector::IssueDetector,
        low::state_variable_could_be_constant::StateVariableCouldBeConstantDetector,
    };

    #[test]

    fn test_state_variable_could_be_declared_constant() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariableCouldBeDeclaredConstant.sol",
        );

        let mut detector = StateVariableCouldBeConstantDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
