//! Router
//!
//! Function router.
//!
//! TODO
//! * Help resolve internal calls to corresponding function definitions
//! * Help guess external calls to corresponding function definitions

use std::collections::HashMap;

use crate::{
    ast::{ContractDefinition, Visibility},
    context::workspace_context::WorkspaceContext,
};

pub fn build_router(context: &WorkspaceContext) {
    let contracts = context.deployable_contracts();
    for _contract in contracts {}
}

pub fn build_ic_router_for_contract(
    context: &WorkspaceContext,
    base_contract: &ContractDefinition,
) {
    let c3 = base_contract.c3(context).collect::<Vec<_>>();
    for (idx, starting_point) in c3.iter().enumerate() {
        let mut routes = HashMap::new();
        for contract in c3.iter().skip(idx) {
            for func in contract.function_definitions() {
                if matches!(func.visibility, Visibility::Private | Visibility::External) {
                    continue;
                }
                routes.insert(func.selectorish(), func.id);
            }
        }
    }
}

#[cfg(test)]
mod mir_router {
    use crate::test_utils::load_solidity_source_unit;

    use super::build_ic_router_for_contract;

    #[test]
    pub fn same_contract_internal_call() {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/router/InternalCalls.sol");

        let contract = context.find_contract_by_name("Basic2");
        build_ic_router_for_contract(&context, contract);
    }
}
