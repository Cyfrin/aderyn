#![allow(clippy::collapsible_match)]

#[cfg(test)]
mod callgraph_tests {
    use crate::{
        ast::{FunctionDefinition, ModifierDefinition},
        context::{
            investigator::{StandardInvestigator, StandardInvestigatorVisitor},
            workspace_context::{ASTNode, WorkspaceContext},
        },
    };

    use serial_test::serial;

    fn get_function_by_name(context: &WorkspaceContext, name: &str) -> ASTNode {
        ASTNode::from(
            context
                .function_definitions()
                .into_iter()
                .find(|&x| x.name == *name)
                .unwrap(),
        )
    }

    fn get_modifier_definition_by_name(context: &WorkspaceContext, name: &str) -> ASTNode {
        ASTNode::from(
            context
                .modifier_definitions()
                .into_iter()
                .find(|&x| x.name == *name)
                .unwrap(),
        )
    }

    #[test]
    #[serial]
    fn test_callgraph_is_not_none() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );
        assert!(context.forward_callgraph.is_some());
        assert!(context.reverse_callgraph.is_some());
    }

    #[test]
    #[serial]
    fn test_tower1_modifier_has_no_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let visit_eighth_floor1 = get_function_by_name(&context, "visitEighthFloor1");

        let investigator = StandardInvestigator::new(&context, &[&visit_eighth_floor1]).unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.downstream_func_definitions_names.is_empty());
        assert!(tracker.downstream_modifier_definitions_names.is_empty());
    }

    #[test]
    #[serial]
    fn test_tower2_modifier_has_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let pass_through_ninth_floor2 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor2");

        let investigator =
            StandardInvestigator::new(&context, &[&pass_through_ninth_floor2]).unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_downstream_functions_with_names(&["visitEighthFloor2"]));
    }

    #[test]
    #[serial]
    fn test_tower3_modifier_has_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let pass_through_ninth_floor3 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor3");

        let investigator =
            StandardInvestigator::new(&context, &[&pass_through_ninth_floor3]).unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_downstream_functions_with_names(&["visitEighthFloor3"]));
    }

    #[test]
    #[serial]
    fn test_tower4_functions_has_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CallGraphTests.sol",
        );

        let recurse = get_function_by_name(&context, "recurse");

        let investigator = StandardInvestigator::new(&context, &[&recurse]).unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_downstream_functions_with_names(&["recurse"]));
    }

    struct Tracker<'a> {
        context: &'a WorkspaceContext,
        entry_points: Vec<(String, usize, String)>,
        downstream_func_definitions_names: Vec<String>,
        downstream_modifier_definitions_names: Vec<String>,
    }

    impl<'a> Tracker<'a> {
        fn new(context: &WorkspaceContext) -> Tracker {
            Tracker {
                context,
                entry_points: vec![],
                downstream_func_definitions_names: vec![],
                downstream_modifier_definitions_names: vec![],
            }
        }

        // downstream functions
        fn has_found_downstream_functions_with_names(&self, name: &[&str]) -> bool {
            name.iter().all(|&n| {
                self.downstream_func_definitions_names
                    .contains(&n.to_string())
            })
        }
    }

    impl StandardInvestigatorVisitor for Tracker<'_> {
        fn visit_entry_point(&mut self, node: &ASTNode) -> eyre::Result<()> {
            self.entry_points
                .push(self.context.get_node_sort_key_pure(node));
            Ok(())
        }
        fn visit_downstream_function_definition(
            &mut self,
            node: &crate::ast::FunctionDefinition,
        ) -> eyre::Result<()> {
            self.downstream_func_definitions_names
                .push(node.name.to_string());
            Ok(())
        }
        fn visit_downstream_modifier_definition(
            &mut self,
            node: &crate::ast::ModifierDefinition,
        ) -> eyre::Result<()> {
            self.downstream_modifier_definitions_names
                .push(node.name.to_string());
            Ok(())
        }
    }
}
