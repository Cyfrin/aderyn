use super::{ECDest, Router};
use crate::{
    ast::{ContractDefinition, FunctionCall, FunctionKind, Visibility},
    context::workspace::WorkspaceContext,
};
use std::collections::{hash_map::Entry, HashMap};

impl Router {
    pub(super) fn _resolve_external_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<ECDest> {
        // do not resolve if it's internal function call
        if func_call.is_internal_call() == Some(true) {
            return None;
        }

        // check if it's illegal base contract type
        if !base_contract.is_deployable_contract() {
            return None;
        }

        let suspect = func_call.suspected_target_function(context)?;
        let selector = suspect.function_selector.as_ref()?;
        let lookup_index = self.external_calls.get(&base_contract.id)?;

        match lookup_index.routes.get(selector) {
            Some(resolved) => Some(resolved.clone()),
            None => lookup_index.routes.get("FALLBACK").cloned(), /* it cannot be RECEIVE because
                                                                   * data is non empty */
        }
    }
}

/// If function selector field isn't present, this algorithm cannot work.
/// Therefore, if for some reason it's not found, we return an empty hashmap
pub(super) fn build_ec_router_for_contract(
    context: &WorkspaceContext,
    base_contract: &ContractDefinition,
) -> HashMap<String, ECDest> {
    let c3 = base_contract.c3(context).collect::<Vec<_>>();
    let mut routes = HashMap::new();
    for contract in c3.iter() {
        // Loop through public state variables
        for var in contract.top_level_variables() {
            if var.visibility == Visibility::Public {
                let Some(func_selector) = var.function_selector.as_ref() else {
                    return HashMap::new();
                };
                if let Entry::Vacant(e) = routes.entry(func_selector.to_string()) {
                    e.insert(ECDest::PseduoExtFn(var.id));
                }
            }
        }
        // Loop through externally available functions
        for func in contract.function_definitions() {
            match *func.kind() {
                FunctionKind::Function => {
                    match func.visibility {
                        Visibility::Public => {
                            let Some(func_selector) = func.function_selector.as_ref() else {
                                return HashMap::new();
                            };
                            if let Entry::Vacant(e) = routes.entry(func_selector.to_string()) {
                                e.insert(ECDest::PublicFn(func.id));
                            }
                        }
                        Visibility::External => {
                            let Some(func_selector) = func.function_selector.as_ref() else {
                                return HashMap::new();
                            };
                            if let Entry::Vacant(e) = routes.entry(func_selector.to_string()) {
                                e.insert(ECDest::RealExtFn(func.id));
                            }
                        }
                        _ => (),
                    };
                }
                FunctionKind::Receive => {
                    if let Entry::Vacant(e) = routes.entry("RECEIVE".to_string()) {
                        e.insert(ECDest::Receive(func.id));
                    }
                }
                FunctionKind::Fallback => {
                    if let Entry::Vacant(e) = routes.entry("FALLBACK".to_string()) {
                        e.insert(ECDest::Fallback(func.id));
                    }
                }
                FunctionKind::FreeFunction => unreachable!(),
                FunctionKind::Constructor => (),
            };
        }
    }
    routes
}
