use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::{
    ast::{Expression, FunctionDefinition, Statement},
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UninitializedStateVariablesDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl UninitializedStateVariablesDetector {
    fn is_state_variable(&mut self, expression: &Expression) -> Option<String> {
        // Implement logic to check if the expression is a state variable
        // For instance, if `expression` is an Identifier or MemberAccess,
        // determine if it refers to a state variable.
        // Return Some(variable_name) if it's a state variable, None otherwise.
        match expression {
            Expression::Identifier(identifier) => Some(identifier.name.clone()),
            Expression::MemberAccess(member_access) => {
                if member_access.referenced_declaration.is_some() {
                    Some(member_access.member_name.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn function_uses_variable_without_initialization(
        &mut self,
        function_node: &FunctionDefinition,
        variable_name: &str,
    ) -> bool {
        // Implement logic to check if the function uses the given variable name without it being initialized
        // This would typically involve parsing the function body and looking for usage of the variable
        // Return true if the variable is used without being initialized, false otherwise
        let function_body = function_node.body.as_ref();
        for statement in &function_body.unwrap().statements {
            match statement {
                Statement::ExpressionStatement(expression_statement) => {
                    let expression = expression_statement.expression.to_owned();
                    match expression {
                        Expression::MemberAccess(member_access) => {
                            if member_access.member_name == variable_name {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // If no uninitialized usage is found, return false.
        false
    }
}

impl Detector for UninitializedStateVariablesDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        let mut initialized_variables = HashSet::new();

        // Track assignments to state variables
        for (assignment, _) in &loader.assignments {
            if let Some(variable_name) = self.is_state_variable(&assignment.left_hand_side) {
                initialized_variables.insert(variable_name);
            }
        }

        // for state variable declaration statements in loader state variable declaration statements keys
        for variable_declaration in loader.variable_declarations.keys() {
            if variable_declaration.state_variable
                && !initialized_variables.contains(&variable_declaration.name)
            {
                // Iterate over all function nodes and check if this variable is used without initialization
                for function_definition in loader.function_definitions.keys() {
                    if self.function_uses_variable_without_initialization(
                        function_definition,
                        &variable_declaration.name,
                    ) {
                        // Insert the state variable declaration into the found instances
                        self.found_instances.insert(
                            loader.get_node_sort_key(&ASTNode::VariableDeclaration(
                                variable_declaration.clone(),
                            )),
                            variable_declaration.src.clone(),
                        );
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Uninitialized state variables")
    }

    fn description(&self) -> String {
        String::from("Use of uninitialized state variables can lead to unexpected behavior and result in permanent loss of funds.")
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod uninitialized_state_variables_detector_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::UninitializedStateVariablesDetector;

    #[test]
    fn test_uninitialized_state_variables_detector() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/StateVariables.sol/StateVariables.json",
        );
        let mut detector = UninitializedStateVariablesDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an uninitialized state variable that is being used
        assert!(found);
        // assert that the detector found the correct number of instances (1)
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Uninitialized state variables")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Use of uninitialized state variables can lead to unexpected behavior and result in permanent loss of funds."
            )
        );
    }
}
