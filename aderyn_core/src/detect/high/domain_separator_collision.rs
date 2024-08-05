use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{ASTNode, NodeID, Visibility};

use crate::capture;
use crate::context::browser::{ExtractFunctionDefinitions, ExtractVariableDeclarations};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DomainSeparatorCollisionDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DomainSeparatorCollisionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Analyze each contract in
        for current_contract in context.contract_definitions() {
            if let Some(contract_ids) = current_contract.linearized_base_contracts.as_ref() {
                let current_contract_is_erc20 = contract_ids.iter().any(|i| {
                    context.nodes.get(i).is_some_and(|c| {
                        if let ASTNode::ContractDefinition(contract) = c {
                            if contract.name.contains("ERC20") {
                                return true;
                            }
                        }
                        false
                    })
                });

                if !current_contract_is_erc20 {
                    continue;
                }

                // Now we know that current contract is an ERC20
                // So we need to go through all the public/external functions + state variables
                // in it's inheritance hirearchy and make sure that it doesn't collide with DOMAIN_SEPARATOR() sign

                for contract_id in contract_ids {
                    if let Some(ASTNode::ContractDefinition(contract)) =
                        context.nodes.get(contract_id)
                    {
                        // First deal with the variables, then tackle the functions
                        let variables = ExtractVariableDeclarations::from(contract).extracted;

                        for var in variables {
                            if var.visibility != Visibility::Public || !var.state_variable {
                                continue;
                            }
                            // Now we know it's a public state variable
                            // So we check to see the function signature matches!
                            if var.name == "DOMAIN_SEPARATOR"
                                && var
                                    .type_descriptions
                                    .type_string
                                    .as_ref()
                                    .is_some_and(|ts| {
                                        !(ts.starts_with("mapping(") || ts.ends_with("]"))
                                    })
                            {
                                // We eliminate the cases of mappings and arrays because they take parameters
                                // so it clearly does not collide with DOMAIN_SEPARATOR()
                                capture!(self, context, var);
                            } else {
                                // Now, cover the case where the name could be different, but the function selector matches

                                if var
                                    .function_selector
                                    .as_ref()
                                    .is_some_and(|selector| selector == "3644e515")
                                {
                                    // cast sig "function DOMAIN_SEPARATOR() external view returns (bytes32)"
                                    // 0x3644e515

                                    capture!(self, context, var);
                                }
                            }
                        }

                        // Now deal with the functions
                        let functions = ExtractFunctionDefinitions::from(contract).extracted;

                        for func in functions {
                            if (func.visibility != Visibility::Public
                                && func.visibility != Visibility::External)
                                || !func.implemented
                            {
                                continue;
                            }
                            // Now we know the function is either public/external and is implemented
                            if func.name == "DOMAIN_SEPARATOR"
                                && func.parameters.parameters.is_empty()
                            {
                                capture!(self, context, func);
                            } else {
                                if func
                                    .function_selector
                                    .as_ref()
                                    .is_some_and(|selector| selector == "3644e515")
                                {
                                    // cast sig "function DOMAIN_SEPARATOR() external view returns (bytes32)"
                                    // 0x3644e515

                                    capture!(self, context, func);
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
        String::from("Domain Separator collision for ERC20 contracts.")
    }

    fn description(&self) -> String {
        String::from("Remove or rename the pubic/external variable or function that collides with DOMAIN_SEPARATOR()")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::DomainSeparatorCollision)
    }
}

#[cfg(test)]
mod domain_separator_collision_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::domain_separator_collision::DomainSeparatorCollisionDetector,
    };

    #[test]
    #[serial]
    fn test_domain_separator_collision() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DomainSeparatorCollision.sol",
        );

        let mut detector = DomainSeparatorCollisionDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
    }
}
