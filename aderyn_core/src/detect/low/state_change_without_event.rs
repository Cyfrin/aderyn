use std::{collections::BTreeMap, error::Error};

use crate::ast::{FunctionKind, NodeID};

use crate::{
    capture,
    context::{
        browser::ExtractEmitStatements,
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct StateVariableChangesWithoutEventDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StateVariableChangesWithoutEventDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for func in helpers::get_implemented_external_and_public_functions(context) {
            if *func.kind() == FunctionKind::Constructor {
                continue;
            }
            let callgraphs =
                CallGraphConsumer::get(context, &[&(func.into())], CallGraphDirection::Inward)?;

            for callgraph in callgraphs {
                let mut tracker = EventEmissionTracker { does_emit_events: false };
                callgraph.accept(context, &mut tracker)?;

                if tracker.does_emit_events {
                    continue;
                }

                // At this point, we know that no events are emitted
                if let Some(changes) = func.state_variable_changes(context) {
                    if changes.state_variables_have_been_manipulated() {
                        capture!(self, context, func);
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State Change Without Event")
    }

    fn description(&self) -> String {
        String::from("There are state variable changes in this function but no event is emitted. Consider emitting an event to enable offchain indexers to track the changes.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::StateChangeWithoutEvent)
    }
}

struct EventEmissionTracker {
    does_emit_events: bool,
}

impl CallGraphVisitor for EventEmissionTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        // Don't bother checking if we already know there is an event emitted
        if self.does_emit_events {
            return Ok(());
        }

        // Check for events
        let events = ExtractEmitStatements::from(node).extracted;
        if !events.is_empty() {
            self.does_emit_events = true;
        }
        Ok(())
    }
}

#[cfg(test)]
mod state_variable_changes_without_events_tests {

    use crate::detect::{
        detector::IssueDetector,
        low::state_change_without_event::StateVariableChangesWithoutEventDetector,
    };

    #[test]

    fn test_state_variable_changes_without_events() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariablesChangesWithoutEvents.sol",
        );

        let mut detector = StateVariableChangesWithoutEventDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 8);
    }
}
