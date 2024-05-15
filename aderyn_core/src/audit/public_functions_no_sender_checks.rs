use prettytable::{row, Row};

use super::auditor::AuditorDetector;
use crate::{
    ast::{
        NodeType,
    },
    context::{
        browser::{
            ExtractModifierInvocations, Peek,
        },
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::helpers::{
        get_implemented_external_and_public_functions,
        has_msg_sender_binary_operation,
    },
};
use std::{
    error::Error,
};

#[derive(Clone)]
pub struct NoChecksInstance {
    pub contract_name: String,
    pub function_name: String,
    pub source_code: String,
}

#[derive(Default)]
pub struct PublicFunctionsNoSenderChecks {
    found_instances: Vec<NoChecksInstance>,
}

impl AuditorDetector for PublicFunctionsNoSenderChecks {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let functions =
            get_implemented_external_and_public_functions(context).filter(|function_definition| {
                // Check if the function has an owner-related modifier
                let does_not_have_an_owner_modifier =
                    !ExtractModifierInvocations::from(*function_definition)
                        .extracted
                        .iter()
                        .any(|modifier| {
                            modifier.modifier_name.name == "onlyOwner"
                                || modifier.modifier_name.name == "onlyAdmin"
                                || modifier.modifier_name.name == "onlyRole"
                                || modifier.modifier_name.name == "requiresAuth"
                        });
                // Check if the function has a `msg.sender` BinaryOperation check
                let has_msg_sender_binary_operation =
                    has_msg_sender_binary_operation(function_definition);
                // TODO Check if the function has a hasRole identifier with msg.sender as an arg
                does_not_have_an_owner_modifier && !has_msg_sender_binary_operation
            });

        functions.for_each(|function_definition| {
            if let ASTNode::ContractDefinition(contract_definition) = context
                .get_closest_ancestor(function_definition.id, NodeType::ContractDefinition)
                .unwrap()
            {
                let contract_name = contract_definition.name.clone();
                self.found_instances.push(NoChecksInstance {
                    contract_name,
                    function_name: function_definition.name.clone(),
                    source_code: function_definition.peek(context).unwrap(),
                });
            }
        });

        println!("Number of instances: {}", self.found_instances.len());

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Attack Surface - External Contract `call` and `delegatecall` Instances")
    }

    fn table_titles(&self) -> Row {
        row!["Contract", "Function", "Code"]
    }

    fn table_rows(&self) -> Vec<Row> {
        self.found_instances
            .iter()
            .map(|instance| {
                row![
                    instance.contract_name,
                    instance.function_name,
                    instance.source_code
                ]
            })
            .collect()
    }
}

#[cfg(test)]
mod attack_surface_detector_tests {
    use crate::{
        audit::{
            auditor::AuditorDetector,
            public_functions_no_sender_checks::PublicFunctionsNoSenderChecks,
        },
        detect::detector::detector_test_helpers::load_multiple_contracts,
    };

    #[test]
    fn test_attack_surface_detector() {
        let context =
            load_multiple_contracts(vec![
                "../tests/contract-playground/out/PublicFunctionsWithoutSenderCheck.sol/OwnableExamples.json",
                "../tests/contract-playground/out/PublicFunctionsWithoutSenderCheck.sol/AccessControlExamples.json",
                "../tests/contract-playground/out/PublicFunctionsWithoutSenderCheck.sol/ManualCheckExamples.json",]
                );

        let mut detector = PublicFunctionsNoSenderChecks::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        assert!(detector.found_instances.len() == 3);
    }
}
