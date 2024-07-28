use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    ast::{ASTNode, Expression, Mutability, NodeID, VariableDeclaration},
    context::{
        browser::{ExtractEmitStatements, GetImmediateChildren},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueSeverity},
};
use crate::{
    capture, context::browser::ExtractAssignments, context::browser::ExtractFunctionDefinitions,
};
use eyre::Result;

#[derive(Default)]
pub struct StateVariableNotLoggedInEventDetector {
    // All the state variables, set at the beginning of the detect Function
    mutable_state_variables: HashMap<i64, VariableDeclaration>,
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StateVariableNotLoggedInEventDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        /*
         * How this detector works:
         * Create a list of all state variables
         * Iterate through all contracts
         * In each contract iterate through all functions
         * Search each function for assignments
         * For each assignment found check if the left hand side of the assignment is a state variable using prior created list
         * If LHS is a state variable then look at all emitted events in the same function
         * Check if any of the parameters of found events match the state variable
         * If not, raise an issue
         */

        //collect all state variables together
        self.mutable_state_variables = context
            .variable_declarations()
            .iter()
            .filter_map(|&var_decl| {
                if !var_decl.constant
                    && matches!(var_decl.mutability(), Some(Mutability::Mutable))
                    && var_decl.state_variable
                {
                    Some((var_decl.id, (*var_decl).clone())) // Deref and clone the VariableDeclaration.
                } else {
                    None
                }
            })
            .collect();

        for contract in context.contract_definitions() {
            for function in ExtractFunctionDefinitions::from(contract)
                .extracted
                .into_iter()
            {
                for assignment in ExtractAssignments::from(&function).extracted.into_iter() {
                    let left_hand_side = assignment.left_hand_side.as_ref();
                    let right_hand_side = assignment.right_hand_side.as_ref();

                    if let Expression::Identifier(left_identifier) = left_hand_side {
                        if let Expression::Identifier(right_identifier) = right_hand_side {
                            if let Some(reference_id) = left_identifier.referenced_declaration {
                                if self.mutable_state_variables.contains_key(&reference_id) {
                                    //assignment is happening on state variable
                                    let mut counter: u32 = 0; //counter for number of times state variable is logged in events in same function

                                    //now we must search all events listed in function
                                    for event_emission in
                                        ExtractEmitStatements::from(&function.body.clone().unwrap())
                                            .extracted
                                            .into_iter()
                                    {
                                        //if a parameter matches, increment a counter, then after checking all events we can check if the counter is zero
                                        for child in event_emission
                                            .event_call
                                            .children(context)
                                            .unwrap()
                                            .into_iter()
                                        {
                                            if let ASTNode::Identifier(child_identifier) = child {
                                                if child_identifier.name == left_identifier.name
                                                    || child_identifier.name
                                                        == right_identifier.name
                                                {
                                                    counter += 1;
                                                }
                                            }
                                        }
                                        //if the counter is still zero then no event logs the state var and we capture the assignment line
                                        //reset the counter to zero for each loop of a new state var
                                    }
                                    if counter == 0 {
                                        //no event mentions the state variable, report as an issue
                                        capture!(self, context, assignment);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("State variable modification not logged in event")
    }

    fn description(&self) -> String {
        String::from("State variable assignment not recorded in event logs")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!(
            "{}",
            IssueDetectorNamePool::StateVariableNotLoggedInEventDetector
        )
    }
}

#[cfg(test)]
mod template_detector_tests {
    use crate::detect::detector::IssueDetector;

    use super::StateVariableNotLoggedInEventDetector;

    #[test]
    fn test_state_variable_not_event_logged() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/InconsistentUints.sol",
        );
        let mut detector = StateVariableNotLoggedInEventDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("State variable modification not logged in event")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("State variable assignment not recorded in event logs")
        );

        //now lets check if local variable assignment triggers the detector
        let context2 = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariableEvents.sol",
        );

        let mut detector2 = StateVariableNotLoggedInEventDetector::default();
        let found2 = detector2.detect(&context2).unwrap();
        // assert that the detector found an issue
        assert!(found2);
        // assert that the detector found the correct number of instances
        // one function correctly emits an event and another only alters a local variable
        // so we only expect one valid instance
        assert_eq!(detector2.instances().len(), 1);
    }
}
