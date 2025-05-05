//! Router
//!
//! Function router.
//!
//! Currently it is only used for internal calls.

use crate::{
    ast::{
        ASTNode, ContractDefinition, ContractKind, Expression, FunctionCall, FunctionDefinition,
        Identifier, NodeID, NodeType, Visibility,
    },
    context::{browser::GetClosestAncestorOfTypeX, workspace_context::WorkspaceContext},
};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
pub struct Router {
    /// For instantiable contracts only (non abstract)
    /// Does not contain private and external call routes
    /// KEY => Node ID of base contract
    pub internal_calls: HashMap<NodeID, ICRoutes>,
}

#[derive(Debug)]
pub struct ICRoutes {
    pub routes: BaseRoute,
}

// Starting Point Contract Definition -> Lookup
type BaseRoute = HashMap<NodeID, StartLookupRoute>;

// Function selectorish -> Function Definition Node ID
type StartLookupRoute = HashMap<String, NodeID>;

// Router interface
impl Router {
    pub fn build(context: &WorkspaceContext) -> Self {
        let internal_calls = context
            .deployable_contracts()
            .into_iter()
            .map(|contract| {
                let base_routes = build_ic_router_for_contract(context, contract);
                (contract.id, ICRoutes { routes: base_routes })
            })
            .collect();
        Self { internal_calls }
    }
    /// Returns Function Definition by attempting to resolve internal function calls given the base
    /// contract from which the call takes place.
    ///
    /// Goal -
    ///
    /// Pre-requisite: Check that the function is a legal internal call (doesn't leave the contract)
    ///
    /// 1. suspects (functions) that are `private` and `library` are returned directly as they
    ///    cannot be overriden
    /// 2. lookup through inheritance tree of base contract is performed to find relevant target -
    ///    if not found, suspect function is returned as a fallback mechanism
    ///
    /// Note - Not all styles of internal calls are resolved successfully at the moment. Lot of
    /// unknowns.
    pub fn resolve_internal_call<'a>(
        &self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
        func_call: &'a FunctionCall,
    ) -> Option<&'a FunctionDefinition> {
        // do not resovle if it's not internal function call
        if func_call.is_internal_call() != Some(true) {
            return None;
        }

        // check if it's illegal value - i.e function call that cannot be called from the base
        // contract must be discarded
        if let Some(ASTNode::ContractDefinition(caller_contract)) =
            func_call.closest_ancestor_of_type(context, NodeType::ContractDefinition)
        {
            if !caller_contract.is_in(&context, base_contract) {
                return None;
            }
        }

        // TODO: check if it's illegal base contract (deployable condition)
        // TODO: write test for internal function

        let func = func_call.suspected_target_function(context)?;

        if func.visibility == Visibility::Private {
            return Some(func);
        }

        if func.closest_ancestor_of_type(context, NodeType::ContractDefinition).is_some_and(|c| {
            matches!(
                c,
                ASTNode::ContractDefinition(ContractDefinition { kind: ContractKind::Library, .. })
            )
        }) {
            return Some(func);
        }

        return self.perform_ic_lookup_through_inheritance_tree_and_fallback_to_suspect(
            context,
            base_contract,
            func_call,
        );
    }

    /// Lookup the internal function that will be invoked based on the base contract by matching
    /// patterns agains function call sties. If lookup exhausts the overloaded methods, return the
    /// suspect.
    ///
    /// Goal -
    /// match the selectorish against the inheritance hirearchy if needed and resolve the function
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
    ///  * free functions can be overriden, therefore lookup
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

        // direct calls must be strat their lookup from the base_contract
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

// Router Utils

fn build_ic_router_for_contract(
    context: &WorkspaceContext,
    base_contract: &ContractDefinition,
) -> HashMap<NodeID, HashMap<String, NodeID>> {
    let c3 = base_contract.c3(context).collect::<Vec<_>>();
    let mut base_routes = HashMap::new();
    for (idx, starting_point) in c3.iter().enumerate() {
        let mut routes = HashMap::new();
        for contract in c3.iter().skip(idx) {
            for func in contract.function_definitions() {
                if matches!(func.visibility, Visibility::Internal | Visibility::Public) {
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

#[cfg(test)]
mod mir_router {
    use crate::{
        ast::ASTNode,
        context::{browser::ExtractFunctionCalls, workspace_context::WorkspaceContext},
        test_utils::load_solidity_source_unit,
    };

    use super::Router;

    // Utility function to help debug router.
    #[allow(dead_code)]
    pub fn display(router: &Router, context: &WorkspaceContext) {
        println!("Internal calls");
        println!("==============");
        for (base_contract_id, ic) in &router.internal_calls {
            let Some(ASTNode::ContractDefinition(c)) = context.nodes.get(&base_contract_id) else {
                eprintln!("Couldn't resovle contract with ID {}", base_contract_id);
                return;
            };
            println!("Base contract - {}", c.name);
            for (start_lookup, lookup) in &ic.routes {
                let Some(ASTNode::ContractDefinition(s)) = context.nodes.get(start_lookup) else {
                    eprintln!("Couldn't resovle contract with ID {}", start_lookup);
                    return;
                };
                println!("Start lookup - {}", s.name);
                for (func_selectorish, def_id) in lookup {
                    let Some(ASTNode::FunctionDefinition(f)) = context.nodes.get(def_id) else {
                        eprintln!("Couldn't resovle contract with ID {}", def_id);
                        return;
                    };
                    println!(
                        "{} - {} - {:?}",
                        func_selectorish,
                        f.name,
                        context.get_node_sort_key_from_capturable(&f.clone().into())
                    );
                }
            }
            println!("-------------");
        }
    }

    fn get_router_ctx() -> (Router, WorkspaceContext) {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/router/InternalCalls.sol");

        (Router::build(&context), context)
    }

    #[test]
    pub fn resolves_calls_3() {
        let (router, context) = get_router_ctx();

        let basic3_top_contract = context.find_contract_by_name("Basic3Top");
        let basic3_right_contract = context.find_contract_by_name("Basic3Right");
        let basic3_left_contract = context.find_contract_by_name("Basic3Left");
        let basic3_down2_contract = context.find_contract_by_name("Basic3Down2");
        let basic2_contract = context.find_contract_by_name("Basic2");
        let basic2_child_contract = context.find_contract_by_name("Basic2Child");

        let basic3_top_function = basic3_top_contract.find_function_by_name("help");
        let basic3_top_live = basic3_top_contract.find_function_by_name("live");
        let basic3_left_function = basic3_left_contract.find_function_by_name("help");
        let basic2_child_gcall_function = basic2_child_contract.find_function_by_name("gcall");
        let basic2_g_function = basic2_contract.find_function_by_name("g");
        let basic2_child_g_function = basic2_child_contract.find_function_by_name("g");
        let basic3_right_function = basic3_right_contract.find_function_by_name("help");
        let basic3_down2_function = basic3_down2_contract.find_function_by_name("help");
        let basic3_down2_live = basic3_down2_contract.find_function_by_name("live");

        let basic3_top_function_calls = ExtractFunctionCalls::from(basic3_top_function).extracted;
        let basic3_right_function_calls =
            ExtractFunctionCalls::from(basic3_right_function).extracted;
        let basic3_left_function_calls = ExtractFunctionCalls::from(basic3_left_function).extracted;
        let basic3_down2_function_calls =
            ExtractFunctionCalls::from(basic3_down2_function).extracted;
        let basic2_child_gcall_function_calls =
            ExtractFunctionCalls::from(basic2_child_gcall_function).extracted;

        let a = router
            .resolve_internal_call(&context, basic3_down2_contract, &basic3_down2_function_calls[0])
            .unwrap();
        assert_eq!(a.id, basic3_right_function.id);

        let b = router
            .resolve_internal_call(&context, basic3_down2_contract, &basic3_top_function_calls[0])
            .unwrap();
        assert_eq!(b.id, basic3_down2_live.id);

        let c = router
            .resolve_internal_call(&context, basic3_down2_contract, &basic3_down2_function_calls[1])
            .unwrap();
        assert_eq!(c.id, basic3_top_live.id);

        let d = router
            .resolve_internal_call(&context, basic3_down2_contract, &basic3_down2_function_calls[2])
            .unwrap();
        assert_eq!(d.id, basic3_left_function.id);

        let e = router
            .resolve_internal_call(&context, basic3_down2_contract, &basic3_right_function_calls[0])
            .unwrap();
        assert_eq!(e.id, basic3_left_function.id);

        let f = router
            .resolve_internal_call(&context, basic3_down2_contract, &basic3_left_function_calls[0])
            .unwrap();
        assert_eq!(f.id, basic3_top_function.id);

        let g = router
            .resolve_internal_call(&context, basic3_right_contract, &basic3_right_function_calls[0])
            .unwrap();
        assert_eq!(g.id, basic3_top_function.id);

        assert_eq!(basic2_child_g_function.selectorish(), basic2_g_function.selectorish());
        let h = router
            .resolve_internal_call(
                &context,
                basic2_child_contract,
                &basic2_child_gcall_function_calls[1],
            )
            .unwrap();
        assert_eq!(h.id, basic2_child_g_function.id);
    }

    #[test]
    pub fn resolves_calls_4() {
        let (router, ctx) = get_router_ctx();

        let contract = ctx.find_contract_by_name("Basic4");
        let main = contract.find_function_by_name("main");
        let priv_func = contract.find_function_by_name("priv");

        let library = ctx.find_contract_by_name("Basic4Lib");
        let lib_help1 = library.find_function_by_name("help1");

        let func_calls = ExtractFunctionCalls::from(main).extracted;

        // internal calls
        assert_eq!(func_calls[0].is_internal_call(), Some(true));
        assert_eq!(func_calls[1].is_internal_call(), Some(true));
        assert_eq!(func_calls[2].is_internal_call(), Some(true));
        assert_eq!(func_calls[3].is_internal_call(), Some(true));
        assert_eq!(func_calls[4].is_internal_call(), Some(true));

        // external calls
        assert_eq!(func_calls[5].is_internal_call(), Some(false));
        assert_eq!(func_calls[6].is_internal_call(), Some(false));
        assert_eq!(func_calls[7].is_internal_call(), Some(false));

        let f0 = router.resolve_internal_call(&ctx, contract, &func_calls[0]);
        let f1 = router.resolve_internal_call(&ctx, contract, &func_calls[1]);
        let f2 = router.resolve_internal_call(&ctx, contract, &func_calls[2]);
        let f3 = router.resolve_internal_call(&ctx, contract, &func_calls[3]);
        let f4 = router.resolve_internal_call(&ctx, contract, &func_calls[4]);

        // Lib calls
        assert_eq!(f0.unwrap().id, lib_help1.id);
        assert_eq!(f1.unwrap().id, lib_help1.id);
        assert_eq!(f2.unwrap().id, lib_help1.id);
        assert_eq!(f3.unwrap().id, lib_help1.id);

        // Private funcs
        assert_eq!(f4.unwrap().id, priv_func.id);
    }
}
