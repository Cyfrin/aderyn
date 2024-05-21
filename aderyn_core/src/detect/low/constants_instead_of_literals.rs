use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::{
    ast::{Literal, LiteralKind, NodeID},
    capture,
    context::{
        browser::{
            ExtractFunctionDefinitions, ExtractLiterals, ExtractModifierDefinitions,
            GetImmediateParent,
        },
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ConstantsInsteadOfLiteralsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ConstantsInsteadOfLiteralsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Get all contracts
        // For each contract
        //      Get all Function definitions (and to the same for modifiers)
        //          Get all literals
        //          For each literal
        //              if literal.value is not 0 or 1
        //                  if the literal.value appears more than once, then capture all instances

        for contract in context.contract_definitions() {
            let mut literal_values_found: HashMap<String, Vec<Literal>> = HashMap::new();

            for function in ExtractFunctionDefinitions::from(contract)
                .extracted
                .into_iter()
            {
                for literal in ExtractLiterals::from(&function).extracted.into_iter() {
                    if (literal.kind == LiteralKind::Number
                        && literal.value != Some(String::from("0"))
                        && literal.value != Some(String::from("1")))
                        && literal.value != Some(String::from("2"))
                        || literal.kind == LiteralKind::HexString
                        || literal.kind == LiteralKind::Address
                    {
                        // If the literal is used as an index access in a variable, don't capture it
                        if let Some(ASTNode::IndexAccess(_)) = literal.parent(context) {
                            continue;
                        }

                        if let Some(literal_value) = literal.value.as_ref() {
                            if literal_values_found.contains_key(literal_value) {
                                literal_values_found
                                    .get_mut(literal_value)
                                    .unwrap()
                                    .push(literal);
                            } else {
                                literal_values_found.insert(literal_value.clone(), vec![literal]);
                            }
                        }
                    }
                }
            }

            for modifier in ExtractModifierDefinitions::from(contract)
                .extracted
                .into_iter()
            {
                for literal in ExtractLiterals::from(&modifier).extracted.into_iter() {
                    if (literal.kind == LiteralKind::Number
                        && literal.value != Some(String::from("0"))
                        && literal.value != Some(String::from("1")))
                        && literal.value != Some(String::from("2"))
                        || literal.kind == LiteralKind::HexString
                        || literal.kind == LiteralKind::Address
                    {
                        // If the literal is used as an index access in a variable, don't capture it
                        if let Some(ASTNode::IndexAccess(_)) = context.get_parent(literal.id) {
                            continue;
                        }

                        if let Some(literal_value) = literal.value.as_ref() {
                            if literal_values_found.contains_key(literal_value) {
                                literal_values_found
                                    .get_mut(literal_value)
                                    .unwrap()
                                    .push(literal);
                            } else {
                                literal_values_found.insert(literal_value.clone(), vec![literal]);
                            }
                        }
                    }
                }
            }

            for (_, literals) in literal_values_found.iter() {
                if literals.len() > 1 {
                    for literal in literals {
                        capture!(self, context, literal);
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Define and use `constant` variables instead of using literals")
    }

    fn description(&self) -> String {
        String::from("If the same constant literal value is used multiple times, create a constant state variable and reference it throughout the contract.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ConstantsInsteadOfLiterals)
    }
}

#[cfg(test)]
mod constants_instead_of_literals_tests {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::ConstantsInsteadOfLiteralsDetector;

    #[test]
    #[serial]
    fn test_constants_instead_of_literals_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ConstantsLiterals.sol",
        );

        let mut detector = ConstantsInsteadOfLiteralsDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of instances
        assert_eq!(detector.instances().len(), 8);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Define and use `constant` variables instead of using literals")
        );
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            String::from(
                "If the same constant literal value is used multiple times, create a constant state variable and reference it throughout the contract."
            )
        );
    }
}
