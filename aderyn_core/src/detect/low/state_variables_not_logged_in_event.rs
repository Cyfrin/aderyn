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

        // All the state variables, set at the beginning of the detect Function

        let mutable_state_variables: HashMap<i64, VariableDeclaration> = context
            .variable_declarations()
            .iter()
            .filter_map(|&var_decl| {
                if !var_decl.constant
                    && matches!(var_decl.mutability(), Some(Mutability::Mutable))
                    && var_decl.state_variable
                {
                    Some((var_decl.id, var_decl.clone())) // clone the VariableDeclaration.
                } else {
                    None
                }
            })
            .collect();

        for function in context
            .function_definitions()
            .into_iter()
            .filter(|function| !function.is_constructor)
        {
            for assignment in ExtractAssignments::from(function).extracted.into_iter() {
                //if we use an operator such as += we do not want to check for event args with RHS
                let check_rhs = assignment.operator == "=";
                let left_hand_side = assignment.left_hand_side.as_ref();
                let right_hand_side = assignment.right_hand_side.as_ref();
                let left_identifier_id = return_identifier_referenced_dec(left_hand_side.clone());
                let right_identifier_name_result = return_identifier_name(right_hand_side.clone());
                let left_identifier_name_result = return_identifier_name(left_hand_side.clone());
                let referenced_id = left_identifier_id?;
                let right_identifier_name = right_identifier_name_result.unwrap();
                let left_identifier_name = left_identifier_name_result.unwrap();

                //assignment is happening on state variable
                if mutable_state_variables.contains_key(&referenced_id) {
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
                                check_rhs,
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
                Ok(reference_id)
            } else {
                Err("Identifier has no ref_dec")
            }
        }
        Expression::MemberAccess(left_struct) => {
            if let Expression::Identifier(left_identifer) = *left_struct.expression {
                if let Some(reference_id) = left_identifer.referenced_declaration {
                    Ok(reference_id)
                } else {
                    Err("MemberAccess Identifier has no ref_dec")
                }
            } else {
                Err("Assignment struct has no identifier")
            }
        }
        _ => Err("Unexpected type in assignment"),
    }
}

fn return_identifier_name(expression: Expression) -> Result<String, &'static str> {
    match expression {
        Expression::Identifier(exp_identifier) => Ok(exp_identifier.name),
        //we only get MemberAccess when the expression is a subfield of a struct so it is safe to just grab that member's name
        Expression::MemberAccess(exp_struct) => Ok(exp_struct.member_name),
        Expression::Literal(exp_literal) => {
            Ok(exp_literal.value.ok_or("Failed to unwrap literal value")?)
        }
        _ => Err("Unexpected type in assignment"),
    }
}

fn check_event_args(
    event: &ASTNode,
    left_var_name: &String,
    right_var_name: &String,
    referenced_id: i64,
    check_rhs: bool,
) -> bool {
    match event {
        ASTNode::Identifier(event_identifier) => {
            event_identifier.name == *left_var_name
                || (check_rhs && event_identifier.name == *right_var_name)
        }
        ASTNode::MemberAccess(event_member_access) => {
            //if we are checking a struct subfield name we must confirm the parent structs are the same
            if let Expression::Identifier(event_identifier) =
                *event_member_access.expression.clone()
            {
                if let Some(struct_ref_id) = event_identifier.referenced_declaration {
                    return struct_ref_id == referenced_id
                        && (event_member_access.member_name == *left_var_name
                            || (check_rhs && event_identifier.name == *right_var_name));
                }
            }
            //event does not have identifier so cannot match struct
            false
        }
        ASTNode::Literal(event_literal) => {
            //if we are checking a literal then they do not have a name so we must check the value instead
            if let Some(event_arg) = &event_literal.value {
                return event_arg == left_var_name || (check_rhs && event_arg == right_var_name);
            }
            false
        }

        //we are only looking for subfields of structs, if the event does not have an expression which is an identifier it must not match
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

        const EXPECTED_NO_OF_FAILURES: usize = 10;
        let mut detector = StateVariableNotLoggedInEventDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        // we expect five valid instance
        assert_eq!(detector.instances().len(), EXPECTED_NO_OF_FAILURES);

        //check the correct assignment lines are triggering detection
        //@dev if StateVariableEvents.sol is modified these line notations will have to be updated

        let expected_lines: [usize; EXPECTED_NO_OF_FAILURES] =
            [25, 29, 33, 43, 47, 52, 63, 74, 75, 89];
        for instance in detector.instances() {
            assert!(expected_lines.contains(&instance.0 .1))
        }
    }
}
