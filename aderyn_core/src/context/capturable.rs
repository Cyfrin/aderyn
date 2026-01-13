use crate::{ast::*, context::workspace::WorkspaceContext};

/*
 * Define
 *  - Capturable Node
 *
 *  impl From<..> for Capturable
 *  impl Capturable
 */

macro_rules! make_capturable_node {
    (
        regular: $($type:ident),* $(,)?;
        yul: $($yul_type:ident),* $(,)?;
        yul_sourceless: $($yul_sourceless:ident),* $(,)?;
    ) => {

        define_ast_node! (
            $($type),*,
            $($yul_type),*,
            $($yul_sourceless),*,
            SourceUnit,
            ASTNode,
        );

        impl_conversion_for_ast_node! (
            $($type),*,
            $($yul_type),*,
            $($yul_sourceless),*,
            SourceUnit,
            ASTNode,
        );

        impl Capturable {
            pub fn make_key(&self, context: &WorkspaceContext) -> (String, usize, String) {
                match self {
                    Self::ASTNode(node) => context.get_node_sort_key(node),
                    Self::SourceUnit(n) => context.get_node_sort_key(&n.into()),
                    $(Self::$type(n) => context.get_node_sort_key(&n.into()),)*
                    $(Self::$yul_type(n) => context.get_node_sort_key(&n.into()),)*
                    $(Self::$yul_sourceless(n) => context.get_node_sort_key(&n.into()),)*
                }
            }
            pub fn id(&self) -> Option<NodeID> {
                match self {
                    Self::ASTNode(ast_node) => ast_node.id(),
                    Self::SourceUnit(source_unit_node) => Some(source_unit_node.id),
                    $(Self::$type(n) => Some(n.id),)*
                    $(Self::$yul_type(_) => None,)*
                    $(Self::$yul_sourceless(_) => None,)*
                }
            }
        }

    };
}

macro_rules! define_ast_node {
    ($($type:ident),* $(,)?) => {

        #[derive(Clone)]
        pub enum Capturable {
            $($type($type),)*
        }

    };
}

macro_rules! impl_conversion_for_ast_node {
    ($($type:ident),* $(,)?) => {
        // Regular nodes + Yul nodes + Yul sourceless nodes + SourceUnit + AST Node
        $(
            impl From<$type> for Capturable {
                fn from(value: $type) -> Self {
                    Self::$type(value)
                }
            }

            impl From<&$type> for Capturable {
                fn from(value: &$type) -> Self {
                    Self::$type(value.clone())
                }
            }
        )*
    };
}

with_node_types!(make_capturable_node);
