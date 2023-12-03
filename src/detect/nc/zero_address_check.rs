use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::{
    ast::*,
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

// TODO, this is actually a crazy compilcated detector for a fairly simple issue.
// During the first context load of the contracts, the context should enable easy
// retrieval of certain types inside of another type. For example, get all the Assignments
// inside of a FunctionDefinition. This would make this detector much simpler.

#[derive(Default)]
pub struct ZeroAddressCheckDetector {
    // All the state variables, set at the beginning of the detect function
    mutable_address_state_variables: HashMap<i64, VariableDeclaration>,

    // TRANSIENT VARIABLES
    // HashMap where the key is the referenced_declaration in a binary operation that
    // is checked against a zero address. The key will be a VariableDeclaration key
    binary_checks_against_zero_address: HashMap<i64, bool>,
    // HashMap where the key is the referenced_declaration of the right hand side of an assignment
    // where the left hand side is a mutable address state variable
    assignments_to_mutable_address_state_variables: HashMap<i64, Assignment>,

    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl ZeroAddressCheckDetector {
    fn reset_transient_variables(&mut self) {
        self.binary_checks_against_zero_address = HashMap::new();
        self.assignments_to_mutable_address_state_variables = HashMap::new();
    }
}

impl AstBaseVisitor for ZeroAddressCheckDetector {
    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        // if the left hand side referenced_declaration is in the mutable_address_state_variables return true
        let left_hand_side = node.left_hand_side.as_ref();
        if let Expression::Identifier(left_identifier) = left_hand_side {
            if self
                .mutable_address_state_variables
                .contains_key(&left_identifier.referenced_declaration)
            {
                // add the right hand side referenced_declaration to the assignments_to_mutable_address_state_variables
                let right_hand_side = node.right_hand_side.as_ref();
                if let Expression::Identifier(right_identifier) = right_hand_side {
                    self.assignments_to_mutable_address_state_variables
                        .insert(right_identifier.referenced_declaration, node.clone());
                }
            }
        };
        Ok(true)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        // if the binaryoperation operator is "==" or "!="
        if node.operator == "==" || node.operator == "!=" {
            // if the left hand side is an identifier add its referenced_declaration to the binary_checks_against_zero_address
            // OR
            // if the right hand side is an identifier add its referenced_declaration to the binary_checks_against_zero_address
            let left_expression = node.left_expression.as_ref();
            if let Expression::Identifier(left_identifier) = left_expression {
                self.binary_checks_against_zero_address
                    .insert(left_identifier.referenced_declaration, true);
            };

            let right_expression = node.right_expression.as_ref();
            if let Expression::Identifier(right_identifier) = right_expression {
                self.binary_checks_against_zero_address
                    .insert(right_identifier.referenced_declaration, true);
            }
        }
        Ok(true)
    }
}

impl Detector for ZeroAddressCheckDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // Get all address state variables
        self.mutable_address_state_variables = loader
            .variable_declarations
            .iter() // We can consume the Vec since it's just references.
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
        for function_definition in loader.function_definitions.iter() {
            // Reset transient variables
            self.reset_transient_variables();
            // Visit the function definition using the BinaryOperator and Assignment visitors
            function_definition.accept(self)?;
            // if there are assignments to mutable address state variables that are not present
            // in the binary_checks_against_zero_address, add the assignment to the found_no_zero_address_check
            for (key, value) in &self.assignments_to_mutable_address_state_variables {
                if !self.binary_checks_against_zero_address.contains_key(key) {
                    self.found_instances.insert(
                        loader.get_node_sort_key(&ASTNode::Assignment(value.clone())),
                        value.src.clone(),
                    );
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

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod zero_address_check_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract_from_json, Detector},
        nc::zero_address_check::ZeroAddressCheckDetector,
    };

    #[test]
    fn test_deprecated_oz_functions_detector() {
        let context_loader = load_contract_from_json(
            "./tests/contract-playground/out/StateVariables.sol/StateVariables.json",
        );
        let mut detector = ZeroAddressCheckDetector::default();
        let found = detector.detect(&context_loader).unwrap();
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
