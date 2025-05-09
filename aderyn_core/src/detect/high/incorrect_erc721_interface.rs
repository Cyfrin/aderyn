use std::{collections::BTreeMap, error::Error};

use crate::ast::{ASTNode, NodeID, Visibility};

use crate::{
    capture,
    context::{browser::ExtractFunctionDefinitions, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct IncorrectERC721InterfaceDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for IncorrectERC721InterfaceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Analyze each contract in context
        for current_contract in context.contract_definitions() {
            // Look through it's inheritance hierarchy to determine if it's an ERC721
            let contract_ids = &current_contract.linearized_base_contracts;
            let current_contract_is_erc721 = contract_ids.iter().any(|i| {
                context.nodes.get(i).is_some_and(|c| {
                    if let ASTNode::ContractDefinition(contract) = c {
                        if contract.name.contains("ERC721") {
                            return true;
                        }
                    }
                    false
                })
            });

            if !current_contract_is_erc721 {
                continue;
            }

            // Now we know that current contract is an ERC721

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

                        if func.represents_erc721_balance_of() && !func.returns_uint256() {
                            capture!(self, context, func);
                        }

                        if (func.represents_erc721_get_approved()
                            || func.represents_erc721_owner_of())
                            && !func.returns_address()
                        {
                            capture!(self, context, func);
                        }

                        if (func.represents_erc721_safe_transfer_from()
                            || func.represents_erc721_transfer_from())
                            && !func.returns_nothing()
                        {
                            capture!(self, context, func);
                        }

                        if (func.represents_erc721_approve()
                            || func.represents_erc721_set_approval_for_all())
                            && !func.returns_nothing()
                        {
                            capture!(self, context, func);
                        }

                        if func.represents_erc721_is_approved_for_all() && !func.returns_bool() {
                            capture!(self, context, func);
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
        String::from("Incorrect ERC721 interface")
    }

    fn description(&self) -> String {
        String::from("Incorrect return values for ERC721 functions. A contract compiled with Solidity > 0.4.22 \
            interacting with these functions will fail to execute them, as the return value is missing. Set the \
            appropriate return values and types for the defined ERC721 functions."
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::IncorrectERC721Interface)
    }
}

mod erc721_matching_function_signature_helper {
    //! This module matches function signature only (name + parameters)
    //! This means, that the return value could be different.

    use crate::ast::FunctionDefinition;

    struct SignatureMatcher<'a> {
        name: &'a str,
        parameter_types: Vec<&'a str>,
    }

    // Helps with checking if a function definition satisfied a signature matcher
    impl SignatureMatcher<'_> {
        fn satisfies(&self, func: &FunctionDefinition) -> bool {
            if func.name != self.name {
                return false;
            }
            let params = &func.parameters.parameters;
            if params.len() != self.parameter_types.len() {
                return false;
            }
            #[allow(clippy::needless_range_loop)]
            for idx in 0..params.len() {
                if let Some(func_param_type) = params[idx].type_descriptions.type_string.as_ref() {
                    let target = &self.parameter_types[idx];
                    if *target == "address" {
                        if func_param_type == "address" || func_param_type == "address payable" {
                            continue;
                        } else {
                            return false;
                        }
                    } else if func_param_type != target {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }
    }

    // ERC721 function signature matching
    impl FunctionDefinition {
        pub fn represents_erc721_get_approved(&self) -> bool {
            let satisifer =
                SignatureMatcher { name: "getApproved", parameter_types: vec!["uint256"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc721_transfer_from(&self) -> bool {
            let satisifer = SignatureMatcher {
                name: "transferFrom",
                parameter_types: vec!["address", "address", "uint256"],
            };
            satisifer.satisfies(self)
        }

        pub fn represents_erc721_approve(&self) -> bool {
            let satisifer =
                SignatureMatcher { name: "approve", parameter_types: vec!["address", "uint256"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc721_is_approved_for_all(&self) -> bool {
            let satisifer = SignatureMatcher {
                name: "isApprovedForAll",
                parameter_types: vec!["address", "address"],
            };
            satisifer.satisfies(self)
        }

        pub fn represents_erc721_balance_of(&self) -> bool {
            let satisifer =
                SignatureMatcher { name: "balanceOf", parameter_types: vec!["address"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc721_owner_of(&self) -> bool {
            let satisifer = SignatureMatcher { name: "ownerOf", parameter_types: vec!["uint256"] };
            satisifer.satisfies(self)
        }

        pub fn represents_erc721_safe_transfer_from(&self) -> bool {
            let type_1_staisfier = SignatureMatcher {
                name: "safeTransferFrom",
                parameter_types: vec!["address", "address", "uint256", "bytes"],
            };

            let type_2_satisifer = SignatureMatcher {
                name: "safeTransferFrom",
                parameter_types: vec!["address", "address", "uint256"],
            };

            type_1_staisfier.satisfies(self) || type_2_satisifer.satisfies(self)
        }

        pub fn represents_erc721_set_approval_for_all(&self) -> bool {
            let satisfier = SignatureMatcher {
                name: "setApprovalForAll",
                parameter_types: vec!["address", "bool"],
            };
            satisfier.satisfies(self)
        }
    }

    // Helpers to match return types (bool & uint256)
    impl FunctionDefinition {
        pub fn returns_nothing(&self) -> bool {
            self.return_parameters.parameters.is_empty()
        }

        pub fn returns_bool(&self) -> bool {
            let params = &self.return_parameters.parameters;
            params.len() == 1
                && params[0]
                    .type_descriptions
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| type_string == "bool")
        }

        pub fn returns_uint256(&self) -> bool {
            let params = &self.return_parameters.parameters;
            params.len() == 1
                && params[0]
                    .type_descriptions
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| type_string == "uint256")
        }

        pub fn returns_address(&self) -> bool {
            let params = &self.return_parameters.parameters;
            params.len() == 1
                && params[0].type_descriptions.type_string.as_ref().is_some_and(|type_string| {
                    type_string == "address" || type_string == "address payable"
                })
        }
    }
}

#[cfg(test)]
mod incorrect_erc721_tests {

    use crate::detect::{
        detector::IssueDetector, high::incorrect_erc721_interface::IncorrectERC721InterfaceDetector,
    };

    #[test]

    fn test_incorrect_erc721_functions() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/IncorrectERC721.sol",
        );

        let mut detector = IncorrectERC721InterfaceDetector::default();
        let found = detector.detect(&context).unwrap();

        // We capture every faulty method in the IncorrectERC721 contract that has the wrong return
        // type

        assert!(found);

        assert_eq!(detector.instances().len(), 8);
    }
}
