use std::{collections::HashMap, error::Error};

use crate::{
    ast::{Assignment, BinaryOperation, Expression, VariableDeclaration},
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
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

    // List of all the assignments without zero address checks found
    found_no_zero_address_check: Vec<Option<ASTNode>>,
}

impl ZeroAddressCheckDetector {
    fn reset_transient_variables(&mut self) {
        self.binary_checks_against_zero_address = HashMap::new();
        self.assignments_to_mutable_address_state_variables = HashMap::new();
    }
}

impl ASTConstVisitor for ZeroAddressCheckDetector {
    // if the left hand side referenced_declaration is in the mutable_address_state_variables return true
    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
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

    // If the binary
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
        self.mutable_address_state_variables = loader.get_address_state_variables_by_id().clone();

        // Get all function definitions
        for function_definition in loader.get_function_definitions() {
            // Reset transient variables
            self.reset_transient_variables();
            // Visit the function definition using the BinaryOperator and Assignment visitors
            function_definition.accept(self)?;
            // if there are assignments to mutable address state variables that are not present
            // in the binary_checks_against_zero_address, add the assignment to the found_no_zero_address_check
            for (key, value) in &self.assignments_to_mutable_address_state_variables {
                if !self.binary_checks_against_zero_address.contains_key(key) {
                    self.found_no_zero_address_check
                        .push(Some(ASTNode::Assignment(value.clone())));
                }
            }
        }

        Ok(!self.found_no_zero_address_check.is_empty())
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

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_no_zero_address_check.clone()
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
        let context_loader =
            load_contract("./tests/contract-playground/out/StateVariables.sol/StateVariables.json");
        let mut detector = ZeroAddressCheckDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found the issue
        assert!(found);
        // assert that the detector found the correct number of issues
        assert_eq!(detector.found_no_zero_address_check.len(), 1);
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
