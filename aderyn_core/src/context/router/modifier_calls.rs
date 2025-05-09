use super::Router;
use crate::{
    ast::{
        ASTNode, ContractDefinition, ContractKind, IdentifierOrIdentifierPath, ModifierDefinition,
        ModifierInvocation, NodeID, NodeType,
    },
    context::{browser::GetClosestAncestorOfTypeX, workspace::WorkspaceContext},
};
use std::collections::{hash_map::Entry, HashMap};

impl Router {
    pub(super) fn _resolve_modifier_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        modifier_call: &'a ModifierInvocation,
    ) -> Option<&'a ModifierDefinition> {
        // check if it's illegal base contract type
        if !base_contract.is_deployable_contract() {
            return None;
        }

        // check if it's illegal value - i.e function call that cannot be called from the base
        // contract must be discarded
        if let Some(ASTNode::ContractDefinition(caller_contract)) =
            modifier_call.closest_ancestor_of_type(context, NodeType::ContractDefinition)
        {
            if caller_contract.kind == ContractKind::Contract
                && !caller_contract.is_in(context, base_contract)
            {
                return None;
            } else if caller_contract.kind == ContractKind::Library {
                // If an internal modifier call happens from a library, suspect cannot be overridden
                // As of now, libraries do not have inheritance
                //
                // NOTE: for this case, we don't check that the modifier call from library can
                // actually happen and is trigger(able) by the base contract.
                let aux_modifier = modifier_call.suspected_target_modifier(context)?;
                return Some(aux_modifier);
            }
        }

        self.perform_mc_lookup_through_inheritance_tree_and_fallback_to_suspect(
            context,
            base_contract,
            modifier_call,
        )
    }

    fn perform_mc_lookup_through_inheritance_tree_and_fallback_to_suspect<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        modifier_call: &'a ModifierInvocation,
    ) -> Option<&'a ModifierDefinition> {
        let aux_modifier = modifier_call.suspected_target_modifier(context)?;
        let selectorish = aux_modifier.selectorish();
        let base_index = self.modifier_calls.get(&base_contract.id)?;

        let resolve = |starting_point: &ContractDefinition| -> Option<&ModifierDefinition> {
            let starting_point = starting_point.id;
            let lookup_index = base_index.routes.get(&starting_point)?;
            match lookup_index.get(&selectorish) {
                Some(modifier_id) => match context.nodes.get(modifier_id) {
                    Some(ASTNode::ModifierDefinition(modifier_def)) => Some(modifier_def),
                    _ => None,
                },
                // if not found in lookup fallback to aux function (suspect function)
                None => Some(aux_modifier),
            }
        };

        // TODO: Investigate when other enumeration can be triggered.
        if let IdentifierOrIdentifierPath::IdentifierPath(p) = &modifier_call.modifier_name {
            // Ex: `B.modify` is a full path so then the suspected target must be right.
            if p.name.contains('.') {
                return Some(aux_modifier);
            }
            // Ex: `modify`
            return resolve(base_contract);
        }

        None
    }
}

pub(super) fn build_mc_router_for_contract(
    context: &WorkspaceContext,
    base_contract: &ContractDefinition,
) -> HashMap<NodeID, HashMap<String, NodeID>> {
    let c3 = base_contract.c3(context).collect::<Vec<_>>();
    let mut base_routes = HashMap::new();
    for (idx, starting_point) in c3.iter().enumerate() {
        let mut routes = HashMap::new();
        for contract in c3.iter().skip(idx) {
            for modifier in contract.modifier_definitions() {
                if let Entry::Vacant(e) = routes.entry(modifier.selectorish()) {
                    e.insert(modifier.id);
                }
            }
        }
        base_routes.insert(starting_point.id, routes);
    }
    base_routes
}
