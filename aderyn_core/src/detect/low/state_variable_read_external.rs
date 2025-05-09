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
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct StateVariableReadExternalDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StateVariableReadExternalDetector {
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
        String::from("State Variable is Read as External")
    }

    fn description(&self) -> String {
        String::from(
            "The contract reads it's own state variable using `this` which adds an unnecessary STATICCALL. Consider removing `this` to access the variable from storage.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::StateVariableReadExternal.to_string()
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
                    if let Some(ASTNode::MemberAccess(member_access)) = context.nodes.get(id) {
                        member_names.push(member_access)
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
    Ok(contract
        .linearized_base_contracts
        .iter()
        .flat_map(|ancestor_id| {
            if let Some(ancestor) = context.nodes.get(ancestor_id) {
                let public_variable_declarations =
                    ExtractVariableDeclarations::from(ancestor).extracted;
                return Some(
                    public_variable_declarations
                        .into_iter()
                        .filter(|declaration| {
                            declaration.state_variable
                                && declaration.visibility == Visibility::Public
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

    use crate::detect::{
        detector::IssueDetector,
        low::state_variable_read_external::StateVariableReadExternalDetector,
    };

    #[test]

    fn test_public_variable_read_in_external_context() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/PublicVariableReadInExternalContext.sol",
        );

        let mut detector = StateVariableReadExternalDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 4);
    }
}
