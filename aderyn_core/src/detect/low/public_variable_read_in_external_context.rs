use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::ast::{
    ASTNode, ContractDefinition, Expression, Identifier, MemberAccess, NodeID, Visibility,
};

use crate::{
    capture,
    context::{
        browser::{ExtractFunctionCalls, ExtractVariableDeclarations},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::{eyre, Result};

#[derive(Default)]
pub struct PublicVariableReadInExternalContextDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for PublicVariableReadInExternalContextDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.contract_definitions() {
            // Public state variables including the base contracts
            if let Ok(public_state_variable_names) =
                find_all_public_state_variables_names_for_contract(context, contract)
            {
                // Find all the `X`s that appear with the pattern `this.X()`
                let this_member_accesses =
                    find_all_public_member_names_called_using_this_keyword_in_contract(
                        context, contract,
                    );

                for member_access in this_member_accesses {
                    if public_state_variable_names.contains(&member_access.member_name) {
                        capture!(self, context, member_access);
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
        String::from("Public variables of a contract read in an external context (using `this`).")
    }

    fn description(&self) -> String {
        String::from(
            "The contract reads it's own variable using `this` which adds an unnecessary STATICCALL. Remove `this` and access the variable like storage.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::PublicVariableReadInExternalContext.to_string()
    }
}

fn find_all_public_member_names_called_using_this_keyword_in_contract<'a>(
    context: &'a WorkspaceContext,
    contract: &ContractDefinition,
) -> Vec<&'a MemberAccess> {
    let mut member_names = vec![];

    let function_calls = ExtractFunctionCalls::from(contract).extracted;
    for function_call in function_calls {
        if let Expression::MemberAccess(MemberAccess { id, expression, .. }) =
            function_call.expression.as_ref()
        {
            if let Expression::Identifier(Identifier { name, .. }) = expression.as_ref() {
                if name == "this" {
                    if let Some(ASTNode::MemberAccess(member_acess)) = context.nodes.get(id) {
                        member_names.push(member_acess)
                    }
                }
            }
        }
    }

    member_names
}

// Scans the linearized base contracts and returns a list of all the NodeIDs of public variable
// declarations
fn find_all_public_state_variables_names_for_contract(
    context: &WorkspaceContext,
    contract: &ContractDefinition,
) -> Result<HashSet<String>, Box<dyn Error>> {
    let inheritance_ancestors =
        contract.linearized_base_contracts.as_ref().ok_or(eyre!("base contracts not found!"))?;

    Ok(inheritance_ancestors
        .iter()
        .flat_map(|ancestor_id| {
            if let Some(ancestor) = context.nodes.get(ancestor_id) {
                let public_variable_declaraions =
                    ExtractVariableDeclarations::from(ancestor).extracted;
                return Some(
                    public_variable_declaraions
                        .into_iter()
                        .filter(|declaration| {
                            declaration.state_variable &&
                                declaration.visibility == Visibility::Public
                        })
                        .collect::<Vec<_>>(),
                );
            }
            None
        })
        .flatten()
        .map(|v| v.name.clone())
        .collect())
}

#[cfg(test)]
mod public_variable_read_in_external_context_detector_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::public_variable_read_in_external_context::PublicVariableReadInExternalContextDetector,
    };

    #[test]
    #[serial]
    fn test_public_variable_read_in_external_context() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/PublicVariableReadInExternalContext.sol",
        );

        let mut detector = PublicVariableReadInExternalContextDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is low
        assert_eq!(detector.severity(), crate::detect::detector::IssueSeverity::Low);
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from(
                "Public variables of a contract read in an external context (using `this`)."
            )
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "The contract reads it's own variable using `this` which adds an unnecessary STATICCALL. Remove `this` and access the variable like storage.",
            )
        );
    }
}
