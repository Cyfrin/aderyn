use std::{collections::BTreeMap, error::Error, ops::Add};

use crate::{
    ast::{NodeID, TypeName, VariableDeclaration},
    capture,
    context::{
        browser::ExtractVariableDeclarations,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct InconsistentTypeNamesDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

#[derive(Debug, Default)]
struct TypeNameCounter {
    int_count: Vec<ASTNode>,
    int256_count: Vec<ASTNode>,
    uint_count: Vec<ASTNode>,
    uint256_count: Vec<ASTNode>,
}

impl Add<&TypeNameCounter> for TypeNameCounter {
    type Output = TypeNameCounter;

    fn add(mut self, rhs: &TypeNameCounter) -> Self::Output {
        self.int256_count.append(&mut rhs.int256_count.clone());
        self.int_count.append(&mut rhs.int_count.clone());
        self.uint256_count.append(&mut rhs.uint256_count.clone());
        self.uint_count.append(&mut rhs.uint_count.clone());
        TypeNameCounter {
            int256_count: self.int256_count,
            int_count: self.int_count,
            uint256_count: self.uint256_count,
            uint_count: self.uint_count,
        }
    }
}

impl TypeNameCounter {
    fn is_int_consistent(&self) -> bool {
        self.int256_count.len() * self.int_count.len() == 0
    }

    fn is_uint_consistent(&self) -> bool {
        self.uint256_count.len() * self.uint_count.len() == 0
    }
}

fn count_names_in_type_name(
    variable_declaration: &VariableDeclaration,
    type_name: &TypeName,
) -> TypeNameCounter {
    let mut counter = TypeNameCounter::default();
    match type_name {
        TypeName::ElementaryTypeName(e) => {
            if e.name == "uint" {
                counter.uint_count.push(variable_declaration.into());
            } else if e.name == "uint256" {
                counter.uint256_count.push(variable_declaration.into());
            } else if e.name == "int" {
                counter.int_count.push(variable_declaration.into());
            } else if e.name == "int256" {
                counter.int256_count.push(variable_declaration.into());
            }
        }
        TypeName::FunctionTypeName(_) => (),
        TypeName::ArrayTypeName(e) => {
            let base = &*e.base_type;
            let tc = count_names_in_type_name(variable_declaration, base);
            counter = counter + &tc;
        }
        TypeName::Mapping(e) => {
            let key_type = &*e.key_type;
            let tc_keys = count_names_in_type_name(variable_declaration, key_type);
            counter = counter + &tc_keys;

            let value_type = &*e.value_type;
            let tc_value = count_names_in_type_name(variable_declaration, value_type);
            counter = counter + &tc_value;
        }
        TypeName::UserDefinedTypeName(_) => {}
        TypeName::String(name) => {
            if name == "uint" {
                counter.uint_count.push(variable_declaration.into());
            } else if name == "uint256" {
                counter.uint256_count.push(variable_declaration.into());
            } else if name == "int" {
                counter.int_count.push(variable_declaration.into());
            } else if name == "int256" {
                counter.int256_count.push(variable_declaration.into());
            }
        }
    };
    counter
}

impl IssueDetector for InconsistentTypeNamesDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.contract_definitions() {
            let mut contract_counter = TypeNameCounter::default();
            let extracted_variable_declarations =
                ExtractVariableDeclarations::from(contract).extracted;

            for variable_declaration in extracted_variable_declarations.iter() {
                if let Some(type_name) = &variable_declaration.type_name {
                    // println!("{:?}, {:?}", variable_declaration.name, type_name);
                    let counter = count_names_in_type_name(variable_declaration, type_name);
                    contract_counter = contract_counter + &counter;
                }
            }

            if !contract_counter.is_int_consistent() {
                for node in contract_counter.int_count.iter() {
                    capture!(self, context, node);
                }
            }

            if !contract_counter.is_uint_consistent() {
                for node in contract_counter.uint_count.iter() {
                    capture!(self, context, node);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from(
            "Inconsistency in declaring uint256/uint (or) int256/int variables within a contract. Use explicit size declarations (uint256 or int256).",
        )
    }

    fn description(&self) -> String {
        String::from("Consider keeping the naming convention consistent in a given contract. Explicit size declarations are preferred (uint256, int256) over implicit ones (uint, int) to avoid confusion.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::InconsistentTypeNames)
    }
}

#[cfg(test)]
mod inconsistent_type_names {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::InconsistentTypeNamesDetector;

    #[test]
    #[serial]
    fn test_inconsistent_type_names_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/InconsistentUints.sol",
        );

        let mut detector = InconsistentTypeNamesDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        println!("{:#?}", detector.instances());

        assert_eq!(detector.instances().len(), 7);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }

    #[test]
    #[serial]
    fn test_inconsistent_type_names_with_casting_sol() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/Casting.sol",
        );

        let mut detector = InconsistentTypeNamesDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        println!("{:#?}", detector.instances());

        assert_eq!(detector.instances().len(), 2);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
