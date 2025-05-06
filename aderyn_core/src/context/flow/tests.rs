#[cfg(test)]
mod control_flow_tests {
    use crate::{
        context::{
            flow::{
                visualizer::control_flow_tests::output_graph, Block, Cfg, CfgNode,
                CfgNodeDescriptor, CfgNodeId,
            },
            workspace::WorkspaceContext,
        },
        detect::test_utils::load_solidity_source_unit,
    };

    // Sample use of CFG
    impl Cfg {
        pub fn accept_block(&mut self, context: &WorkspaceContext, block: &Block) {
            let start = self.add_start_node();
            let end = self.add_end_node();
            let block = self.add_block_node(block);

            self.add_flow_edge(start, block);
            self.add_flow_edge(block, end);

            while let Some(reduction_candidate) = self.reduction_queue.pop_front() {
                self.reduce(context, reduction_candidate);
            }
        }
    }

    // Accept block (Pre calibration checks)
    #[test]

    fn simple_program_function1() {
        /*

        First example
        --------------
        Consider
        ../tests/contract-playground/src/control_flow/SimpleProgram.sol
        SimpleProgram : function1

        Deconstruct the function step by step until we have a graph with only
        Every function has a body Block so we start with the following graph and reduce it to primitives

        Step 1:

            Let 'a be the ID node the CfgNode(Block b)

            reduction_queue : [ 'a ]

            Sn(Block) -> CfgNode(Block b) 'a -> En(Block)

            Short form:
            Sn -> CfgStartNode
            En -> CfgEndNode

        Step 2:

            reduction_queue: [ ]

            Sn ->
                Sn -> CfgNode(VariableDeclarationStatement v) -> En ->
                Sn -> CfgNode(ExpressionStatement e) -> En ->
                Sn -> CfgNode(ExpressionStatement e) -> En ->
            En

        */

        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function1");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function1 not to be defined"));

        assert_eq!(cfg.nodes.len(), 7);

        assert!(matches!(
            cfg.nodes.get(&CfgNodeId(3)).unwrap(),
            CfgNode { id: _, nd: CfgNodeDescriptor::Start(_) }
        ));

        assert!(matches!(
            cfg.nodes.get(&CfgNodeId(4)).unwrap(),
            CfgNode { id: _, nd: CfgNodeDescriptor::End(_) }
        ));

        output_graph(&context, &cfg, "SimpleProgram_function1");
    }

    #[test]

    fn simple_program_function2() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function2");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function2 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function2");
        assert_eq!(cfg.nodes.len(), 14);
    }

    #[test]

    fn simple_program_function3() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function3");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function3 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function3");
        assert_eq!(cfg.nodes.len(), 12);
    }

    #[test]

    fn simple_program_function4() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function4");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function4 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function4");
        assert_eq!(cfg.nodes.len(), 48);
    }

    #[test]

    fn simple_program_function5() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function5");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function5 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function5");
        assert_eq!(cfg.nodes.len(), 25);
    }

    #[test]

    fn simple_program_function6() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function6");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function6 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function6");
        assert_eq!(cfg.nodes.len(), 31);
    }

    #[test]

    fn simple_program_function7() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function7");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function7 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function7");
        assert_eq!(cfg.nodes.len(), 22);
    }

    #[test]

    fn simple_program_function8() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function8");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function8 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function8");
        assert_eq!(cfg.nodes.len(), 48);
    }

    #[test]

    fn simple_program_function9() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function9");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function9 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function9");
        assert_eq!(cfg.nodes.len(), 15);
    }

    #[test]

    fn simple_program_function10() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function10");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function10 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function10");
        assert_eq!(cfg.nodes.len(), 9);
    }

    // Accept-Function-Body (Post calibration checks)

    #[test]

    fn simple_program_function11() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function11");
        let (cfg, _, _) = Cfg::from_function_body(&context, function).unwrap();

        output_graph(&context, &cfg, "SimpleProgram_function11");
        assert_eq!(cfg.nodes.len(), 26);
        assert_eq!(cfg.total_edges(), 27);
    }

    #[test]

    fn simple_program_function12() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function12");
        let (cfg, _, _) = Cfg::from_function_body(&context, function).unwrap();

        output_graph(&context, &cfg, "SimpleProgram_function12");
        assert_eq!(cfg.nodes.len(), 42);
        assert_eq!(cfg.total_edges(), 44);
    }

    #[test]

    fn simple_program_function13() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function13");
        let (cfg, _, _) = Cfg::from_function_body(&context, function).unwrap();

        output_graph(&context, &cfg, "SimpleProgram_function13");
        assert_eq!(cfg.nodes.len(), 36);
        assert_eq!(cfg.total_edges(), 38);
    }

    #[test]

    fn simple_program_function14() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function14");
        let (cfg, _, _) = Cfg::from_function_body(&context, function).unwrap();

        output_graph(&context, &cfg, "SimpleProgram_function14");
        assert_eq!(cfg.nodes.len(), 46);
        assert_eq!(cfg.total_edges(), 49);
    }

    #[test]

    fn simple_program_function15() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function15");
        let (cfg, _, _) = Cfg::from_function_body(&context, function).unwrap();

        output_graph(&context, &cfg, "SimpleProgram_function15");
        assert_eq!(cfg.nodes.len(), 70);
        assert_eq!(cfg.total_edges(), 75);
    }

    #[test]

    fn simple_program_function16() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function16");
        let (cfg, _, _) = Cfg::from_function_body(&context, function).unwrap();

        output_graph(&context, &cfg, "SimpleProgram_function16");
        assert_eq!(cfg.nodes.len(), 82);
        assert_eq!(cfg.total_edges(), 88);
    }
}
