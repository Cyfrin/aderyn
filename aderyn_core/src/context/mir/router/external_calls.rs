use super::{ECDest, Router};
use crate::{
    ast::{ContractDefinition, FunctionCall, FunctionDefinition},
    context::workspace::WorkspaceContext,
};
use std::collections::HashMap;

impl Router {
    pub(super) fn _resolve_external_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
}

pub(super) fn build_ec_router_for_contract(
    context: &WorkspaceContext,
    base_contract: &ContractDefinition,
) -> HashMap<String, ECDest> {
    HashMap::new()
}
