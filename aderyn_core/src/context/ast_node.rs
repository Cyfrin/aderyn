use crate::ast::*;

/*
 * Define
 *  - ASTNode
 *
 *  impl From<..> for ASTNode
 *  impl Node for ASTNode
 *  impl ASTNode
 */

macro_rules! make_ast_node {
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
        );

        impl_conversion_for_ast_node! (
            $($type),*,
            $($yul_type),*,
            $($yul_sourceless),*,
            SourceUnit,
        );

        impl_node_for_ast_node!(
            $($type),*,
            $($yul_type),*,
            $($yul_sourceless),*,
            SourceUnit,
        );

        impl ASTNode {
            pub fn node_type(&self) -> NodeType {
                match self {
                    $(ASTNode::$type(_) => NodeType::$type,)*
                    $(ASTNode::$yul_type(_) => NodeType::$yul_type,)*
                    $(ASTNode::$yul_sourceless(_) => NodeType::$yul_sourceless,)*
                    ASTNode::SourceUnit(_) => NodeType::SourceUnit,
                }
            }
            pub fn id(&self) -> Option<NodeID> {
                match self {
                    $(ASTNode::$type(n) => Some(n.id),)*
                    $(ASTNode::$yul_type(_) => None,)*
                    $(ASTNode::$yul_sourceless(_) => None,)*
                    ASTNode::SourceUnit(n) => Some(n.id),
                }
            }


            // NOTE: YulAssignment and other sourceless yul nodes don't come with "src" field, so we can't
            // capture location. Therefore they are categorized as "yul_sourceless" in the with_node_types!
            // macro. Temporary workaround is to inspect higher level Yul constructs (such as YulFunctionCall,
            // YulIdentifier, YulLiteral, etc) and then search for trigger conditions inside those.
            // If we need to flag, then flag the higher level Yul construct.
            //
            // Hopefully later versions of Solc emit some kind of "src" field. At that point, you can
            // move YulAssignment and others from yul_sourceless to yul in macros.rs!
            pub fn src(&self) -> Option<&str> {
                match self {
                    $(ASTNode::$type(node) => Some(&node.src),)*
                    $(ASTNode::$yul_type(node) => Some(&node.src),)*
                    $(ASTNode::$yul_sourceless(_) => None,)*
                    ASTNode::SourceUnit(node) => Some(&node.src),
                }
            }
        }

    };
}

macro_rules! define_ast_node {
    ($($type:ident),* $(,)?) => {

        #[derive(Debug, Clone, PartialEq)]
        pub enum ASTNode {
            $($type($type),)*
        }

    };
}

macro_rules! impl_conversion_for_ast_node {
    ($($type:ident),* $(,)?) => {

        $(
            impl From<$type> for ASTNode {
                fn from(value: $type) -> Self {
                    ASTNode::$type(value)
                }
            }

            impl From<&$type> for ASTNode {
                fn from(value: &$type) -> Self {
                    ASTNode::$type(value.clone())
                }
            }
        )*

    };
}

macro_rules! impl_node_for_ast_node {
    ($($type:ident),* $(,)?) => {
        impl Node for ASTNode {
            fn accept(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                match self {
                    $(ASTNode::$type(n) => n.accept(visitor),)*
                }
            }
            fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                match self {
                    $(ASTNode::$type(n) => n.accept_metadata(visitor),)*
                }
            }
            fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                visitor.visit_node_id(self.id())?;
                Ok(())
            }
        }
    };
}

with_node_types!(make_ast_node);
