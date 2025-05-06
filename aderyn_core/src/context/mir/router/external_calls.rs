use super::Router;
use crate::{
    ast::{ContractDefinition, FunctionCall, FunctionDefinition, NodeID},
    context::workspace::WorkspaceContext,
};
use std::collections::HashMap;

impl Router {
    pub(crate) fn _resolve_external_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
}

pub(crate) fn build_ec_router_for_contract(
    context: &WorkspaceContext,
    base_contract: &ContractDefinition,
) -> HashMap<String, NodeID> {
    HashMap::new()
}
