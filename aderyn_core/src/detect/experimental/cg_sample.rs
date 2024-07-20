#![allow(clippy::collapsible_match)]

#[cfg(test)]
mod callgraph_tests {
    use crate::context::{
        investigator::{
            StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
        },
        workspace_context::{ASTNode, WorkspaceContext},
    };

    #[test]
    fn test_callgraph_is_not_none() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );
        assert!(context.forward_callgraph.is_some());
        assert!(context.reverse_callgraph.is_some());
    }

    #[test]
    fn test_callgraph_downstream() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );

        let func_enter_tenth_floor1 = context
            .function_definitions()
            .into_iter()
            .find(|&x| x.name == "enterTenthFloor3".to_owned())
            .unwrap()
            .to_owned();

        let investigator = StandardInvestigator::for_specific_nodes(
            &context,
            &[&ASTNode::FunctionDefinition(func_enter_tenth_floor1)],
            StandardInvestigationStyle::Downstream,
        )
        .unwrap();

        let tracker = Tracker { context: &context };

        investigator.investigate(&context, &tracker).unwrap();
    }

    #[test]
    fn test_callgraph_upstream1() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/Tower.sol",
        );

        let pass_through_ninth_floor3 = context
            .modifier_definitions()
            .into_iter()
            .find(|&x| x.name == "passThroughNinthFloor3".to_owned())
            .unwrap()
            .to_owned();

        let investigator = StandardInvestigator::for_specific_nodes(
            &context,
            &[&ASTNode::ModifierDefinition(pass_through_ninth_floor3)],
            StandardInvestigationStyle::Upstream,
        )
        .unwrap();

        let tracker = Tracker { context: &context };

        investigator.investigate(&context, &tracker).unwrap();
    }

    struct Tracker<'a> {
        context: &'a WorkspaceContext,
    }

    impl StandardInvestigatorVisitor for Tracker<'_> {
        fn visit_entry_point(&self, node: &ASTNode) -> eyre::Result<()> {
            println!(
                "Entry point {:?}",
                self.context.get_node_sort_key_pure(node)
            );
            Ok(())
        }
        fn visit_downstream_function_definition(
            &self,
            node: &crate::ast::FunctionDefinition,
        ) -> eyre::Result<()> {
            println!("Downstream func {:?}", node.name);
            Ok(())
        }
        fn visit_downstream_modifier_definition(
            &self,
            node: &crate::ast::ModifierDefinition,
        ) -> eyre::Result<()> {
            println!("Downstream Modifier {:?}", node.name);
            Ok(())
        }
        fn visit_upstream_function_definition(
            &self,
            node: &crate::ast::FunctionDefinition,
        ) -> eyre::Result<()> {
            println!("Upstream func {:?}", node.name);
            Ok(())
        }
        fn visit_upstream_modifier_definition(
            &self,
            node: &crate::ast::ModifierDefinition,
        ) -> eyre::Result<()> {
            println!("Upstream Modifier {:?}", node.name);
            Ok(())
        }
    }
}
