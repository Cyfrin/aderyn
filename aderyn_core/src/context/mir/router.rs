//! Router
//!
//! Function router.
//!
//! Currently it is only used for internal calls.

mod external_calls;
mod internal_calls;
mod modifier_calls;
mod tests;

use external_calls::build_ec_router_for_contract;
use internal_calls::build_ic_router_for_contract;
use modifier_calls::build_mc_router_for_contract;

use crate::{ast::*, context::workspace::WorkspaceContext};
use std::collections::HashMap;

/// Router
///
/// ---------
///
/// Given an instantiable base contract, this helps resolve calls to definitions that are either
/// in the base contract's inheritance hierarchy or an internal function of library.
///
/// When control flow stays within the contract:
///
/// [`Router::internal_calls`]:
/// * Helps resolve function calls to corresponding functions.
///
/// [`Router::modifier_calls`]:
/// * Helps resolve modifier calls to corresponding modifiers.
///
/// ----------
///
/// Given an instantiable base contract this helps resolve calls to public or external function
/// definitions in the base contract's hierarchy.
///
/// When control flows leaves the contract:
///
/// [`Router::external_calls`]:
/// * Helps resolve function calls to corresponding functions.
///
/// -----------
///
/// TODO:
/// Given a library resolve external calls made to public or external functions (delegate calls).
/// It's actually straightforward - Just directly return target suspect. But right now there is no
/// requirement for this. Therefore, I'lll pass on this one until I see a need.
///
/// -----------
///
/// NOTE: KEYS for the above are Node IDs of base contracts that are instantiable (non abstract)
///
/// -----------
#[derive(Debug)]
pub struct Router {
    /// resolves internal calls made to private, public and internal functions.
    pub internal_calls: HashMap<NodeID, ICRoutes>,

    /// resolved external calls made to public and external functions.
    pub external_calls: HashMap<NodeID, ECRoutes>,

    /// resolves internal calls made to modifiers.
    pub modifier_calls: HashMap<NodeID, MCRoutes>,
}

#[derive(Debug)]
pub struct ICRoutes {
    pub routes: BaseRoute<ICStartLookupRoute>,
}

#[derive(Debug)]
pub struct ECRoutes {
    pub routes: ECStartLookupRoute,
}

#[derive(Debug)]
pub struct MCRoutes {
    pub routes: BaseRoute<MCStartLookupRoute>,
}

// Starting Point Contract Definition -> Lookup
type BaseRoute<T> = HashMap<NodeID, T>;

// Function selectorish -> Function Definition Node ID
type ICStartLookupRoute = HashMap<String, NodeID>;

// Function selectorish -> Function Definition Node ID
type ECStartLookupRoute = HashMap<String, NodeID>;

// Modifier selectorish -> Modifier Definition Node ID
type MCStartLookupRoute = HashMap<String, NodeID>;

// Router interface
impl Router {
    pub fn build(context: &WorkspaceContext) -> Self {
        let internal_calls = context
            .deployable_contracts()
            .map(|contract| {
                let base_routes = build_ic_router_for_contract(context, contract);
                (contract.id, ICRoutes { routes: base_routes })
            })
            .collect();
        let external_calls = context
            .deployable_contracts()
            .map(|contract| {
                let base_routes = build_ec_router_for_contract(context, contract);
                (contract.id, ECRoutes { routes: base_routes })
            })
            .collect();
        let modifier_calls = context
            .deployable_contracts()
            .map(|contract| {
                let base_routes = build_mc_router_for_contract(context, contract);
                (contract.id, MCRoutes { routes: base_routes })
            })
            .collect();
        Self { internal_calls, external_calls, modifier_calls }
    }
    pub fn resolve_modifier_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        modifier_call: &'a ModifierInvocation,
    ) -> Option<&'a ModifierDefinition> {
        self._resolve_modifier_call(context, base_contract, modifier_call)
    }
    pub fn resolve_internal_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        self._resolve_internal_call(context, base_contract, func_call)
    }
    pub fn resolve_external_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        self._resolve_external_call(context, base_contract, func_call)
    }
}
