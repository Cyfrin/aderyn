#![allow(unused_imports, unused_macros)]

/// Macro that expands to a struct with common AST node fields.
macro_rules! ast_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub id: $crate::ast::NodeID,
            pub src: String,
            $(
                $(#[$field_meta])*
                pub $field: $ty
            ),*
        }
    };
}

macro_rules! ast_node_no_partial_eq {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, Clone, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub id: $crate::ast::NodeID,
            pub src: String,
            $(
                $(#[$field_meta])*
                pub $field: $ty
            ),*
        }
    };
}

/// A macro that expands to a struct with common expression node fields.
macro_rules! expr_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)*
        }
    ) => {
        ast_node!(
            $(#[$struct_meta])*
            struct $name {
                argument_types: Option<Vec<TypeDescriptions>>,
                #[serde(default)]
                is_constant: bool,
                #[serde(default)]
                is_l_value: bool,
                #[serde(default)]
                is_pure: bool,
                #[serde(default)]
                l_value_requested: bool,
                type_descriptions: TypeDescriptions,
                $(
                    $(#[$field_meta])*
                    $field: $ty
                ),*
            }
        );
    }
}

/// A macro that expands to an enum where each variant also contains a struct of the same name.
///
/// The inner value of each variant is boxed since AST types are inherently recursive.
macro_rules! node_group {
    ($group:ident; $( $name:ident ),* $(,)*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
        #[serde(tag = "nodeType")]
        #[allow(clippy::large_enum_variant)]
        pub enum $group {
            $(
                $name($name),
            )*
        }
    };
}

macro_rules! stmt_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)*
        }
    ) => {
        ast_node!(
            $(#[$struct_meta])*
            struct $name {
                documentation: Option<String>,
                $(
                    $(#[$field_meta])*
                    $field: $ty
                ),*
            }
        );
    }
}

macro_rules! generate_ast_methods {
    (
        regular: $( $type:ident ),* $(,)*;
        yul: $( $yul_type:ident ),* $(,)*;
    ) => {

        #[derive(Debug, Clone, PartialEq)]
        pub enum ASTNode {
            $($type($type),)*
            $($yul_type($yul_type),)*
        }

        // Regular nodes
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

        // Yul nodes
        $(
            impl From<$yul_type> for ASTNode {
                fn from(value: $yul_type) -> Self {
                    ASTNode::$yul_type(value)
                }
            }

            impl From<&$yul_type> for ASTNode {
                fn from(value: &$yul_type) -> Self {
                    ASTNode::$yul_type(value.clone())
                }
            }
        )*

        impl ASTNode {
            pub fn node_type(&self) -> NodeType {
                match self {
                    $(ASTNode::$type(_) => NodeType::$type,)*
                    $(ASTNode::$yul_type(_) => NodeType::$yul_type,)*
                }
            }
            pub fn id(&self) -> Option<NodeID> {
                match self {
                    $(ASTNode::$type(n) => Some(n.id),)*
                    $(ASTNode::$yul_type(_) => None,)*
                }
            }
        }

        impl Node for ASTNode {
            fn accept(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                match self {
                    $(ASTNode::$type(n) => n.accept(visitor),)*
                    $(ASTNode::$yul_type(n) => n.accept(visitor),)*
                }
            }
            fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                match self {
                    $(ASTNode::$type(n) => n.accept_metadata(visitor),)*
                    $(ASTNode::$yul_type(n) => n.accept_metadata(visitor),)*
                }
            }
            fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
                visitor.visit_node_id(self.id())?;
                Ok(())
            }
        }

        impl ASTNode {
            pub fn src(&self) -> Option<&str> {
                match self {
                    $(ASTNode::$type(node) => Some(&node.src),)*
                    $(ASTNode::$yul_type(node) => Some(&node.src),)*
                }
            }
        }

    };
}

macro_rules! accept_id {
    () => {
        fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
            visitor.visit_node_id(Some(self.id))?;
            Ok(())
        }
    };
}

pub(crate) use accept_id;
pub(crate) use ast_node;
pub(crate) use ast_node_no_partial_eq;
pub(crate) use expr_node;
pub(crate) use generate_ast_methods;
pub(crate) use node_group;
pub(crate) use stmt_node;
