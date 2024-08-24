use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;

use crate::ast::{ASTNode, ContractKind, NodeID};

use crate::capture;
use crate::context::browser::ExtractVariableDeclarations;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct MissingInheritanceDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for MissingInheritanceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Key -> Interface ID, Value -> Collection of function selectors in the inheritance
        let mut interface_function_selectors: HashMap<NodeID, Vec<String>> = Default::default();

        for interface in context
            .contract_definitions()
            .into_iter()
            .filter(|&c| c.kind == ContractKind::Interface)
        {
            if let Some(full_interface) = &interface.linearized_base_contracts {
                for interface_node_id in full_interface {
                    if let Some(ASTNode::ContractDefinition(interface_node)) =
                        context.nodes.get(interface_node_id)
                    {
                        let selectors: Vec<String> = interface_node
                            .function_definitions()
                            .iter()
                            .flat_map(|f| f.function_selector.clone())
                            .collect();
                        interface_function_selectors
                            .entry(interface.id)
                            .or_insert(selectors);
                    }
                }
            }
        }

        // Key -> Contract ID, Value -> Collection of function selectors in the contract
        let mut contract_function_selectors: HashMap<NodeID, Vec<String>> = Default::default();

        // Key -> Contract ID, Value -> Set of contract/interface IDs in it's heirarchy
        let mut inheritance_map: HashMap<NodeID, BTreeSet<NodeID>> = Default::default();

        for contract in context
            .contract_definitions()
            .into_iter()
            .filter(|&c| c.kind == ContractKind::Contract)
        {
            if let Some(full_contract) = &contract.linearized_base_contracts {
                inheritance_map
                    .entry(contract.id)
                    .or_insert(BTreeSet::from_iter(full_contract.iter().copied()));

                for contract_node_id in full_contract {
                    if let Some(ASTNode::ContractDefinition(contract_node)) =
                        context.nodes.get(contract_node_id)
                    {
                        let function_selectors: Vec<String> = contract_node
                            .function_definitions()
                            .iter()
                            .flat_map(|f| f.function_selector.clone())
                            .collect();

                        let all_variables =
                            ExtractVariableDeclarations::from(contract_node).extracted;

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
        }

        for (contract_id, contract_function_selectors) in contract_function_selectors {
            if contract_function_selectors.is_empty() {
                continue;
            }
            let inheritances = inheritance_map.entry(contract_id).or_default();
            for (_potentially_missing_interface, interface_function_selectors) in
                interface_function_selectors
                    .iter()
                    .filter(|(&k, _)| !inheritances.contains(&k))
            {
                if interface_function_selectors.is_empty() {
                    continue;
                }
                if interface_function_selectors
                    .iter()
                    .all(|s| contract_function_selectors.contains(s))
                {
                    // Now we know that `_potentially_missing_interface` is missing inheritance for `contract_id`
                    if let Some(contract) = context.nodes.get(&contract_id) {
                        capture!(self, context, contract);
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Potentially missing inheritance for contract.")
    }

    fn description(&self) -> String {
        String::from("There is an interface that is potentially missing (not included in) the inheritance of this contract. If that's not the case, consider using the same interface instead of defining multiple identical interfaces.")
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
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, low::missing_inheritance::MissingInheritanceDetector,
    };

    #[test]
    #[serial]
    fn test_missing_inheritance() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MissingInheritance.sol",
        );

        let mut detector = MissingInheritanceDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
