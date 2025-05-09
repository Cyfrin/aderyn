use std::{collections::BTreeMap, convert::identity, error::Error};

use crate::ast::{ASTNode, NodeID, Visibility};

use crate::{
    capture,
    context::{browser::ExtractFunctionDefinitions, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct IncorrectERC20InterfaceDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for IncorrectERC20InterfaceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Analyze each contract in context
        for current_contract in context.contract_definitions() {
            // Look through it's inheritance hierarchy to determine if it's an ERC20
            let contract_ids = &current_contract.linearized_base_contracts;
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

            for contract_id in contract_ids {
                if let Some(ASTNode::ContractDefinition(contract)) = context.nodes.get(contract_id)
                {
                    let functions = ExtractFunctionDefinitions::from(contract).extracted;

                    for func in functions {
                        if (func.visibility != Visibility::Public
                            && func.visibility != Visibility::External)
                            || !func.implemented
                        {
                            continue;
                        }

                        if (func.represents_erc20_transfer().is_some_and(identity)
                            || func.represents_erc20_transfer_from().is_some_and(identity)
                            || func.represents_erc20_approve().is_some_and(identity))
                            && !func.returns_bool()
                        {
                            capture!(self, context, func);
                        }

                        if (func.represents_erc20_allowance().is_some_and(identity)
                            || func.represents_erc20_balance_of().is_some_and(identity)
                            || func.represents_erc20_total_supply().is_some_and(identity))
                            && !func.returns_uint256()
                        {
                            capture!(self, context, func)
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
        String::from("Incorrect ERC20 interface")
    }

    fn description(&self) -> String {
        String::from("Incorrect return values for ERC20 functions. A contract compiled with Solidity > 0.4.22 \
            interacting with these functions will fail to execute them, as the return value is missing. Set the \
            appropriate return values and types for the defined ERC20 functions."
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::IncorrectERC20Interface)
    }
}

mod erc_matching_function_signature_helper {
    //! This module matches function signature only (name + parameters)
    //! This means, that the return value could be different.

    use crate::ast::FunctionDefinition;

    struct SignatureMatcher<'a> {
        name: &'a str,
        parameter_types: Vec<&'a str>,
    }

    // Helps with checking if a function definition satisfied a signature matcher
    impl SignatureMatcher<'_> {
        fn satisfies(&self, func: &FunctionDefinition) -> Option<bool> {
            if func.name != self.name {
                return Some(false);
            }
            let params = &func.parameters.parameters;
            if params.len() != self.parameter_types.len() {
                return Some(false);
            }
            #[allow(clippy::needless_range_loop)]
            for idx in 0..params.len() {
                if let Some(func_param_type) = params[idx].type_descriptions.type_string.as_ref() {
                    let target = &self.parameter_types[idx];
                    if *target == "address" {
                        if func_param_type == "address" || func_param_type == "address payable" {
                            continue;
                        } else {
                            return Some(false);
                        }
                    } else if func_param_type != target {
                        return Some(false);
                    }
                } else {
                    return None;
                }
            }
            Some(true)
        }
    }

    // ERC20 function signature matching
    impl FunctionDefinition {
        pub fn represents_erc20_transfer(&self) -> Option<bool> {
            let satisifer =
                SignatureMatcher { name: "transfer", parameter_types: vec!["address", "uint256"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc20_transfer_from(&self) -> Option<bool> {
            let satisifer = SignatureMatcher {
                name: "transferFrom",
                parameter_types: vec!["address", "address", "uint256"],
            };
            satisifer.satisfies(self)
        }

        pub fn represents_erc20_approve(&self) -> Option<bool> {
            let satisifer =
                SignatureMatcher { name: "approve", parameter_types: vec!["address", "uint256"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc20_allowance(&self) -> Option<bool> {
            let satisifer =
                SignatureMatcher { name: "allowance", parameter_types: vec!["address", "address"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc20_balance_of(&self) -> Option<bool> {
            let satisifer =
                SignatureMatcher { name: "balanceOf", parameter_types: vec!["address"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc20_total_supply(&self) -> Option<bool> {
            let satisifer = SignatureMatcher { name: "totalSupply", parameter_types: vec![] };
            satisifer.satisfies(self)
        }
    }
}

#[cfg(test)]
mod incorrect_erc20_tests {

    use crate::detect::{
        detector::IssueDetector, high::incorrect_erc20_interface::IncorrectERC20InterfaceDetector,
    };

    #[test]

    fn test_incorrect_erc20_functions() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/IncorrectERC20.sol",
        );

        let mut detector = IncorrectERC20InterfaceDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 5);
    }
}
