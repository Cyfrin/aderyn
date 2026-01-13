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

macro_rules! accept_id {
    () => {
        fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
            visitor.visit_node_id(Some(self.id))?;
            Ok(())
        }
    };
}

macro_rules! with_node_types {
    ($callback:ident) => {
        $callback! {
            regular:
                ArrayTypeName,
                Assignment,
                BinaryOperation,
                Block,
                Break,
                Conditional,
                Continue,
                ContractDefinition,
                DoWhileStatement,
                ElementaryTypeName,
                ElementaryTypeNameExpression,
                EmitStatement,
                EnumDefinition,
                EnumValue,
                ErrorDefinition,
                EventDefinition,
                ExpressionStatement,
                ForStatement,
                FunctionCall,
                FunctionCallOptions,
                FunctionDefinition,
                FunctionTypeName,
                Identifier,
                IdentifierPath,
                IfStatement,
                ImportDirective,
                IndexAccess,
                IndexRangeAccess,
                InheritanceSpecifier,
                InlineAssembly,
                Literal,
                Mapping,
                MemberAccess,
                ModifierDefinition,
                ModifierInvocation,
                NewExpression,
                OverrideSpecifier,
                ParameterList,
                PlaceholderStatement,
                PragmaDirective,
                Return,
                RevertStatement,
                StructDefinition,
                StructuredDocumentation,
                TryCatchClause,
                TryStatement,
                TupleExpression,
                UnaryOperation,
                UncheckedBlock,
                UserDefinedTypeName,
                UserDefinedValueTypeDefinition,
                UsingForDirective,
                VariableDeclaration,
                VariableDeclarationStatement,
                WhileStatement;
            yul:
                YulFunctionCall,
                YulIdentifier,
                YulLiteral;
            yul_sourceless:
                YulAssignment,
                YulBlock,
                YulCase,
                YulExpression,
                YulExpressionStatement,
                YulForLoop,
                YulFunctionDefinition,
                YulIf,
                YulStatement,
                YulSwitch,
                YulTypedName,
                YulVariableDeclaration;
        }
    };
}

pub(crate) use accept_id;
pub(crate) use ast_node;
pub(crate) use ast_node_no_partial_eq;
pub(crate) use expr_node;
pub(crate) use node_group;
pub(crate) use stmt_node;
pub(crate) use with_node_types;
