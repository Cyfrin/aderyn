use std::{collections::BTreeMap, convert::identity, error::Error};

use crate::ast::{ASTNode, NodeID};

use crate::{
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct CacheArrayLengthDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for CacheArrayLengthDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN -
        //
        // First, look at the condition of the for loop, if it contains `<state_variable>.length`
        // see if it's possible to cache it.
        //
        // Investigate the body of the loop to see if anywhere the said state variable is
        // manipulated. If no manipulations, it means that the state variable could be
        // cached.
        //

        for for_loop in context.for_statements() {
            if let Some(changes) = for_loop.state_variable_changes(context) {
                // Find all the storage arrays on which `.length` is checked in for loop's
                // condition
                let state_vars =
                    for_loop.state_variables_lengths_that_are_referenced_in_condition(context);

                // Now see if any of the storage array has been manipulated. If yes, then it doesn't
                // qualify for caching
                let they_are_not_manipulated_in_the_for_loop =
                    state_vars.iter().all(|state_var_id| {
                        if let Some(ASTNode::VariableDeclaration(var)) =
                            context.nodes.get(state_var_id)
                        {
                            if changes
                                .state_variable_has_not_been_manipulated(var)
                                .is_some_and(identity)
                            {
                                return true;
                            }
                        }
                        false
                    });

                // Here, we know that none of the storage arrays whose length was referenced,
                // changes in the loop So we report them as potential caches.
                if !state_vars.is_empty() && they_are_not_manipulated_in_the_for_loop {
                    capture!(self, context, for_loop);
                }
            }
        }

        // TODO - After sorting out helper modules, extend this logic to other kinds of loops
        // For slither parity, only for loops is fine.

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Storage Array Length not Cached")
    }

    fn description(&self) -> String {
        String::from(
            "Calling `.length` on a storage array in a loop condition is expensive. Consider caching the length in a local variable in memory before the loop and reusing it.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::StorageArrayLengthNotCached)
    }
}

mod loop_investigation_helper {
    use std::collections::BTreeSet;

    use crate::{
        ast::{ASTNode, Expression, ForStatement, Identifier, NodeID, TypeDescriptions},
        context::{
            browser::{ApproximateStorageChangeFinder, ExtractMemberAccesses},
            graph::{CallGraph, CallGraphDirection, CallGraphVisitor},
            workspace_context::WorkspaceContext,
        },
    };

    impl ForStatement {
        pub fn state_variables_lengths_that_are_referenced_in_condition(
            &self,
            context: &WorkspaceContext,
        ) -> BTreeSet<NodeID> {
            let mut state_vars_lengths_that_are_referenced = BTreeSet::new();

            if let Some(condition) = self.condition.as_ref() {
                let member_accesses = ExtractMemberAccesses::from(condition).extracted;
                for member_access in member_accesses {
                    if member_access.member_name != "length" {
                        continue;
                    }
                    if let Expression::Identifier(Identifier {
                        referenced_declaration: Some(id),
                        type_descriptions: TypeDescriptions { type_string: Some(type_string), .. },
                        ..
                    }) = member_access.expression.as_ref()
                    {
                        if let Some(ASTNode::VariableDeclaration(variable_declaration)) =
                            context.nodes.get(id)
                        {
                            if variable_declaration.state_variable
                                && type_string.ends_with("] storage ref")
                            {
                                state_vars_lengths_that_are_referenced.insert(*id);
                            }
                        }
                    }
                }
            }

            state_vars_lengths_that_are_referenced
        }

        /// Investigates the body of the for loop with the help callgraph and accumulates all the
        /// state variables that have been changed
        pub fn state_variable_changes<'a>(
            &self,
            context: &'a WorkspaceContext,
        ) -> Option<ApproximateStorageChangeFinder<'a>> {
            let mut tracker = StateVariableChangeTracker { changes: None, context };

            let callgraph =
                CallGraph::new(context, &[&(self.into())], CallGraphDirection::Inward).ok()?;

            callgraph.accept(context, &mut tracker).ok()?;

            tracker.changes.take()
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
mod cache_array_length_tests {

    use crate::detect::{
        detector::IssueDetector, low::storage_array_length_not_cached::CacheArrayLengthDetector,
    };

    #[test]

    fn test_cache_array_length() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CacheArrayLength.sol",
        );

        let mut detector = CacheArrayLengthDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances

        println!("{:#?}", detector.instances());

        assert_eq!(detector.instances().len(), 3);
        // assert the severity is low
        assert_eq!(detector.severity(), crate::detect::detector::IssueSeverity::Low);
    }
}
