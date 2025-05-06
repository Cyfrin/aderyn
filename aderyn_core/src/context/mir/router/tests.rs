#[cfg(test)]
mod mir_router {
    use crate::{
        ast::ASTNode,
        context::{
            browser::ExtractFunctionCalls, mir::router::Router, workspace::WorkspaceContext,
        },
        test_utils::load_solidity_source_unit,
    };

    // Utility function to help debug router.
    #[allow(dead_code)]
    pub fn display(router: &Router, context: &WorkspaceContext) {
        println!("Internal calls");
        println!("==============");
        for (base_contract_id, ic) in &router.internal_calls {
            let Some(ASTNode::ContractDefinition(c)) = context.nodes.get(base_contract_id) else {
                eprintln!("Couldn't resolve contract with ID {}", base_contract_id);
                return;
            };
            println!("Base contract - {}", c.name);
            for (start_lookup, lookup) in &ic.routes {
                let Some(ASTNode::ContractDefinition(s)) = context.nodes.get(start_lookup) else {
                    eprintln!("Couldn't resolve contract with ID {}", start_lookup);
                    return;
                };
                println!("Start lookup - {}", s.name);
                for (func_selectorish, def_id) in lookup {
                    let Some(ASTNode::FunctionDefinition(f)) = context.nodes.get(def_id) else {
                        eprintln!("Couldn't resolve contract with ID {}", def_id);
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

    fn get_ic_router_ctx() -> (Router, WorkspaceContext) {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/router/InternalCalls.sol");

        (Router::build(&context), context)
    }

    fn get_mc_router_ctx() -> (Router, WorkspaceContext) {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/router/ModifierCalls.sol");

        (Router::build(&context), context)
    }

    #[test]
    pub fn resolves_internal_calls_3() {
        let (router, context) = get_ic_router_ctx();

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
    pub fn resolves_internal_calls_4() {
        let (router, context) = get_ic_router_ctx();

        let contract = context.find_contract_by_name("Basic4");
        let main = contract.find_function_by_name("main");
        let priv_func = contract.find_function_by_name("priv");

        let library = context.find_contract_by_name("Basic4Lib");
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

        let f0 = router.resolve_internal_call(&context, contract, &func_calls[0]);
        let f1 = router.resolve_internal_call(&context, contract, &func_calls[1]);
        let f2 = router.resolve_internal_call(&context, contract, &func_calls[2]);
        let f3 = router.resolve_internal_call(&context, contract, &func_calls[3]);
        let f4 = router.resolve_internal_call(&context, contract, &func_calls[4]);

        // Lib calls
        assert_eq!(f0.unwrap().id, lib_help1.id);
        assert_eq!(f1.unwrap().id, lib_help1.id);
        assert_eq!(f2.unwrap().id, lib_help1.id);
        assert_eq!(f3.unwrap().id, lib_help1.id);

        // Private funcs
        assert_eq!(f4.unwrap().id, priv_func.id);
    }

    #[test]
    pub fn resolves_internal_calls_5() {
        let (router, context) = get_ic_router_ctx();

        let free_func = context.find_free_function_by_name("free");
        let basic6_contract = context.find_contract_by_name("Basic6");
        let basic6_function = basic6_contract.find_function_by_name("main");
        let basic6_function_calls = ExtractFunctionCalls::from(basic6_function).extracted;

        let a = router
            .resolve_internal_call(&context, basic6_contract, &basic6_function_calls[0])
            .unwrap();
        assert_eq!(a.id, free_func.id);

        let basic7_contract = context.find_contract_by_name("Basic7");
        let basic7_function = basic7_contract.find_function_by_name("main");
        let basic7_function_calls = ExtractFunctionCalls::from(basic7_function).extracted;

        let b = router
            .resolve_internal_call(&context, basic7_contract, &basic7_function_calls[0])
            .unwrap();
        assert_eq!(b.id, free_func.id);

        let basic8_contract = context.find_contract_by_name("Basic8");
        let basic8_function = basic8_contract.find_function_by_name("main");
        let basic8_free = basic8_contract.find_function_by_name("free");
        let basic8_function_calls = ExtractFunctionCalls::from(basic8_function).extracted;

        let c = router
            .resolve_internal_call(&context, basic8_contract, &basic8_function_calls[0])
            .unwrap();
        assert_eq!(c.id, basic8_free.id);

        let basic9_contract = context.find_contract_by_name("Basic9");
        let basic9_function = basic9_contract.find_function_by_name("help");
        let basic9_function_calls = ExtractFunctionCalls::from(basic9_function).extracted;

        let d = router
            .resolve_internal_call(&context, basic9_contract, &basic9_function_calls[0])
            .unwrap();
        assert_eq!(d.id, basic8_free.id);
    }

    #[test]
    pub fn resolve_modifier_calls_1() {
        let (router, context) = get_mc_router_ctx();

        let a_contract = context.find_contract_by_name("A");
        let b_contract = context.find_contract_by_name("B");
        let c_contract = context.find_contract_by_name("C");
        let d_library = context.find_contract_by_name("D");

        let a_func = a_contract.find_function_by_name("geez");
        let b_func = b_contract.find_function_by_name("tree");
        let c_func = c_contract.find_function_by_name("main");
        let show_func = d_library.find_function_by_name("show");

        let a_modifier_call = &a_func.modifiers[0];
        let b_modifier_call = &b_func.modifiers[0];
        let c_modifier_call_1 = &c_func.modifiers[0];
        let c_modifier_call_2 = &c_func.modifiers[1];
        let show_modifier = &show_func.modifiers[0];

        let a = router.resolve_modifier_call(&context, b_contract, a_modifier_call).unwrap();
        assert_eq!(a.id, b_contract.find_modifier_by_name("modify").id);

        let b = router.resolve_modifier_call(&context, c_contract, c_modifier_call_1).unwrap();
        assert_eq!(b.id, b_contract.find_modifier_by_name("modify").id);

        let c = router.resolve_modifier_call(&context, c_contract, c_modifier_call_2).unwrap();
        assert_eq!(c.id, c_contract.find_modifier_by_name("modify").id);

        let d = router.resolve_modifier_call(&context, b_contract, b_modifier_call).unwrap();
        assert_eq!(d.id, b_contract.find_modifier_by_name("modify").id);

        let e = router.resolve_modifier_call(&context, c_contract, show_modifier).unwrap();
        assert_eq!(e.id, d_library.find_modifier_by_name("modify").id);
    }

    #[test]
    pub fn resolves_internal_from_library_calls() {
        let (router, context) = get_ic_router_ctx();

        let basic4_contract = context.find_contract_by_name("Basic4");
        let basic4_lib = context.find_contract_by_name("Basic4Lib");
        let basic4_aux_lib = context.find_contract_by_name("Basic4AuxLib");
        let help1_func = basic4_lib.find_function_by_name("help1");
        let ext2_func = basic4_lib.find_function_by_name("ext2");
        let aux2 = basic4_aux_lib.find_function_by_name("aux2");
        let func_calls = ExtractFunctionCalls::from(help1_func).extracted;

        assert!(func_calls[0].is_internal_call().unwrap());
        assert!(!func_calls[1].is_internal_call().unwrap());
        assert!(func_calls[2].is_internal_call().unwrap());

        let a = router.resolve_internal_call(&context, basic4_contract, &func_calls[0]).unwrap();
        assert_eq!(a.id, ext2_func.id);

        let b = router.resolve_internal_call(&context, basic4_contract, &func_calls[2]).unwrap();
        assert_eq!(b.id, aux2.id);

        // external calls return none
        let c = router.resolve_internal_call(&context, basic4_contract, &func_calls[1]);
        assert!(c.is_none());
    }
}
