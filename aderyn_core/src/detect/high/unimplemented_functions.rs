use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::{ASTNode, ContractDefinition, ContractKind, FunctionDefinition, NodeID};

use crate::capture;
use crate::context::browser::ExtractVariableDeclarations;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnimplementedFunctionsDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnimplementedFunctionsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context
            .contract_definitions()
            .into_iter()
            .filter(|&c| c.kind == ContractKind::Contract)
        {
            // dbg!(contract.name.clone());
            if let Some(mut unimplemented_signaturishes) =
                get_function_signaturish_of_all_functions_in_linearized_chain(
                    context, contract, false,
                )
            {
                if let Some(implemented_signaturishes) =
                    get_function_signaturish_of_all_functions_in_linearized_chain(
                        context, contract, true,
                    )
                {
                    // First, cancel away the unimplemented signaturishes of those functions that have been
                    // implemented as public variable declarations in derived contracts. Then, from the remaining ones,
                    // cancel away those that have been implemented as functions in the derived contracts.
                    let all_state_variable_names =
                        get_all_state_variables_names_of_contract(contract);

                    unimplemented_signaturishes
                        .retain(|f| !all_state_variable_names.iter().any(|s| f.contains(s)));

                    // If any of the remaining funcs have not been implemented as a function in derived contracts,
                    // then there is at least 1 function that has been left unimplemented. So we capture the contract.
                    for unimplemented_signaturish in unimplemented_signaturishes {
                        if !implemented_signaturishes.contains(&unimplemented_signaturish) {
                            // This means, there is at least 1 function that has not been implemented but is
                            // present in the linearized base contracts as a function definition directly as
                            // a child of some base contract of this derived contract.
                            // dbg!(unimplemented_signaturish.clone());

                            if let Some(is_abstract) = contract.is_abstract {
                                if !is_abstract {
                                    capture!(self, context, contract)
                                }
                            } else {
                                // Support old versions of Solidity (< 0.6) where abstract contracts can only
                                // be detected by seeing if a contract's function definition ended with a `;`
                                // wheras with the later syntax, you get `is_abstract` injected into the AST
                                // So that helps us a little bit more when it comes to deciding whether we should
                                // capture the contract or not.
                                capture!(self, context, contract);
                            }

                            // We have determined this contract has missing functions, so now go and explore
                            // other contracts. That's why break from this for loop.
                            break;
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
        String::from("Contract has missing function implementations")
    }

    fn description(&self) -> String {
        String::from("There is at least 1 function in the base contracts of this contract that has not been implemented.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::UnimplementedFunctions.to_string()
    }
}

fn get_all_state_variables_names_of_contract(contract: &ContractDefinition) -> Vec<String> {
    let state_variables = ExtractVariableDeclarations::from(contract).extracted;
    state_variables
        .into_iter()
        .filter(|v| v.state_variable)
        .map(|v| v.name.clone())
        .collect()
}

fn get_function_signaturish_of_all_functions_in_linearized_chain(
    context: &WorkspaceContext,
    contract: &ContractDefinition,
    function_is_implemented: bool,
) -> Option<HashSet<String>> {
    let mut function_signaturishes = HashSet::new();

    let base_contracts_ids = contract.linearized_base_contracts.clone()?;
    for base_contract_id in base_contracts_ids {
        let base_contract = context.nodes.get(&base_contract_id)?;
        if let ASTNode::ContractDefinition(contract_definition) = base_contract {
            for func in contract_definition
                .function_definitions()
                .into_iter()
                .filter(|&f| (f.implemented == function_is_implemented))
            {
                if let Ok(func_signaturish) = construct_function_signaturish(func) {
                    function_signaturishes.insert(func_signaturish);
                }
            }
        }
    }

    Some(function_signaturishes)
}

/// A regular function signature looks something like this `func1(bool,address,uint256)`
/// But.... this is a signaturish - It's made up term that looks something like this `func1(t_bool,t_address,..,)`
/// The advantage being we can got those values from type_identifiers
fn construct_function_signaturish(
    function_definition: &FunctionDefinition,
) -> Result<String, Box<dyn Error>> {
    let function_name = function_definition.name.clone();

    let mut params_separated_by_comma = String::new();

    for parameter in function_definition.parameters.parameters.iter() {
        let type_id = parameter
            .type_descriptions
            .type_identifier
            .clone()
            .ok_or(eyre::eyre!("type id not found for at least one paramter !"))?;
        params_separated_by_comma.push_str(&type_id);
        params_separated_by_comma.push(',');
    }

    Ok(format!("{}({})", function_name, params_separated_by_comma))
}

#[cfg(test)]
mod unimplemented_functions_detector_tests {
    use crate::detect::{
        detector::IssueDetector, high::unimplemented_functions::UnimplementedFunctionsDetector,
    };

    #[test]
    fn test_unimplemented_functions() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnimplementedFunctions.sol",
        );

        let mut detector = UnimplementedFunctionsDetector::default();
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
        assert_eq!(
            detector.title(),
            String::from("Contract has missing function implementations")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("There is at least 1 function in the base contracts of this contract that has not been implemented.")
        );
    }
}
