use super::Router;
use crate::{
    ast::{
        ASTNode, ContractDefinition, ContractKind, Expression, FunctionCall, FunctionDefinition,
        FunctionKind, Identifier, NodeID, NodeType, Visibility,
    },
    context::{browser::GetClosestAncestorOfTypeX, workspace::WorkspaceContext},
};
use std::collections::{hash_map::Entry, HashMap};

impl Router {
    /// Returns Function Definition by attempting to resolve internal function calls given the base
    /// contract from which the call takes place.
    ///
    /// Goal -
    ///
    /// Pre-requisite: Check that the function is a legal internal call (doesn't leave the contract)
    ///
    /// 1. suspects (functions) that are `private` and `library` are returned directly as they
    ///    cannot be overridden
    /// 2. lookup through inheritance tree of base contract is performed to find relevant target -
    ///    if not found, suspect function is returned as a fallback mechanism
    ///
    /// Note - Not all styles of internal calls are resolved successfully at the moment. Lot of
    /// unknowns.
    pub(super) fn _resolve_internal_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        // do not resolve if it's not internal function call
        // very very very important check.
        if func_call.is_internal_call() != Some(true) {
            return None;
        }

        // check if it's illegal base contract type
        if !base_contract.is_deployable_contract() {
            return None;
        }

        let func = func_call.suspected_target_function(context)?;

        // check if it's illegal value - i.e function call that cannot be called from the base
        // contract must be discarded
        //
        // Sometimes it may be a free function so it's okay to not have a containing contract
        // definition.
        if let Some(ASTNode::ContractDefinition(caller_contract)) =
            func_call.closest_ancestor_of_type(context, NodeType::ContractDefinition)
        {
            if caller_contract.kind == ContractKind::Contract
                && !caller_contract.is_in(context, base_contract)
            {
                return None;
            } else if caller_contract.kind == ContractKind::Library {
                // If an internal function call happens from a library, suspect cannot be overridden
                // As of now, libraries do not have inheritance
                //
                // NOTE: for this case, we don't check that the internal library call can actually
                // happen and is trigger(able) by the base contract.
                return Some(func);
            }
        }

        if func.visibility == Visibility::Private {
            return Some(func);
        }

        // If an internal function call happens to a library, suspect cannot be overridden
        // As of now, libraries do not have inheritance
        if func.closest_ancestor_of_type(context, NodeType::ContractDefinition).is_some_and(|c| {
            matches!(
                c,
                ASTNode::ContractDefinition(ContractDefinition { kind: ContractKind::Library, .. })
            )
        }) {
            return Some(func);
        }

        self.perform_ic_lookup_through_inheritance_tree_and_fallback_to_suspect(
            context,
            base_contract,
            func_call,
        )
    }

    /// Lookup the internal function that will be invoked based on the base contract by matching
    /// patterns against function call sties. If lookup exhausts the overloaded methods, return the
    /// suspect.
    ///
    /// Goal -
    /// match the selectorish against the inheritance hierarchy if needed and resolve the function
    ///
    /// <.. Pattern matching ...>
    ///
    /// 1. regular call like `xyz()`:
    ///     - starting point = base contract
    /// 2. laidback super call `super.xyz()`:
    ///     - starting point = calling contrat's parent in the inheritance tree of base contract
    /// 3. explicit super call `Grandparent.xyz()`:
    ///     - starting point = Grandparent contract in the inheritance tree of the base contract
    ///
    /// Auxiliary function exists to
    ///  * provide selectorish
    ///  * act as fallback if lookup exhausts without a match (maybe it's a free function)
    ///  * free functions can be overridden, therefore lookup
    ///
    /// Note - Library calls are already resolved before calling this function.
    ///
    /// pattern matching is not exhaustive here. Look inside [`FunctionCall::is_internal_call`] and
    /// [`FunctionCall::suspected_target_function`] to ensure consistent logic.
    fn perform_ic_lookup_through_inheritance_tree_and_fallback_to_suspect<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        let aux_func = func_call.suspected_target_function(context)?;
        let selectorish = aux_func.selectorish();
        let base_index = self.internal_calls.get(&base_contract.id)?;

        let resolve = |starting_point: &ContractDefinition| -> Option<&FunctionDefinition> {
            let starting_point = starting_point.id;
            let lookup_index = base_index.routes.get(&starting_point)?;
            match lookup_index.get(&selectorish) {
                Some(func_id) => match context.nodes.get(func_id) {
                    Some(ASTNode::FunctionDefinition(func_def)) => Some(func_def),
                    _ => None,
                },
                // if not found in lookup fallback to aux function (suspect function)
                None => Some(aux_func),
            }
        };

        // direct calls must be start their lookup from the base_contract
        if let Expression::Identifier(_) = func_call.expression.as_ref() {
            return resolve(base_contract);
        }

        if let Expression::MemberAccess(member_access) = func_call.expression.as_ref() {
            if let Expression::Identifier(Identifier {
                name,
                referenced_declaration: Some(ref_id),
                ..
            }) = member_access.expression.as_ref()
            {
                // case - explicit super call
                // super calls must start their lookup from the calling contract's parent
                if name == "super" {
                    if let Some(ASTNode::ContractDefinition(calling_contract)) =
                        func_call.closest_ancestor_of_type(context, NodeType::ContractDefinition)
                    {
                        let next = calling_contract.next_in(context, base_contract)?;
                        return resolve(next);
                    }
                }
                // case - laidback super call
                // start lookup from the directly specified contract (dsc)
                else if let Some(ASTNode::ContractDefinition(called_contract)) =
                    context.nodes.get(ref_id)
                {
                    // safety check
                    if called_contract.is_in(context, base_contract) {
                        return resolve(called_contract);
                    }
                }
            }
        }
        None
    }
}

pub(super) fn build_ic_router_for_contract(
    context: &WorkspaceContext,
    base_contract: &ContractDefinition,
) -> HashMap<NodeID, HashMap<String, NodeID>> {
    let c3 = base_contract.c3(context).collect::<Vec<_>>();
    let mut base_routes = HashMap::new();
    for (idx, starting_point) in c3.iter().enumerate() {
        let mut routes = HashMap::new();
        for contract in c3.iter().skip(idx) {
            for func in contract.function_definitions() {
                if matches!(*func.kind(), FunctionKind::Function)
                    && matches!(func.visibility, Visibility::Internal | Visibility::Public)
                {
                    if let Entry::Vacant(e) = routes.entry(func.selectorish()) {
                        e.insert(func.id);
                    }
                }
            }
        }
        base_routes.insert(starting_point.id, routes);
    }
    base_routes
}
