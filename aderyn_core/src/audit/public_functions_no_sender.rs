use prettytable::{row, Row};

use super::auditor::AuditorDetector;
use crate::{
    ast::{FunctionKind, NodeType},
    context::{
        browser::ExtractModifierInvocations,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::helpers::{
        get_implemented_external_and_public_functions, has_msg_sender_binary_operation,
    },
};
use std::{cmp::Ordering, collections::BTreeSet, error::Error};

#[derive(Clone, Eq, PartialEq)]
pub struct NoChecksInstance {
    pub contract_name: String,
    pub function_name: String,
    pub function_kind: FunctionKind,
}

impl Ord for NoChecksInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_contract = self.contract_name.cmp(&other.contract_name);
        if by_contract == Ordering::Equal {
            self.function_name.cmp(&other.function_name)
        } else {
            by_contract
        }
    }
}

impl PartialOrd for NoChecksInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct PublicFunctionsNoSenderChecksDetector {
    // contract_name, function_name
    found_instances: BTreeSet<NoChecksInstance>,
}

impl AuditorDetector for PublicFunctionsNoSenderChecksDetector {
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
                self.found_instances.insert(NoChecksInstance {
                    contract_name,
                    function_name: function_definition.name.clone(),
                    function_kind: function_definition.kind.clone(),
                });
            }
        });

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Public and External Functions Without `msg.sender` Checks")
    }

    fn table_titles(&self) -> Row {
        row!["Contract", "Function Kind", "Function Name"]
    }

    fn table_rows(&self) -> Vec<Row> {
        self.found_instances
            .iter()
            .map(|instance| {
                row![
                    instance.contract_name,
                    instance.function_kind,
                    instance.function_name,
                ]
            })
            .collect()
    }

    fn skeletal_clone(&self) -> Box<dyn AuditorDetector> {
        Box::<PublicFunctionsNoSenderChecksDetector>::default()
    }
}

#[cfg(test)]
mod public_functions_no_sender_checks {
    use crate::{
        audit::{
            auditor::AuditorDetector,
            public_functions_no_sender::PublicFunctionsNoSenderChecksDetector,
        },
        detect::test_utils::load_solidity_source_unit,
    };

    #[test]
    fn test_public_functions_no_sender_checks() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/auditor_mode/PublicFunctionsWithoutSenderCheck.sol",
        );

        let mut detector = PublicFunctionsNoSenderChecksDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        assert!(detector.found_instances.len() == 5);
    }
}
