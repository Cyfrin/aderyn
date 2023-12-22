use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::Mutability;
use crate::{
    ast::{Expression, VariableDeclaration},
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
            // TODO: handle tuple expression
            _ => None,
        }
    }
}

impl Detector for UninitializedStateVariablesDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        let mut found_state_variables: HashSet<VariableDeclaration> = HashSet::new();
        let mut found_state_variables_names: HashSet<String> = HashSet::new();
        let mut found_assignments: HashSet<String> = HashSet::new();

        for assignment in loader.assignments.keys() {
            if let Some(variable_name) = self.is_state_variable(&assignment.left_hand_side) {
                found_assignments.insert(variable_name);
            }
        }

        println!("found_assignments: {:?}", found_assignments);

        for variable_declaration in loader.variable_declarations.keys() {
            if variable_declaration.state_variable
                && variable_declaration.mutability == Some(Mutability::Mutable)
                && variable_declaration.value.is_none()
                && !found_assignments.contains(&variable_declaration.name)
            {
                found_state_variables.insert(variable_declaration.clone());
                found_state_variables_names.insert(variable_declaration.name.clone());
            }
        }

        println!(
            "found_state_variables_names: {:?}",
            found_state_variables_names
        );

        // insert the instances remaining in found_state_variables into self.found_instances
        found_state_variables
            .into_iter()
            .for_each(|state_variable| {
                // TODO: detector should be modified to insert the instances where the uninitialized state variable is used, since the count will be off
                self.found_instances.insert(
                    loader.get_node_sort_key(&ASTNode::VariableDeclaration(state_variable.clone())),
                    state_variable.src.clone(),
                );
            });

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
        assert_eq!(detector.instances().len(), 2);
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
