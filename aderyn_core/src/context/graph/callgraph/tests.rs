#![allow(clippy::collapsible_match)]

#[cfg(test)]
mod callgraph_test_functions {
    use crate::{
        ast::{FunctionDefinition, ModifierDefinition},
        context::{
            graph::{callgraph::CallGraphConsumer, traits::CallGraphVisitor},
            workspace::{ASTNode, WorkspaceContext},
        },
    };

    use crate::context::graph::callgraph::CallGraphDirection::{BothWays, Inward, Outward};

    fn get_function_by_name(context: &WorkspaceContext, name: &str) -> ASTNode {
        ASTNode::from(
            context.function_definitions().into_iter().find(|&x| x.name == *name).unwrap(),
        )
    }

    fn get_modifier_definition_by_name(context: &WorkspaceContext, name: &str) -> ASTNode {
        ASTNode::from(
            context.modifier_definitions().into_iter().find(|&x| x.name == *name).unwrap(),
        )
    }

    #[test]
    fn test_callgraph_is_not_none() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );
        assert!(context.inward_callgraph.is_some());
        assert!(context.outward_callgraph.is_some());
    }

    #[test]
    fn test_tower1_modifier_has_no_inward_legacy() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let visit_eighth_floor1 = get_function_by_name(&context, "visitEighthFloor1");

        let callgraph =
            CallGraphConsumer::get_legacy(&context, &[&visit_eighth_floor1], Inward).unwrap();

        let mut tracker = Tracker::new(&context);
        callgraph.accept(&context, &mut tracker).unwrap();

        assert!(tracker.inward_func_definitions_names.is_empty());
        assert!(tracker.inward_modifier_definitions_names.is_empty());
    }

    #[test]
    fn test_tower1_modifier_has_no_inward() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let visit_eighth_floor1 = get_function_by_name(&context, "visitEighthFloor1");

        let callgraphs = CallGraphConsumer::get(&context, &[&visit_eighth_floor1], Inward).unwrap();

        for callgraph in callgraphs {
            let mut tracker = Tracker::new(&context);
            callgraph.accept(&context, &mut tracker).unwrap();

            assert!(tracker.inward_func_definitions_names.is_empty());
            assert!(tracker.inward_modifier_definitions_names.is_empty());
        }
    }

    #[test]
    fn test_tower1_modifier_has_outward_legacy() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let visit_eighth_floor1 = get_function_by_name(&context, "visitEighthFloor1");

        let callgraph =
            CallGraphConsumer::get_legacy(&context, &[&visit_eighth_floor1], Outward).unwrap();

        let mut tracker = Tracker::new(&context);
        callgraph.accept(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_outward_modifiers_with_names(&["passThroughNinthFloor1"]));
        assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor1"]));
    }

    #[test]
    fn test_tower1_modifier_has_outward() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let visit_eighth_floor1 = get_function_by_name(&context, "visitEighthFloor1");

        let callgraphs =
            CallGraphConsumer::get(&context, &[&visit_eighth_floor1], Outward).unwrap();

        for callgraph in callgraphs {
            let mut tracker = Tracker::new(&context);
            callgraph.accept(&context, &mut tracker).unwrap();

            assert!(tracker.has_found_outward_modifiers_with_names(&["passThroughNinthFloor1"]));
            assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor1"]));
        }
    }

    #[test]
    fn test_tower2_modifier_has_both_outward_and_inward_legacy() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let pass_through_ninth_floor2 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor2");

        let callgraph =
            CallGraphConsumer::get_legacy(&context, &[&pass_through_ninth_floor2], BothWays)
                .unwrap();

        let mut tracker = Tracker::new(&context);
        callgraph.accept(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_inward_functions_with_names(&["visitEighthFloor2"]));
        assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor2"]));
    }

    #[test]
    fn test_tower2_modifier_has_both_outward_and_inward() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let pass_through_ninth_floor2 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor2");

        let callgraphs =
            CallGraphConsumer::get(&context, &[&pass_through_ninth_floor2], BothWays).unwrap();

        for callgraph in callgraphs {
            let mut tracker = Tracker::new(&context);
            callgraph.accept(&context, &mut tracker).unwrap();

            assert!(tracker.has_found_inward_functions_with_names(&["visitEighthFloor2"]));
            assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor2"]));
        }
    }

    #[test]
    fn test_tower3_modifier_has_both_outward_and_inward_legacy() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let pass_through_ninth_floor3 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor3");

        let callgraph =
            CallGraphConsumer::get_legacy(&context, &[&pass_through_ninth_floor3], BothWays)
                .unwrap();

        let mut tracker = Tracker::new(&context);
        callgraph.accept(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor3"]));
        assert!(tracker.has_found_inward_functions_with_names(&["visitEighthFloor3"]));
        assert!(tracker.has_not_found_any_outward_functions_with_name("visitSeventhFloor3"));
        assert!(tracker.has_found_outward_side_effect_functions_with_name(&["visitSeventhFloor3"]));
    }

    #[test]
    fn test_tower3_modifier_has_both_outward_and_inward() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let pass_through_ninth_floor3 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor3");

        let callgraphs =
            CallGraphConsumer::get(&context, &[&pass_through_ninth_floor3], BothWays).unwrap();

        for callgraph in callgraphs {
            let mut tracker = Tracker::new(&context);
            callgraph.accept(&context, &mut tracker).unwrap();

            assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor3"]));
            assert!(tracker.has_found_inward_functions_with_names(&["visitEighthFloor3"]));
            assert!(tracker.has_not_found_any_outward_functions_with_name("visitSeventhFloor3"));
            assert!(
                tracker.has_found_outward_side_effect_functions_with_name(&["visitSeventhFloor3"])
            );
        }
    }

    #[test]
    fn test_tower3_functions_has_outward_legacy() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let visit_eighth_floor3 = get_function_by_name(&context, "visitSeventhFloor3");

        let callgraph =
            CallGraphConsumer::get_legacy(&context, &[&visit_eighth_floor3], Outward).unwrap();

        let mut tracker = Tracker::new(&context);
        callgraph.accept(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor3"]));
    }

    #[test]
    fn test_tower3_functions_has_outward() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let visit_eighth_floor3 = get_function_by_name(&context, "visitSeventhFloor3");

        let callgraphs =
            CallGraphConsumer::get(&context, &[&visit_eighth_floor3], Outward).unwrap();

        for callgraph in callgraphs {
            let mut tracker = Tracker::new(&context);
            callgraph.accept(&context, &mut tracker).unwrap();

            assert!(tracker.has_found_outward_functions_with_names(&["enterTenthFloor3"]));
        }
    }

    #[test]
    fn test_tower4_functions_has_outward_and_inward_legacy() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let recurse = get_function_by_name(&context, "recurse");

        let callgraph = CallGraphConsumer::get_legacy(&context, &[&recurse], BothWays).unwrap();

        let mut tracker = Tracker::new(&context);
        callgraph.accept(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_outward_functions_with_names(&["recurse"]));
        assert!(tracker.has_found_inward_functions_with_names(&["recurse"]));
    }

    #[test]
    fn test_tower4_functions_has_outward_and_inward() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let recurse = get_function_by_name(&context, "recurse");

        let callgraphs = CallGraphConsumer::get(&context, &[&recurse], BothWays).unwrap();

        for callgraph in callgraphs {
            let mut tracker = Tracker::new(&context);
            callgraph.accept(&context, &mut tracker).unwrap();

            assert!(tracker.has_found_outward_functions_with_names(&["recurse"]));
            assert!(tracker.has_found_inward_functions_with_names(&["recurse"]));
        }
    }

    struct Tracker<'a> {
        context: &'a WorkspaceContext,
        entry_points: Vec<(String, usize, String)>,
        inward_func_definitions_names: Vec<String>,
        outward_func_definitions_names: Vec<String>,
        inward_modifier_definitions_names: Vec<String>,
        outward_modifier_definitions_names: Vec<String>,
        outward_side_effects_func_definitions_names: Vec<String>,
        outward_side_effects_modifier_definitions_names: Vec<String>,
    }

    impl Tracker<'_> {
        fn new(context: &WorkspaceContext) -> Tracker {
            Tracker {
                context,
                entry_points: vec![],
                inward_func_definitions_names: vec![],
                inward_modifier_definitions_names: vec![],
                outward_func_definitions_names: vec![],
                outward_modifier_definitions_names: vec![],
                outward_side_effects_func_definitions_names: vec![],
                outward_side_effects_modifier_definitions_names: vec![],
            }
        }

        // inward functions
        fn has_found_inward_functions_with_names(&self, name: &[&str]) -> bool {
            name.iter().all(|&n| self.inward_func_definitions_names.contains(&n.to_string()))
        }

        // outward functions
        fn has_found_outward_functions_with_names(&self, name: &[&str]) -> bool {
            name.iter().all(|&n| self.outward_func_definitions_names.contains(&n.to_string()))
        }

        fn has_not_found_any_outward_functions_with_name(&self, name: &str) -> bool {
            !self.outward_func_definitions_names.contains(&name.to_string())
        }

        // outward modifiers
        fn has_found_outward_modifiers_with_names(&self, name: &[&str]) -> bool {
            name.iter().all(|&n| self.outward_modifier_definitions_names.contains(&n.to_string()))
        }

        // outward side effects
        fn has_found_outward_side_effect_functions_with_name(&self, name: &[&str]) -> bool {
            name.iter()
                .all(|&n| self.outward_side_effects_func_definitions_names.contains(&n.to_string()))
        }
    }

    impl CallGraphVisitor for Tracker<'_> {
        fn visit_entry_point(&mut self, node: &ASTNode) -> eyre::Result<()> {
            self.entry_points.push(self.context.get_node_sort_key_pure(node));
            Ok(())
        }
        fn visit_inward_function_definition(
            &mut self,
            node: &crate::ast::FunctionDefinition,
        ) -> eyre::Result<()> {
            self.inward_func_definitions_names.push(node.name.to_string());
            Ok(())
        }
        fn visit_inward_modifier_definition(
            &mut self,
            node: &crate::ast::ModifierDefinition,
        ) -> eyre::Result<()> {
            self.inward_modifier_definitions_names.push(node.name.to_string());
            Ok(())
        }
        fn visit_outward_function_definition(
            &mut self,
            node: &crate::ast::FunctionDefinition,
        ) -> eyre::Result<()> {
            self.outward_func_definitions_names.push(node.name.to_string());
            Ok(())
        }
        fn visit_outward_modifier_definition(
            &mut self,
            node: &crate::ast::ModifierDefinition,
        ) -> eyre::Result<()> {
            self.outward_modifier_definitions_names.push(node.name.to_string());
            Ok(())
        }
        fn visit_outward_side_effect_function_definition(
            &mut self,
            node: &FunctionDefinition,
        ) -> eyre::Result<()> {
            self.outward_side_effects_func_definitions_names.push(node.name.to_string());
            Ok(())
        }
        fn visit_outward_side_effect_modifier_definition(
            &mut self,
            node: &ModifierDefinition,
        ) -> eyre::Result<()> {
            self.outward_side_effects_modifier_definitions_names.push(node.name.to_string());
            Ok(())
        }
    }
}
