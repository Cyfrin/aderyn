use std::{collections::BTreeMap, error::Error, ops::Add};

use crate::{
    ast::{NodeID, TypeName},
    capture,
    context::{browser::ExtractVariableDeclarations, workspace_context::WorkspaceContext},
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
    int_count: usize,
    int256_count: usize,
    uint_count: usize,
    uint256_count: usize,
}

impl TypeNameCounter {
    fn is_consistent(&self) -> bool {
        (self.int256_count * self.int_count == 0) && (self.uint256_count * self.uint_count == 0)
    }
}

impl Add<&TypeNameCounter> for TypeNameCounter {
    type Output = TypeNameCounter;

    fn add(self, rhs: &TypeNameCounter) -> Self::Output {
        TypeNameCounter {
            int256_count: self.int256_count + rhs.int256_count,
            int_count: self.int_count + rhs.int_count,
            uint256_count: self.uint256_count + rhs.uint256_count,
            uint_count: self.uint_count + rhs.uint_count,
        }
    }
}

fn count_names_in_type_name(type_name: &TypeName) -> TypeNameCounter {
    let mut counter = TypeNameCounter::default();
    match type_name {
        TypeName::ElementaryTypeName(e) => {
            if e.name == "uint" {
                counter.uint_count += 1;
            } else if e.name == "uint256" {
                counter.uint256_count += 1;
            } else if e.name == "int" {
                counter.int_count += 1;
            } else if e.name == "int256" {
                counter.int256_count += 1;
            }
        }
        TypeName::FunctionTypeName(_) => (),
        TypeName::ArrayTypeName(e) => {
            let base = &*e.base_type;
            let tc = count_names_in_type_name(base);
            counter = counter + &tc;
        }
        TypeName::Mapping(e) => {
            let key_type = &*e.key_type;
            let tc_keys = count_names_in_type_name(key_type);
            counter = counter + &tc_keys;

            let value_type = &*e.value_type;
            let tc_value = count_names_in_type_name(value_type);
            counter = counter + &tc_value;
        }
        TypeName::UserDefinedTypeName(_) => {}
        TypeName::String(name) => {
            if name == "uint" {
                counter.uint_count += 1;
            } else if name == "uint256" {
                counter.uint256_count += 1;
            } else if name == "int" {
                counter.int_count += 1;
            } else if name == "int256" {
                counter.int256_count += 1;
            }
        }
    };
    counter
}

impl IssueDetector for InconsistentTypeNamesDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.contract_definitions() {
            let mut contract_counter = TypeNameCounter::default();

            for variable_declaration in ExtractVariableDeclarations::from(contract)
                .extracted
                .into_iter()
            {
                if let Some(type_name) = &variable_declaration.type_name {
                    // println!("{:?}, {:?}", variable_declaration.name, type_name);
                    let counter = count_names_in_type_name(type_name);
                    contract_counter = contract_counter + &counter;
                }
            }

            if !contract_counter.is_consistent() {
                // println!("{:?}", contract_counter); <-- Will print the count of each types
                capture!(self, context, contract);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from(
            "Inconsistency in declaring uint256/uint (or) int256/int variables within a contract",
        )
    }

    fn description(&self) -> String {
        String::from("Consider keeping the naming convention consistent in a given contract")
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
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::InconsistentTypeNamesDetector;

    #[test]
    fn test_inconsistent_type_names() {
        let context =
            load_contract("../tests/contract-playground/out/InconsistentUints.sol/InconsistentStateVariablesContract.json");

        let mut detector = InconsistentTypeNamesDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        println!("{:?}", detector.instances());

        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
