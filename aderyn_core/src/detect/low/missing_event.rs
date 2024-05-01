use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{FunctionKind, NodeID},
    capture,
    context::{
        browser::{
            ExtractAssignments, ExtractEmitStatements, ExtractFunctionDefinitions,
            ExtractVariableDeclarations,
        },
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct MissingEventDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for MissingEventDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract_definition in context.contract_definitions() {
            let variable_declarations =
                ExtractVariableDeclarations::from(contract_definition).extracted;
            let function_definitions =
                ExtractFunctionDefinitions::from(contract_definition).extracted;

            for function_definition in function_definitions {
                if function_definition.kind == FunctionKind::Function {
                    let emit_statements =
                        ExtractEmitStatements::from(&function_definition).extracted;
                    let assignments = ExtractAssignments::from(&function_definition).extracted;
                    for assignment in assignments {
                        let referenced_decls = assignment.left_hand_side.referenced_declarations();
                        let found = variable_declarations.iter().any(|var_decl| {
                            var_decl.id == referenced_decls[0] && var_decl.state_variable
                        });
                        if found && emit_statements.is_empty() {
                            capture!(self, context, function_definition);
                        }
                    }
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Functions which modify state should emit an event.")
    }

    fn description(&self) -> String {
        String::from("Functions which modify state should emit an event.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }
    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::MissingEvent)
    }
}

#[cfg(test)]
mod missing_event_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::MissingEventDetector;

    #[test]
    fn test_missing_event() {
        let context =
            load_contract("../tests/contract-playground/out/MissingEvent.sol/MissingEvent.json");

        let mut detector = MissingEventDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of functions modifying state without emitting events
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            "Functions which modify state should emit an event."
        );
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            "Functions which modify state should emit an event."
        );
    }
}
