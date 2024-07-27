use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{ContractKind, FunctionDefinition, NodeID, TypeName};

use crate::capture;
use crate::context::browser::GetImmediateChildren;
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
        for interface in context
            .contract_definitions()
            .into_iter()
            .filter(|&c| c.kind == ContractKind::Interface)
        {
            println!("Intreface: {:?}", interface.name);
            let interface_functions = interface.function_definitions();
            for interface_function in interface_functions {
                println!(
                    "{} - {:?}",
                    interface_function.name,
                    construct_function_signaturish(interface_function)
                );
            }
        }

        for contract in context
            .contract_definitions()
            .into_iter()
            .filter(|&c| c.kind == ContractKind::Contract)
        {
            println!("Contract: {:?}", contract.name);
            let contract_functions = contract.function_definitions();
            for contract_function in contract_functions.iter().filter(|f| f.implemented) {
                println!(
                    "{} - {:?}",
                    contract_function.name,
                    construct_function_signaturish(contract_function)
                );
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

/// A regulat function signature looks something like this `func1(bool,address,uint256)`
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
        assert_eq!(detector.title(), String::from("High Issue Title"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Description of the high issue.")
        );
    }
}
