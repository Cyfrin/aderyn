use std::collections::{BTreeMap, HashMap};
use std::error::Error;

use crate::ast::{
    ContractDefinition, NodeID, NodeType, UserDefinedTypeNameOrIdentifierPath, VariableDeclaration,
};

use crate::capture;
use crate::context::browser::ExtractVariableDeclarations;
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

fn are_duplicate_names_in_inherited_contracts(
    context: &WorkspaceContext,
    variable_name: &str, // Use &str directly for comparison efficiency
    contract_definition: &ContractDefinition, // Use reference to avoid cloning
) -> bool {
    // Check for duplicate variable names in the current contract
    if ExtractVariableDeclarations::from(contract_definition)
        .extracted
        .iter()
        .any(|vd| vd.state_variable && !vd.constant && vd.name == variable_name)
    {
        return true; // Return immediately if a duplicate is found
    }

    // Recursively check base contracts
    for base_contract in &contract_definition.base_contracts {
        if let UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(base_name) =
            &base_contract.base_name
        {
            if let Some(ASTNode::ContractDefinition(contract)) =
                context.nodes.get(&base_name.referenced_declaration)
            {
                if are_duplicate_names_in_inherited_contracts(context, variable_name, contract) {
                    return true; // Return immediately if a duplicate is found
                }
            }
        }
    }

    false // Return false if no duplicates found
}

impl IssueDetector for StateVariableShadowingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut temp_map: HashMap<String, Vec<&VariableDeclaration>> = context
            .variable_declarations()
            .into_iter()
            .filter(|vd| vd.state_variable && !vd.constant)
            .fold(HashMap::new(), |mut acc, var| {
                acc.entry(var.name.clone()).or_default().push(var);
                acc
            });

        // Filter the map to only include entries with more than one variable
        temp_map.retain(|_, v| v.len() > 1);

        for (_, variables) in temp_map {
            for variable in variables {
                // Recurse up the inheritance tree
                let contract_ast =
                    context.get_closest_ancestor(variable.id, NodeType::ContractDefinition);
                if let Some(ASTNode::ContractDefinition(contract)) = contract_ast {
                    for base_contract in &contract.base_contracts {
                        if let UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(base_name) =
                            &base_contract.base_name
                        {
                            if let Some(ASTNode::ContractDefinition(contract)) =
                                context.nodes.get(&base_name.referenced_declaration)
                            {
                                if are_duplicate_names_in_inherited_contracts(
                                    context,
                                    &variable.name,
                                    contract,
                                ) {
                                    capture!(self, context, variable);
                                }
                            }
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
        IssueDetectorNamePool::StateVariableShadowing.to_string()
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
        assert_eq!(detector.instances().len(), 1);
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
