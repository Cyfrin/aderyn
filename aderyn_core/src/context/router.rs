//! Router
//!
//! Function router.

mod external_calls;
mod internal_calls;
mod modifier_calls;
mod tests;

use external_calls::build_ec_router_for_contract;
use internal_calls::build_ic_router_for_contract;
use modifier_calls::build_mc_router_for_contract;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

// Function selector -> ECDest
type ECStartLookupRoute = HashMap<String, ECDest>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ECDest {
    PseduoExtFn(NodeID), // Public State Variable Declaration Id (getter method)
    RealExtFn(NodeID),   // External function Id
    PublicFn(NodeID),    // Public function Id
    Fallback(NodeID),    // Fallback function Id
    Receive(NodeID),     // Receive function Id
}

// Modifier selectorish -> Modifier Definition Node ID
type MCStartLookupRoute = HashMap<String, NodeID>;

// Router interface
impl Router {
    pub fn build(context: &WorkspaceContext) -> Self {
        let route_groups = context
            .par_deployable_contracts()
            .into_par_iter()
            .map(|contract| {
                let contract_id = contract.id;
                let ic_routes = {
                    let base_routes = build_ic_router_for_contract(context, contract);
                    ICRoutes { routes: base_routes }
                };
                let ec_routes = {
                    let base_routes = build_ec_router_for_contract(context, contract);
                    ECRoutes { routes: base_routes }
                };
                let mc_routes = {
                    let base_routes = build_mc_router_for_contract(context, contract);
                    MCRoutes { routes: base_routes }
                };
                (contract_id, ic_routes, ec_routes, mc_routes)
            })
            .collect::<Vec<_>>();

        let mut internal_calls = HashMap::new();
        let mut external_calls = HashMap::new();
        let mut modifier_calls = HashMap::new();

        for routes in route_groups {
            internal_calls.insert(routes.0, routes.1);
            external_calls.insert(routes.0, routes.2);
            modifier_calls.insert(routes.0, routes.3);
        }

        Self { internal_calls, external_calls, modifier_calls }
    }
}

impl WorkspaceContext {
    pub fn entrypoint_functions<'a>(
        &'a self,
        contract: &'a ContractDefinition,
    ) -> Option<Vec<&'a FunctionDefinition>> {
        let router = self.router.as_ref()?;
        let base = router.external_calls.get(&contract.id)?;
        Some(
            base.routes
                .values()
                .flat_map(|r| match r {
                    ECDest::PseduoExtFn(_) => None,
                    ECDest::RealExtFn(id)
                    | ECDest::PublicFn(id)
                    | ECDest::Fallback(id)
                    | ECDest::Receive(id) => {
                        if let Some(ASTNode::FunctionDefinition(func)) = self.nodes.get(id) {
                            return Some(func);
                        }
                        None
                    }
                })
                .collect(),
        )
    }

    pub fn resolve_modifier_call<'a>(
        &'a self,
        base_contract: &'a ContractDefinition,
        modifier_call: &'a ModifierInvocation,
    ) -> Option<&'a ModifierDefinition> {
        let router = self.router.as_ref()?;
        router._resolve_modifier_call(self, base_contract, modifier_call)
    }

    pub fn resolve_internal_call<'a>(
        &'a self,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        let router = self.router.as_ref()?;
        router._resolve_internal_call(self, base_contract, func_call)
    }

    pub fn resolve_external_call<'a>(
        &'a self,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<ECDest> {
        let router = self.router.as_ref()?;
        router._resolve_external_call(self, base_contract, func_call)
    }

    pub fn resolve_fallback_function<'a>(
        &'a self,
        base_contract: &'a ContractDefinition,
    ) -> Option<&'a FunctionDefinition> {
        let router = self.router.as_ref()?;
        router._resolve_fallback_function(self, base_contract)
    }

    pub fn resolve_receive_function<'a>(
        &'a self,
        base_contract: &'a ContractDefinition,
    ) -> Option<&'a FunctionDefinition> {
        let router = self.router.as_ref()?;
        router._resolve_receive_function(self, base_contract)
    }

    pub fn resolve_function_selector(
        &self,
        base_contract: &ContractDefinition,
        selector: impl AsRef<str>,
    ) -> Option<ECDest> {
        let router = self.router.as_ref()?;
        router._resolve_function_selector(base_contract, selector)
    }
}
