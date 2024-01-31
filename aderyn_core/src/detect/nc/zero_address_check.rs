use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
};

use crate::{
    ast::{BinaryOperation, Expression, Mutability, NodeID, VariableDeclaration},
    capture,
    context::{
        browser::{ExtractAssignments, ExtractBinaryOperations},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{Detector, DetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ZeroAddressCheckDetector {
    // All the state variables, set at the beginning of the detect function
    mutable_address_state_variables: HashMap<i64, VariableDeclaration>,

    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl Detector for ZeroAddressCheckDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Get all address state variables
        self.mutable_address_state_variables = context
            .variable_declarations
            .keys()
            .filter_map(|var_decl| {
                if !var_decl.constant
                    && matches!(var_decl.mutability, Some(Mutability::Mutable))
                    && var_decl.state_variable
                    && (var_decl
                        .type_descriptions
                        .type_string
                        .as_deref()
                        .unwrap_or("")
                        .contains("address")
                        || var_decl
                            .type_descriptions
                            .type_string
                            .as_deref()
                            .unwrap_or("")
                            .contains("contract"))
                {
                    Some((var_decl.id, (*var_decl).clone())) // Deref and clone the VariableDeclaration.
                } else {
                    None
                }
            })
            .collect();

        // Get all function definitions
        for function_definition in context.function_definitions.keys() {
            // Get all the binary checks inside the function
            let binary_operations: Vec<BinaryOperation> =
                ExtractBinaryOperations::from(function_definition)
                    .extracted
                    .into_iter()
                    .filter(|x| x.operator == "==" || x.operator == "!=")
                    .collect();

            // Filter the binary checks and extract all node ids into a vector
            let mut binary_checks_against_zero_address = HashSet::new();

            for x in binary_operations {
                let l_node_id: Option<NodeID> = {
                    let l = x.left_expression.as_ref();
                    if let Expression::Identifier(left_identifier) = l {
                        Some(left_identifier.referenced_declaration)
                    } else {
                        None
                    }
                };
                if let Some(l_node_id) = l_node_id {
                    binary_checks_against_zero_address.insert(l_node_id);
                }

                let r_node_id: Option<NodeID> = {
                    let r = x.right_expression.as_ref();
                    if let Expression::Identifier(right_identifier) = r {
                        Some(right_identifier.referenced_declaration)
                    } else {
                        None
                    }
                };
                if let Some(r_node_ids) = r_node_id {
                    binary_checks_against_zero_address.insert(r_node_ids);
                }
            }

            // Get all the assignments in the function
            let assignments = ExtractAssignments::from(function_definition)
                .extracted
                .into_iter()
                .filter(|x| {
                    let left_hand_side = x.left_hand_side.as_ref();
                    if let Expression::Identifier(left_identifier) = left_hand_side {
                        if self
                            .mutable_address_state_variables
                            .contains_key(&left_identifier.referenced_declaration)
                        {
                            return true;
                        }
                    }
                    false
                })
                .filter_map(|x| {
                    let right_hand_side = x.right_hand_side.as_ref();
                    if let Expression::Identifier(right_identifier) = right_hand_side {
                        return Some((right_identifier.referenced_declaration, x.clone()));
                    }
                    None
                });

            // HashMap where the key is the referenced_declaration of the right hand side of an assignment
            // where the left hand side is a mutable address state variable

            let mut assignments_to_mutable_address_state_variables = HashMap::new();

            for tuple in assignments {
                assignments_to_mutable_address_state_variables.insert(tuple.0, tuple.1.clone());
            }

            // if there are assignments to mutable address state variables that are not present
            // in the binary_checks_against_zero_address, add the assignment to the found_no_zero_address_check
            for (key, value) in &assignments_to_mutable_address_state_variables {
                if !binary_checks_against_zero_address.contains(key) {
                    capture!(self, context, value);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from(
            "Missing checks for `address(0)` when assigning values to address state variables",
        )
    }

    fn description(&self) -> String {
        String::from(
            "Assigning values to address state variables without checking for `address(0)`.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", DetectorNamePool::ZeroAddressCheck)
    }
}

#[cfg(test)]
mod zero_address_check_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, Detector},
        nc::zero_address_check::ZeroAddressCheckDetector,
    };

    #[test]
    fn test_deprecated_oz_functions_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/StateVariables.sol/StateVariables.json",
        );

        let mut detector = ZeroAddressCheckDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found the issue
        assert!(found);
        // assert that the detector found the correct number of issues
        assert_eq!(detector.instances().len(), 1);
        // assert that the severity is NC
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from(
                "Missing checks for `address(0)` when assigning values to address state variables"
            )
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Assigning values to address state variables without checking for `address(0)`."
            )
        );
    }
}
