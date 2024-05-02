use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
};

use crate::{
    ast::{Assignment, BinaryOperation, Expression, Mutability, NodeID, VariableDeclaration},
    capture,
    context::{
        browser::{ExtractAssignments, ExtractBinaryOperations, ExtractIdentifiers},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ZeroAddressCheckDetector {
    // All the state variables, set at the beginning of the detect Function
    mutable_address_state_variables: HashMap<i64, VariableDeclaration>,

    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ZeroAddressCheckDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Get all address state variables
        self.mutable_address_state_variables = context
            .variable_declarations()
            .iter()
            .filter_map(|&var_decl| {
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

        // Get all Function definitions
        for function_definition in context.function_definitions() {
            // Get all the binary checks inside the Function
            let binary_operations: Vec<BinaryOperation> =
                ExtractBinaryOperations::from(function_definition)
                    .extracted
                    .into_iter()
                    .filter(|x| (x.operator == "==" || x.operator == "!="))
                    .collect();

            // Filter the binary checks and extract all node ids into a vector
            let mut identifier_reference_declaration_ids_in_binary_checks = HashSet::new();

            for x in binary_operations {
                let l = x.left_expression.as_ref();
                if let Expression::Identifier(left_identifier) = l {
                    identifier_reference_declaration_ids_in_binary_checks
                        .insert(left_identifier.referenced_declaration);
                } else {
                    ExtractIdentifiers::from(l)
                        .extracted
                        .into_iter()
                        .map(|f| f.referenced_declaration)
                        .for_each(|f| {
                            identifier_reference_declaration_ids_in_binary_checks.insert(f);
                        });
                }

                let r = x.right_expression.as_ref();
                if let Expression::Identifier(right_identifier) = r {
                    identifier_reference_declaration_ids_in_binary_checks
                        .insert(right_identifier.referenced_declaration);
                } else {
                    ExtractIdentifiers::from(r)
                        .extracted
                        .into_iter()
                        .map(|f| f.referenced_declaration)
                        .for_each(|f| {
                            identifier_reference_declaration_ids_in_binary_checks.insert(f);
                        });
                }
            }

            // Get all the assignments where the left hand side is a mutable address state variable
            let assignments: Vec<Assignment> = ExtractAssignments::from(function_definition)
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
                        false
                    } else {
                        let left_identifiers = ExtractIdentifiers::from(left_hand_side);
                        for identifier in left_identifiers.extracted {
                            if self
                                .mutable_address_state_variables
                                .contains_key(&identifier.referenced_declaration)
                            {
                                return true;
                            }
                        }
                        false
                    }
                })
                .collect();

            // For each assignment, if the right hand side is in the identifier_reference_declaration_ids_in_binary_checks
            // and is also in the Function.parameters, then add the assignment to the found_instances
            for assignment in assignments {
                if let Expression::Identifier(right_identifier) = &*assignment.right_hand_side {
                    if !identifier_reference_declaration_ids_in_binary_checks
                        .contains(&right_identifier.referenced_declaration)
                        && function_definition
                            .parameters
                            .parameters
                            .iter()
                            .any(|x| x.id == right_identifier.referenced_declaration)
                    {
                        capture!(self, context, assignment);
                    }
                } else {
                    let right_identifiers = ExtractIdentifiers::from(&*assignment.right_hand_side);
                    for right_identifier in right_identifiers.extracted {
                        if !identifier_reference_declaration_ids_in_binary_checks
                            .contains(&right_identifier.referenced_declaration)
                            && function_definition
                                .parameters
                                .parameters
                                .iter()
                                .any(|x| x.id == right_identifier.referenced_declaration)
                        {
                            capture!(self, context, assignment);
                        }
                    }
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
        String::from("Check for `address(0)` when assigning values to address state variables.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ZeroAddressCheck)
    }
}

#[cfg(test)]
mod zero_address_check_tests {
    use crate::{
        ast::NodeType,
        context::{browser::GetClosestAncestorOfTypeX, workspace_context::ASTNode},
        detect::{detector::IssueDetector, low::ZeroAddressCheckDetector},
    };
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_deprecated_oz_functions_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ZeroAddressCheck.sol",
        );

        let mut detector = ZeroAddressCheckDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found the issue
        assert!(found);
        // assert that the detector found the correct number of issues
        assert_eq!(detector.instances().len(), 3);
        for node_id in detector.instances().values() {
            if let ASTNode::Assignment(assignment) = context.nodes.get(node_id).unwrap() {
                if let ASTNode::FunctionDefinition(function) = assignment
                    .closest_ancestor_of_type(&context, NodeType::FunctionDefinition)
                    .unwrap()
                {
                    assert!(function.name.contains("bad"));
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        }
        // assert that the severity is Low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
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
                "Check for `address(0)` when assigning values to address state variables."
            )
        );
    }
}
