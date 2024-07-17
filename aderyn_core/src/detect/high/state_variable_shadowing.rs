use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;

use crate::ast::{
    ContractDefinition, NodeID, NodeType, TypeName, UserDefinedTypeName,
    UserDefinedTypeNameOrIdentifierPath, VariableDeclaration,
};

use crate::capture;
use crate::context::workspace_context::ASTNode;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

// This detector catches an issue that is only present on contracts that can be compiled
// with Solidity version < 0.6.0. In Solidity 0.6.0, the `override` keyword was introduced
// to override state variables from contracts that are inherited and extended.

// Preprocessing that would make this detector more efficient:
// 1. Inheritance/Extension Tree
// 2. Solc version based detector assignment (if solc version < 0.6.0, run this detector on the workspace context)

#[derive(Default)]
pub struct StateVariableShadowingDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

fn gather_contract_definitions(
    context: &WorkspaceContext,
    mut results: HashSet<ContractDefinition>,
    contract_definition: ContractDefinition,
) -> HashSet<ContractDefinition> {
    results.insert(contract_definition.clone());

    for base_contract in contract_definition.base_contracts {
        let base_name = base_contract.base_name;
        if let UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(base_name) = base_name {
            let contract_ast = context.nodes.get(&base_name.referenced_declaration);
            if let Some(ASTNode::ContractDefinition(contract)) = contract_ast {
                results = gather_contract_definitions(context, results, contract.clone());
            }
        }
    }

    return results;
}

impl IssueDetector for StateVariableShadowingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // capture!(self, context, item);

        // TODO: add filter for solc version < 0.6.0
        let mut temp_map: HashMap<String, Vec<&VariableDeclaration>> = context
            .variable_declarations()
            .into_iter()
            .filter(|vd| vd.state_variable && !vd.constant)
            .fold(HashMap::new(), |mut acc, var| {
                acc.entry(var.name.clone())
                    .or_insert_with(Vec::new)
                    .push(var);
                acc
            });

        // Filter the map to only include entries with more than one variable
        temp_map = temp_map.into_iter().filter(|(_, v)| v.len() > 1).collect();

        for (_, vars) in temp_map {
            let mut contract_definitions: HashSet<ContractDefinition> = HashSet::new();
            for var in vars.clone() {
                let contract = context.get_closest_ancestor(var.id, NodeType::ContractDefinition);
                if let Some(contract) = contract {
                    if let ASTNode::ContractDefinition(contract) = contract {
                        contract_definitions = gather_contract_definitions(
                            context,
                            contract_definitions,
                            contract.clone(),
                        );
                    }
                }
            }

            for var in vars {
                let contract = context.get_closest_ancestor(var.id, NodeType::ContractDefinition);
                if let Some(contract) = contract {
                    if let ASTNode::ContractDefinition(contract) = contract {
                        if contract_definitions.contains(contract) {
                            capture!(self, context, var);
                        }
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
        String::from("High Issue Title")
    }

    fn description(&self) -> String {
        String::from("Description of the high issue.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("high-issue-template")
    }
}

#[cfg(test)]
mod state_variable_shadowing_detector_tests {
    use crate::detect::{detector::IssueDetector, high::StateVariableShadowingDetector};

    #[test]
    fn test_state_variable_shadowing_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateShadowing.sol",
        );

        let mut detector = StateVariableShadowingDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from("High Issue Title"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Description of the high issue.")
        );
    }
}
