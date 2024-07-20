#![allow(clippy::collapsible_match)]

#[cfg(test)]
mod callgraph_tests {
    use crate::{
        ast::{FunctionDefinition, ModifierDefinition},
        context::{
            investigator::{
                StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
            },
            workspace_context::{ASTNode, WorkspaceContext},
        },
    };

    fn get_function_by_name(context: &WorkspaceContext, name: &str) -> FunctionDefinition {
        context
            .function_definitions()
            .into_iter()
            .find(|&x| x.name == name.to_string())
            .unwrap()
            .to_owned()
    }

    fn get_modifier_definition_by_name(
        context: &WorkspaceContext,
        name: &str,
    ) -> ModifierDefinition {
        context
            .modifier_definitions()
            .into_iter()
            .find(|&x| x.name == name.to_string())
            .unwrap()
            .to_owned()
    }

    #[test]
    fn test_callgraph_is_not_none() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );
        assert!(context.forward_callgraph.is_some());
        assert!(context.reverse_callgraph.is_some());
    }

    #[test]
    fn test_tower1_modifier_has_no_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );

        let visit_eighth_floor1 = get_function_by_name(&context, "visitEighthFloor1");

        let investigator = StandardInvestigator::new(
            &context,
            &[&ASTNode::FunctionDefinition(visit_eighth_floor1)],
            StandardInvestigationStyle::Downstream,
        )
        .unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.downstream_func_definitions_names.is_empty());
        assert!(tracker.downstream_modifier_definitions_names.is_empty());
    }

    #[test]
    fn test_tower1_modifier_has_upstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );

        let visit_eighth_floor1 = get_function_by_name(&context, "visitEighthFloor1");

        let investigator = StandardInvestigator::new(
            &context,
            &[&ASTNode::FunctionDefinition(visit_eighth_floor1)],
            StandardInvestigationStyle::Upstream,
        )
        .unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_upstream_modifiers_with_names(&["passThroughNinthFloor1"]));
        assert!(tracker.has_found_upstream_functions_with_names(&["enterTenthFloor1"]));
    }

    #[test]
    fn test_tower2_modifier_has_both_upstream_and_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );

        let pass_through_ninth_floor2 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor2");

        let investigator = StandardInvestigator::new(
            &context,
            &[&ASTNode::ModifierDefinition(pass_through_ninth_floor2)],
            StandardInvestigationStyle::BothWays,
        )
        .unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_downstream_functions_with_names(&["visitEighthFloor2"]));
        assert!(tracker.has_found_upstream_functions_with_names(&["enterTenthFloor2"]));
    }

    #[test]
    fn test_tower3_modifier_has_both_upstream_and_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );

        let pass_through_ninth_floor3 =
            get_modifier_definition_by_name(&context, "passThroughNinthFloor3");

        let investigator = StandardInvestigator::new(
            &context,
            &[&ASTNode::ModifierDefinition(pass_through_ninth_floor3)],
            StandardInvestigationStyle::BothWays,
        )
        .unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_upstream_functions_with_names(&["enterTenthFloor3"]));
        assert!(tracker.has_found_downstream_functions_with_names(&["visitEighthFloor3"]));
        assert!(tracker.has_not_found_any_upstream_functions_with_name("visitSeventhFloor3"));
    }

    #[test]
    fn test_tower3_functions_has_upstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );

        let visit_eighth_floor3 = get_function_by_name(&context, "visitSeventhFloor3");

        let investigator = StandardInvestigator::new(
            &context,
            &[&ASTNode::FunctionDefinition(visit_eighth_floor3)],
            StandardInvestigationStyle::Upstream,
        )
        .unwrap();

        let mut tracker = Tracker::new(&context);
        investigator.investigate(&context, &mut tracker).unwrap();

        assert!(tracker.has_found_upstream_functions_with_names(&["enterTenthFloor3"]));
    }

    struct Tracker<'a> {
        context: &'a WorkspaceContext,
        entry_points: Vec<(String, usize, String)>,
        downstream_func_definitions_names: Vec<String>,
        upstream_func_definitions_names: Vec<String>,
        downstream_modifier_definitions_names: Vec<String>,
        upstream_modifier_definitions_names: Vec<String>,
    }

    impl<'a> Tracker<'a> {
        fn new(context: &WorkspaceContext) -> Tracker {
            Tracker {
                context,
                entry_points: vec![],
                downstream_func_definitions_names: vec![],
                downstream_modifier_definitions_names: vec![],
                upstream_func_definitions_names: vec![],
                upstream_modifier_definitions_names: vec![],
            }
        }

        // downstream functions
        fn has_found_downstream_functions_with_names(&self, name: &[&str]) -> bool {
            name.iter().all(|&n| {
                self.downstream_func_definitions_names
                    .contains(&n.to_string())
            })
        }

        // upstream functions
        fn has_found_upstream_functions_with_names(&self, name: &[&str]) -> bool {
            name.iter().all(|&n| {
                self.upstream_func_definitions_names
                    .contains(&n.to_string())
            })
        }

        fn has_not_found_any_upstream_functions_with_name(&self, name: &str) -> bool {
            !self
                .upstream_func_definitions_names
                .contains(&name.to_string())
        }

        // upstream modifiers
        fn has_found_upstream_modifiers_with_names(&self, name: &[&str]) -> bool {
            name.iter().all(|&n| {
                self.upstream_modifier_definitions_names
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
        fn visit_upstream_function_definition(
            &mut self,
            node: &crate::ast::FunctionDefinition,
        ) -> eyre::Result<()> {
            self.upstream_func_definitions_names
                .push(node.name.to_string());
            Ok(())
        }
        fn visit_upstream_modifier_definition(
            &mut self,
            node: &crate::ast::ModifierDefinition,
        ) -> eyre::Result<()> {
            self.upstream_modifier_definitions_names
                .push(node.name.to_string());
            Ok(())
        }
    }
}
