use super::{
    graph::{LegacyWorkspaceCallGraph, WorkspaceCallGraphs},
    router::Router,
};
use crate::{
    ast::{
        ast_visitor::{ASTConstVisitor, Node},
        *,
    },
    stats::IgnoreLine,
};
use paste::paste;
use solidity_ast::EvmVersion;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

macro_rules! create_workspace_context {
    (
        regular: $($type:ident),* $(,)?;
        yul: $($yul_type:ident),* $(,)?;
    ) => {
        paste! {
            #[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
            pub struct NodeContext {
                pub source_unit_id: NodeID,
                pub contract_definition_id: Option<NodeID>,
                pub function_definition_id: Option<NodeID>,
                pub modifier_definition_id: Option<NodeID>,
            }

            #[derive(Default, Debug)]
            pub struct WorkspaceContext {
                pub last_source_unit_id: NodeID,
                pub last_contract_definition_id: Option<NodeID>,
                pub last_function_definition_id: Option<NodeID>,
                pub last_modifier_definition_id: Option<NodeID>,

                pub parent_link: HashMap<NodeID, NodeID>,
                pub evm_version: EvmVersion,

                pub router: Option<Router>, // Function router
                pub src_filepaths: Vec<String>, // Relative source filepaths
                pub sloc_stats: HashMap<String, usize>,
                pub ignore_lines_stats: HashMap<String, Vec<IgnoreLine>>,
                pub nodes: HashMap<NodeID, ASTNode>,

                pub inward_callgraph: Option<LegacyWorkspaceCallGraph>,
                pub outward_callgraph: Option<LegacyWorkspaceCallGraph>,
                pub callgraphs: Option<WorkspaceCallGraphs>,

                pub included: HashSet<PathBuf>, // In-scope files
                pub via_ir: bool, // True if via_ir is configured for the project.

                pub source_units_context: Vec<SourceUnit>,

                // Yul nodes
                $(
                    pub(crate) [<$yul_type:snake s_context>]: HashMap<$yul_type, NodeContext>,
                )*

                // FIXME: YulAssignment doesn't come with "src" field, so we can't capture location
                // Therefore for now it is special cased inside this macro impl. Not sure what to do yet.
                // Temporary workaround is to inspect higher level Yul constructs (such as YulFunctionCall,
                // YulIdentifier, YulLiteral, etc) and then search for trigger conditions inside those.
                // If we need to flag, then fag the higher level Yul construct.
                //
                // Hopefully later versions of Solc emit some kind of "src" field. At that point, you can
                // add YulAssignment to the list where this macro is called!
                pub(crate) yul_assignments_context: HashMap<YulAssignment, NodeContext>,

                // Regular Nodes
                $(
                    pub(crate) [<$type:snake s_context>]: HashMap<$type, NodeContext>,
                )*
            }

            impl WorkspaceContext {
                // Regular nodes
                $(
                    pub fn [<$type:snake s>](&self) -> Vec<&$type> {
                        self.[<$type:snake s_context>].keys().collect()
                    }
                )*

                // Yul nodes
                $(
                    pub fn [<$yul_type:snake s>](&self) -> Vec<&$yul_type> {
                        self.[<$yul_type:snake s_context>].keys().collect()
                    }
                )*

                // FIXME: See FIXME note above
                pub fn yul_assignments(&self) -> Vec<&YulAssignment> {
                    self.yul_assignments_context.keys().collect()
                }

                pub fn source_units(&self) -> Vec<&SourceUnit> {
                    self.source_units_context.iter().collect()
                }

                pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
                    let source_unit_id = match node {
                        // Regular nodes
                        $(
                            ASTNode::$type(n) => self.[<$type:snake s_context>].get(n).map(|c| c.source_unit_id),
                        )*

                        // Yul nodes
                        $(
                            ASTNode::$yul_type(n) => self.[<$yul_type:snake s_context>].get(n).map(|c| c.source_unit_id),
                        )*

                        // Source Unit AST Node
                        ASTNode::SourceUnit(n) => Some(n.id),
                    };
                    source_unit_id.and_then(|id| {
                        self.source_units_context.iter().find(|su| su.id == id)
                    })
                }
            }

            #[derive(Clone)]
            pub enum Capturable {
                $($type($type),)* // Regular nodes
                $($yul_type($yul_type),)* // Yul nodes
                ASTNode(ASTNode),
                SourceUnit(SourceUnit),
            }

            // Regular Nodes
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

            // Yul Nodes
            $(
                impl From<$yul_type> for Capturable {
                    fn from(value: $yul_type) -> Self {
                        Self::$yul_type(value)
                    }
                }

                impl From<&$yul_type> for Capturable {
                    fn from(value: &$yul_type) -> Self {
                        Self::$yul_type(value.clone())
                    }
                }
            )*

            // Source Unit
            impl From<SourceUnit> for Capturable {
                fn from(value: SourceUnit) -> Self {
                    Self::SourceUnit(value)
                }
            }

            impl From<&SourceUnit> for Capturable {
                fn from(value: &SourceUnit) -> Self {
                    Self::SourceUnit(value.clone())
                }
            }

            // ASTNode
            impl From<ASTNode> for Capturable {
                fn from(value: ASTNode) -> Self {
                    Self::ASTNode(value)
                }
            }

            impl From<&ASTNode> for Capturable {
                fn from(value: &ASTNode) -> Self {
                    Self::ASTNode(value.clone())
                }
            }

            impl Capturable {
                pub fn make_key(&self, context: &WorkspaceContext) -> (String, usize, String) {
                    match self {
                        Self::ASTNode(node) => context.get_node_sort_key(node),
                        Self::SourceUnit(n) => context.get_node_sort_key(&n.into()),
                        $(Self::$type(n) => context.get_node_sort_key(&n.into()),)*
                        $(Self::$yul_type(n) => context.get_node_sort_key(&n.into()),)*
                    }
                }
                pub fn id(&self) -> Option<NodeID> {
                    match self {
                        Self::ASTNode(ast_node) => ast_node.id(),
                        Self::SourceUnit(source_unit_node) => Some(source_unit_node.id),
                        $(Self::$type(n) => Some(n.id),)*
                        $(Self::$yul_type(_) => None,)*
                    }
                }
            }

            #[derive(Debug, Clone, PartialEq)]
            pub enum ASTNode {
                $($type($type),)*
                $($yul_type($yul_type),)*
                SourceUnit(SourceUnit),
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

            // Source Unit
            impl From<SourceUnit> for ASTNode {
                fn from(value: SourceUnit) -> Self {
                    ASTNode::SourceUnit(value)
                }
            }

            impl From<&SourceUnit> for ASTNode {
                fn from(value: &SourceUnit) -> Self {
                    ASTNode::SourceUnit(value.clone())
                }
            }

            impl ASTNode {
                pub fn node_type(&self) -> NodeType {
                    match self {
                        $(ASTNode::$type(_) => NodeType::$type,)*
                        $(ASTNode::$yul_type(_) => NodeType::$yul_type,)*
                        ASTNode::SourceUnit(_) => NodeType::SourceUnit,
                    }
                }
                pub fn id(&self) -> Option<NodeID> {
                    match self {
                        $(ASTNode::$type(n) => Some(n.id),)*
                        $(ASTNode::$yul_type(_) => None,)*
                        ASTNode::SourceUnit(n) => Some(n.id),
                    }
                }
            }

            impl Node for ASTNode {
                fn accept(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                    match self {
                        $(ASTNode::$type(n) => n.accept(visitor),)*
                        $(ASTNode::$yul_type(n) => n.accept(visitor),)*
                        ASTNode::SourceUnit(n) => n.accept(visitor),
                    }
                }
                fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                    match self {
                        $(ASTNode::$type(n) => n.accept_metadata(visitor),)*
                        $(ASTNode::$yul_type(n) => n.accept_metadata(visitor),)*
                        ASTNode::SourceUnit(n) => n.accept_metadata(visitor),
                    }
                }
                fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                    visitor.visit_node_id(self.id())?;
                    Ok(())
                }
            }

            impl ASTNode {
                pub fn src(&self) -> Option<&str> {
                    match self {
                        $(ASTNode::$type(node) => Some(&node.src),)*
                        $(ASTNode::$yul_type(node) => Some(&node.src),)*
                        ASTNode::SourceUnit(node) => Some(&node.src),
                    }
                }
            }

        }
    };
}

create_workspace_context! {
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
}
