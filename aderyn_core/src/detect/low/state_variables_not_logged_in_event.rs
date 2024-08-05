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
use crate::{capture, context::browser::ExtractAssignments};
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
         * Create a list of all state variables (this includes structs but not their subfields)
         * Iterates through all functions
         * Searches each function for assignments
         * For each assignment found check if the left hand side of the assignment is a state variable using prior created list
         * If LHS is a state variable then look at all emitted events in the same function
         * Check if any of the parameters of found events match the state variable
         * If not, raise an issue
         */

        //collect all mutable state variables together
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

        for function in context.function_definitions().into_iter() {
            for assignment in ExtractAssignments::from(function).extracted.into_iter() {
                let left_hand_side = assignment.left_hand_side.as_ref();
                let right_hand_side = assignment.right_hand_side.as_ref();
                let left_identifier_id = return_identifier_referenced_dec(left_hand_side.clone());
                let right_identifier_name_result = return_identifier_name(right_hand_side.clone());
                let left_identifier_name_result = return_identifier_name(left_hand_side.clone());
                let referenced_id = left_identifier_id?;
                let right_identifier_name = right_identifier_name_result?;
                let left_identifier_name = left_identifier_name_result?;

                //assignment is happening on state variable
                if self.mutable_state_variables.contains_key(&referenced_id) {
                    let mut counter: u32 = 0; //counter for number of times state variable is logged in events in same function

                    let body = function
                        .body
                        .clone()
                        .ok_or("Failed to clone function body: body is None")?;
                    //now we must search all events listed in function
                    for event_emission in ExtractEmitStatements::from(&body).extracted.into_iter() {
                        let children = event_emission
                            .event_call
                            .children(context)
                            .ok_or("Failed to unwrap ASTNode children: children is None")?;

                        //if a parameter name matches the LHS or RHS name, increment a counter
                        for child in children.into_iter() {
                            if check_event_args(
                                child,
                                &left_identifier_name,
                                &right_identifier_name,
                                referenced_id,
                            ) {
                                counter += 1;
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

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("State variable modification not logged in event")
    }

    fn description(&self) -> String {
        String::from("State variable assignment not recorded in event logs, this will make it difficult for off-chain applications to track changes to state or searching historic event logs")
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

//expressions can be identifiers for variables and memberAccess for struct subfields so we match accordingly
fn return_identifier_referenced_dec(expression: Expression) -> Result<i64, &'static str> {
    match expression {
        Expression::Identifier(left_identifier) => {
            if let Some(reference_id) = left_identifier.referenced_declaration {
                return Ok(reference_id);
            } else {
                return Err("Identifier has no ref_dec");
            }
        }
        Expression::MemberAccess(left_struct) => {
            if let Expression::Identifier(left_identifer) = *left_struct.expression {
                if let Some(reference_id) = left_identifer.referenced_declaration {
                    return Ok(reference_id);
                } else {
                    return Err("MemberAccess Identifier has no ref_dec");
                }
            } else {
                return Err("Assignment struct has no identifier");
            }
        }
        _ => return Err("Unexpected type in assignment"),
    }
}

fn return_identifier_name(expression: Expression) -> Result<String, &'static str> {
    match expression {
        Expression::Identifier(left_identifier) => Ok(left_identifier.name),
        //we only get MemberAccess when the expression is a subfield of a struct so it is safe to just grab that member's name
        Expression::MemberAccess(left_struct) => return Ok(left_struct.member_name),
        _ => return Err("Unexpected type in assignment"),
    }
}

fn check_event_args(
    event: &ASTNode,
    left_var_name: &String,
    right_var_name: &String,
    referenced_id: i64,
) -> bool {
    match event {
        ASTNode::Identifier(event_identifier) => {
            return (event_identifier.name == *left_var_name
                || event_identifier.name == *right_var_name)
        }
        ASTNode::MemberAccess(event_member_access) => {
            //if we are checking a struct subfield name we must confirm the parent structs are the same
            if let Expression::Identifier(event_identifier) =
                *event_member_access.expression.clone()
            {
                if let Some(struct_ref_id) = event_identifier.referenced_declaration {
                    if struct_ref_id == referenced_id {
                        if event_member_access.member_name == *left_var_name
                            || event_member_access.member_name == *right_var_name
                        {
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        //parent struct does not match event struct, so subfields cannot be the same
                        return false;
                    }
                } else {
                    //identifier does not have a referenced id, this should not occur
                    return false;
                }
            } else {
                //we are only looking for subfields of structs, if the event does not have an expression which is an identifier it must not match
                return false;
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod template_detector_tests {
    use crate::detect::detector::IssueDetector;

    use super::StateVariableNotLoggedInEventDetector;

    #[test]
    fn test_state_variable_not_event_logged() {
        //now lets check if local variable assignment triggers the detector
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariableEvents.sol",
        );

        const EXPECTED_NO_OF_FAILURES: usize = 5;
        let mut detector = StateVariableNotLoggedInEventDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        // we expect five valid instance
        assert_eq!(detector.instances().len(), EXPECTED_NO_OF_FAILURES);

        //check the correct assignment lines are triggering detection
        //@dev if StateVariableEvents.sol is modified these line notations will have to be updated

        let expected_lines: [usize; EXPECTED_NO_OF_FAILURES] = [21, 26, 37, 48, 49];
        for instance in detector.instances() {
            assert!(expected_lines.contains(&instance.0 .1))
        }
    }
}
