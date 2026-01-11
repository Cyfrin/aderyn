use super::{
    graph::{LegacyWorkspaceCallGraph, WorkspaceCallGraphs},
    router::Router,
};
use crate::{ast::*, stats::IgnoreLine};
use paste::paste;
use solidity_ast::EvmVersion;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub use crate::ast::ASTNode;

macro_rules! define_node_contexts {
    ($($type:ident),* $(,)?) => {
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
                pub(crate) yul_assignments_context: HashMap<YulAssignment, NodeContext>,
                pub(crate) yul_function_calls_context: HashMap<YulFunctionCall, NodeContext>,
                pub(crate) yul_identifiers_context: HashMap<YulIdentifier, NodeContext>,
                pub(crate) yul_literals_context: HashMap<YulLiteral, NodeContext>,

                // Regular Nodes
                $(
                    pub(crate) [<$type:snake s_context>]: HashMap<$type, NodeContext>,
                )*
            }

            impl WorkspaceContext {
                $(
                    pub fn [<$type:snake s>](&self) -> Vec<&$type> {
                        self.[<$type:snake s_context>].keys().collect()
                    }
                )*

                pub fn yul_assignments(&self) -> Vec<&YulAssignment> {
                    self.yul_assignments_context.keys().collect()
                }
                pub fn yul_function_calls(&self) -> Vec<&YulFunctionCall> {
                    self.yul_function_calls_context.keys().collect()
                }
                pub fn yul_identifiers(&self) -> Vec<&YulIdentifier> {
                    self.yul_identifiers_context.keys().collect()
                }
                pub fn yul_literals(&self) -> Vec<&YulLiteral> {
                    self.yul_literals_context.keys().collect()
                }

                pub fn source_units(&self) -> Vec<&SourceUnit> {
                    self.source_units_context.iter().collect()
                }

                pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
                    let source_unit_id = match node {
                        ASTNode::SourceUnit(n) => Some(n.id),
                        $(
                            ASTNode::$type(n) => self.[<$type:snake s_context>].get(n).map(|c| c.source_unit_id),
                        )*
                        ASTNode::YulFunctionCall(n) => self.yul_function_calls_context.get(n).map(|c| c.source_unit_id),
                        ASTNode::YulIdentifier(n) => self.yul_identifiers_context.get(n).map(|c| c.source_unit_id),
                        ASTNode::YulLiteral(n) => self.yul_literals_context.get(n).map(|c| c.source_unit_id),
                    };
                    source_unit_id.and_then(|id| {
                        self.source_units_context.iter().find(|su| su.id == id)
                    })
                }
            }

            #[derive(Clone)]
            pub enum Capturable {
                $($type($type),)*
                YulFunctionCall(YulFunctionCall),
                YulIdentifier(YulIdentifier),
                YulLiteral(YulLiteral),
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
            impl From<YulFunctionCall> for Capturable {
                fn from(value: YulFunctionCall) -> Self {
                    Self::YulFunctionCall(value)
                }
            }

            impl From<&YulIdentifier> for Capturable {
                fn from(value: &YulIdentifier) -> Self {
                    Self::YulIdentifier(value.clone())
                }
            }

            impl From<YulLiteral> for Capturable {
                fn from(value: YulLiteral) -> Self {
                    Self::YulLiteral(value)
                }
            }

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
                        Self::YulFunctionCall(n) => context.get_node_sort_key(&n.into()),
                        Self::YulIdentifier(n) => context.get_node_sort_key(&n.into()),
                        Self::YulLiteral(n) => context.get_node_sort_key(&n.into()),
                        Self::SourceUnit(n) => context.get_node_sort_key(&n.into()),
                        $(Self::$type(n) => context.get_node_sort_key(&n.into()),)*
                    }
                }
                pub fn id(&self) -> Option<NodeID> {
                    match self {
                        Self::ASTNode(ast_node) => ast_node.id(),
                        Self::YulFunctionCall(_) => None,
                        Self::YulIdentifier(_) => None,
                        Self::YulLiteral(_) => None,
                        Self::SourceUnit(source_unit_node) => Some(source_unit_node.id),
                        $(Self::$type(n) => Some(n.id),)*
                    }
                }
            }
        }
    };
}

define_node_contexts! {
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
    WhileStatement,
}
