use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    error::Error,
};

use crate::ast::{ASTNode, ContractKind, NodeID};

use crate::{
    capture,
    context::{browser::ExtractVariableDeclarations, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct MissingInheritanceDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for MissingInheritanceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Key -> Contract ID, Value -> Collection of function selectors in the contract
        let mut contract_function_selectors: HashMap<NodeID, Vec<String>> = Default::default();

        // Key -> Contract ID, Value -> Set of contract/interface IDs in it's hierarchy
        let mut inheritance_map: HashMap<NodeID, Vec<NodeID>> = Default::default();

        for contract in context.contract_definitions() {
            let full_contract = &contract.linearized_base_contracts;
            inheritance_map
                .entry(contract.id)
                .or_insert(Vec::from_iter(full_contract.iter().copied()));

            for contract_node_id in full_contract {
                if let Some(ASTNode::ContractDefinition(contract_node)) =
                    context.nodes.get(contract_node_id)
                {
                    let function_selectors: Vec<String> = contract_node
                        .function_definitions()
                        .iter()
                        .flat_map(|f| f.function_selector.clone())
                        .collect();

                    let all_variables = ExtractVariableDeclarations::from(contract_node).extracted;

                    let state_variable_function_selectors: Vec<String> = all_variables
                        .into_iter()
                        .flat_map(|v| v.function_selector.clone())
                        .collect();

                    let mut all_function_selectors = Vec::with_capacity(
                        function_selectors.len() + state_variable_function_selectors.len(),
                    );
                    all_function_selectors.extend(function_selectors);
                    all_function_selectors.extend(state_variable_function_selectors);

                    contract_function_selectors
                        .entry(contract.id)
                        .or_insert(all_function_selectors);
                }
            }
        }

        let mut results: HashMap<NodeID, BTreeSet<String>> = Default::default();

        for (contract_id, contract_selectors) in &contract_function_selectors {
            if contract_selectors.is_empty() {
                continue;
            }
            if let Some(ASTNode::ContractDefinition(c)) = context.nodes.get(contract_id) {
                if c.kind != ContractKind::Contract || c.is_abstract {
                    continue;
                }
            }
            let inheritances = inheritance_map.entry(*contract_id).or_default();
            for (potentially_missing_inheritance, missing_function_selectors) in
                &contract_function_selectors
            {
                // Check that it's not empty
                if missing_function_selectors.is_empty() {
                    continue;
                }

                // Check that it's not the same contract
                if potentially_missing_inheritance == contract_id {
                    continue;
                }

                // Check that it's not already inherited
                if inheritances.contains(potentially_missing_inheritance) {
                    continue;
                }

                if let Some(ASTNode::ContractDefinition(c)) =
                    context.nodes.get(potentially_missing_inheritance)
                {
                    if c.kind == ContractKind::Interface || c.is_abstract {
                        // Check that the contract is compatible with the missing inheritance
                        if missing_function_selectors.iter().all(|s| contract_selectors.contains(s))
                        {
                            results.entry(*contract_id).or_default().insert(c.name.clone());
                        }
                    }
                }
            }
        }

        for (contract, missing_inheritances) in results {
            if let Some(ASTNode::ContractDefinition(c)) = context.nodes.get(&contract) {
                // If the contract c already has some inheritance, don't report it because we want
                // to respect the developer's choice.
                if c.linearized_base_contracts.len() != 1 {
                    continue;
                }
                let missing_inheritances_vector =
                    missing_inheritances.iter().cloned().collect::<Vec<_>>();
                let missing_inheritaces_string = missing_inheritances_vector.join(", ");

                let hint = format!(
                    "Is this contract supposed to implement an interface? Consider extending one of the following: {}",
                    missing_inheritaces_string
                );

                capture!(self, context, c, hint);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn title(&self) -> String {
        String::from("Missing Inheritance")
    }

    fn description(&self) -> String {
        String::from("There is an interface / abstract contract that is potentially missing (not included in) the inheritance of this contract.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::MissingInheritance)
    }
}

#[cfg(test)]
mod missing_inheritance_tests {

    use crate::detect::{
        detector::IssueDetector, low::missing_inheritance::MissingInheritanceDetector,
    };

    #[test]
    fn test_missing_inheritance() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MissingInheritance.sol",
        );

        let mut detector = MissingInheritanceDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        assert_eq!(detector.instances().len(), 2);
    }
}
