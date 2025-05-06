use std::{collections::BTreeMap, error::Error};

use crate::ast::{ContractKind, NodeID, NodeType};

use crate::{
    capture,
    context::{
        browser::{ExtractVariableDeclarations, GetClosestAncestorOfTypeX},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct LocalVariableShadowingDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for LocalVariableShadowingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context
            .contract_definitions()
            .into_iter()
            .filter(|&c| c.kind != ContractKind::Library && !c.is_abstract)
        {
            let current_contract_variables = ExtractVariableDeclarations::from(contract).extracted;
            let local_contract_variables = current_contract_variables
                .into_iter()
                .filter(|v| !v.state_variable)
                .collect::<Vec<_>>();
            if let Some(state_variables) =
                contract.get_all_state_variables_in_linearized_base_contracts_chain(context)
            {
                for local_contract_variable in local_contract_variables {
                    if state_variables.iter().any(|v| v.name == local_contract_variable.name) {
                        // It's okay to allow EventDefinitions/ ErrorDefinitions to shadow the state
                        // variable name
                        if local_contract_variable
                            .closest_ancestor_of_type(context, NodeType::EventDefinition)
                            .is_some()
                            || local_contract_variable
                                .closest_ancestor_of_type(context, NodeType::ErrorDefinition)
                                .is_some()
                        {
                            continue;
                        }

                        capture!(self, context, local_contract_variable);
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
        String::from("Local Variable Shadows State Variable")
    }

    fn description(&self) -> String {
        String::from("Rename the local variable that shadows another state variable.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::LocalVariableShadowing)
    }
}

mod contract_hierarchy_variable_helpers {
    use crate::{
        ast::{ASTNode, ContractDefinition, VariableDeclaration},
        context::{browser::ExtractVariableDeclarations, workspace::WorkspaceContext},
    };

    impl ContractDefinition {
        pub fn get_all_state_variables_in_linearized_base_contracts_chain(
            &self,
            context: &WorkspaceContext,
        ) -> Option<Vec<VariableDeclaration>> {
            let mut all_state_variable_ids = vec![];
            for contract_id in self.linearized_base_contracts.iter() {
                if let ASTNode::ContractDefinition(c) = context.nodes.get(contract_id)? {
                    let variable_declarations = ExtractVariableDeclarations::from(c).extracted;
                    all_state_variable_ids
                        .extend(variable_declarations.into_iter().filter(|v| v.state_variable))
                }
            }
            Some(all_state_variable_ids)
        }
    }
}

#[cfg(test)]
mod local_variable_shadowing_tests {

    use crate::detect::{
        detector::IssueDetector, low::local_variable_shadowing::LocalVariableShadowingDetector,
    };

    #[test]

    fn test_local_variable_shadowing() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/LocalVariableShadow.sol",
        );

        let mut detector = LocalVariableShadowingDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        assert_eq!(detector.instances().len(), 3);
    }
}
