use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::{
    ast::{BinaryOperation, Expression, VariableDeclaration},
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DifferentStorageConditionalDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for DifferentStorageConditionalDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // Step 1: Get all state variable declarations
        let state_variables: Vec<&VariableDeclaration> = loader
            .variable_declarations
            .iter()
            .filter(|&var_decl| var_decl.state_variable)
            .collect();

        // Get all state variable IDs
        let state_variable_ids: Vec<i64> = state_variables.iter().map(|var| var.id).collect();

        // Step 2: construct a map of referenced state variable id to binary operations
        let mut binary_operations_by_referenced_state_variable: HashMap<
            i64,
            Vec<&BinaryOperation>,
        > = HashMap::new();

        for binary_operation in loader.binary_operations.iter() {
            if let Expression::Identifier(left_expr) = &*binary_operation.left_expression {
                if state_variable_ids.contains(&left_expr.referenced_declaration) {
                    binary_operations_by_referenced_state_variable
                        .entry(left_expr.referenced_declaration)
                        .or_default()
                        .push(binary_operation);
                }
            }

            if let Expression::Identifier(right_expr) = &*binary_operation.right_expression {
                if state_variable_ids.contains(&right_expr.referenced_declaration) {
                    binary_operations_by_referenced_state_variable
                        .entry(right_expr.referenced_declaration)
                        .or_default()
                        .push(binary_operation);
                }
            }
        }

        for (&var_id, operations) in &binary_operations_by_referenced_state_variable {
            if !operations.is_empty() {
                // Extract the first operation to compare with others
                let first_op = &operations[0];
                let (first_op_side, first_op_operator) = if matches!(&*first_op.left_expression, Expression::Identifier(ident) if ident.referenced_declaration == var_id)
                {
                    ("left", &first_op.operator)
                } else {
                    ("right", &first_op.operator)
                };

                // Check if all operations are consistent with the first one
                let mut first_added = false;
                for op in operations {
                    // Determine the side and operator of the current operation
                    let current_op_side = if matches!(&*op.left_expression, Expression::Identifier(expr) if expr.referenced_declaration == var_id)
                    {
                        "left"
                    } else {
                        "right"
                    };

                    let current_op_operator = &op.operator;

                    // Define valid mirror operators
                    let mirror_operator = match first_op_operator.as_str() {
                        "<" => ">",
                        ">" => "<",
                        _ => first_op_operator, // for other operators, no mirroring logic
                    };

                    // Check if the current operation is consistent or a valid mirror of the first operation
                    let is_consistent_or_mirror = (current_op_side == first_op_side
                        && current_op_operator == first_op_operator)
                        || (current_op_side != first_op_side
                            && current_op_operator == mirror_operator);

                    if !is_consistent_or_mirror {
                        self.found_instances.insert(
                            loader.get_node_sort_key(&ASTNode::BinaryOperation((*op).clone())),
                            op.src.clone(),
                        );
                        if !first_added {
                            self.found_instances.insert(
                                loader.get_node_sort_key(&ASTNode::BinaryOperation(
                                    (*first_op).clone(),
                                )),
                                first_op.src.clone(),
                            );
                            first_added = true;
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Conditional storage checks are not consistent")
    }

    fn description(&self) -> String {
        String::from("When writing `require` or `if` conditionals that check storage values, it is important to be consistent to prevent off-by-one errors. \
        There are instances found where the same storage variable is checked multiple times, but the conditionals are not consistent.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod different_storage_conditionals_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::DifferentStorageConditionalDetector;

    #[test]
    fn test_different_storage_conditionals() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/StorageConditionals.sol/StorageConditionals.json",
        );
        let mut detector = DifferentStorageConditionalDetector::default();
        let found = detector.detect(&context_loader).unwrap();

        // assert found
        assert!(found);
        // assert severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert title
        assert_eq!(
            detector.title(),
            String::from("Conditional storage checks are not consistent")
        );
        // assert description
        assert_eq!(
            detector.description(),
            String::from("When writing `require` or `if` conditionals that check storage values, it is important to be consistent to prevent off-by-one errors. \
        There are instances found where the same storage variable is checked multiple times, but the conditionals are not consistent.")
        );
        // assert instances
        assert_eq!(detector.instances().len(), 3);
    }
}
