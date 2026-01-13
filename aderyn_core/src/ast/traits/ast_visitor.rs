use crate::{
    ast::{macros::with_node_types, *},
    context::workspace::{NodeContext, WorkspaceContext},
};
use eyre::Result;

macro_rules! implement_for_workspace_context {
    (
        regular: $( $node:ident ),* $(,)*;
        yul: $( $yul_node:ident ),* $(,)*;
        yul_sourceless: $( $yul_sourceless_node:ident ),* $(,)*;
    ) => {
        implement_for_workspace_context! {
            regular: $( $node ),*;
            yul: $( $yul_node ),* , $( $yul_sourceless_node ),*;
        }
    };

    (
        regular: $( $node:ident ),* $(,)*;
        yul: $( $yul_node:ident ),* $(,)*;
    ) => {
        paste::paste! {

            impl ASTConstVisitor for WorkspaceContext {

                // Regular nodes
                $(
                    fn [<visit_ $node:snake>](&mut self, node: &$node) -> Result<bool> {
                        self.nodes
                            .insert(node.id, ASTNode::$node(node.clone()));
                        self.[<$node:snake s_context>].insert(
                            node.clone(),
                            NodeContext { source_unit_id: self.last_source_unit_id },
                        );
                        Ok(true)
                    }
                )*

                // Yul nodes (no ID)
                $(
                    fn [<visit_ $yul_node:snake>](&mut self, node: &$yul_node) -> Result<bool> {
                        self.[<$yul_node:snake s_context>].insert(
                            node.clone(),
                            NodeContext { source_unit_id: self.last_source_unit_id },
                        );
                        Ok(true)
                    }
                )*

                fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
                    self.nodes.insert(node.id, ASTNode::SourceUnit(node.clone()));
                    self.source_units_context.push(node.clone());
                    self.last_source_unit_id = node.id;
                    Ok(true)
                }

                fn visit_immediate_children(
                    &mut self,
                    node_id: NodeID,
                    node_children_ids: Vec<NodeID>,
                ) -> Result<()> {
                    for id in node_children_ids {
                        self.parent_link.insert(id, node_id);
                    }
                    Ok(())
                }

            }
        }
    };
}

macro_rules! define_ast_const_visitor {
    (
        regular: $( $node:ident ),* $(,)*;
        yul: $( $yul_node:ident ),* $(,)*;
        yul_sourceless: $( $yul_sourceless_node:ident ),* $(,)*;
    ) => {
        define_ast_const_visitor! {
            regular: $( $node ),*;
            yul: $( $yul_node ),* , $( $yul_sourceless_node ),*;
        }
    };

    (
        regular: $( $node:ident ),* $(,)*;
        yul: $( $yul_node:ident ),* $(,)*;
    ) => {
        paste::paste! {

            pub trait ASTConstVisitor {
                $(
                    fn [<visit_ $node:snake>](&mut self, node: &$node) -> Result<bool> {
                        self.visit_node(node)
                    }
                    fn [<end_visit_ $node:snake>](&mut self, node: &$node) -> Result<()> {
                        self.end_visit_node(node)
                    }
                )*

                $(
                    fn [<visit_ $yul_node:snake>](&mut self, node: &$yul_node) -> Result<bool> {
                        self.visit_node(node)
                    }
                    fn [<end_visit_ $yul_node:snake>](&mut self, node: &$yul_node) -> Result<()> {
                        self.end_visit_node(node)
                    }
                )*

                fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
                    self.visit_node(node)
                }
                fn end_visit_source_unit(&mut self, node: &SourceUnit) -> Result<()> {
                    self.end_visit_node(node)
                }

                fn visit_node(&mut self, _node: &impl Node) -> Result<bool> {
                    Ok(true)
                }
                fn end_visit_node(&mut self, _node: &impl Node) -> Result<()> {
                    Ok(())
                }

                fn visit_immediate_children(
                    &mut self,
                    _node_id: NodeID,
                    _node_children_ids: Vec<NodeID>,
                ) -> Result<()> {
                    Ok(())
                }

                fn visit_node_id(&mut self, _node_id: Option<NodeID>) -> Result<()> {
                    Ok(())
                }
            }
        }
    };
}

with_node_types!(define_ast_const_visitor);
with_node_types!(implement_for_workspace_context);
