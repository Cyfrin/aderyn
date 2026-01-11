use super::{
    browser::GetImmediateParent,
    graph::{LegacyWorkspaceCallGraph, WorkspaceCallGraphs},
    router::Router,
};
pub use crate::ast::ASTNode;
use crate::{ast::*, stats::IgnoreLine};
use solidity_ast::EvmVersion;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use paste::paste;

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

impl WorkspaceContext {
    // Setters
    pub fn set_sloc_stats(&mut self, sloc_stats: HashMap<String, usize>) {
        self.sloc_stats = sloc_stats;
    }

    pub fn set_ignore_lines_stats(&mut self, ignore_lines_stats: HashMap<String, Vec<IgnoreLine>>) {
        self.ignore_lines_stats = ignore_lines_stats;
    }

    // Getters
    pub fn get_parent(&self, node_id: NodeID) -> Option<&ASTNode> {
        self.nodes.get(self.parent_link.get(&node_id)?)
    }

    pub fn get_ancestral_line(&self, node_id: NodeID) -> Vec<&ASTNode> {
        let mut chain = vec![];
        let mut parent = self.nodes.get(&node_id);
        while let Some(next_parent) = parent {
            chain.push(next_parent);
            parent = next_parent.parent(self);
        }
        chain
    }
    pub fn get_closest_ancestor(&self, node_id: NodeID, node_type: NodeType) -> Option<&ASTNode> {
        let mut current_node_id = self.parent_link.get(&node_id)?;
        while let Some(current) = self.nodes.get(current_node_id) {
            if current.node_type() == node_type {
                return Some(current);
            }
            current_node_id = self.parent_link.get(current_node_id)?;
        }
        None
    }
    pub fn get_closest_ancestor_including_self(
        &self,
        node_id: NodeID,
        node_type: NodeType,
    ) -> Option<&ASTNode> {
        if let Some(node) = self.nodes.get(&node_id)
            && node.node_type() == node_type
        {
            return Some(node);
        }
        self.get_closest_ancestor(node_id, node_type)
    }
    pub fn get_source_code_of_node(&self, node_id: NodeID) -> Option<String> {
        let node = self.nodes.get(&node_id)?;
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let src_location = node.src().unwrap_or("");

        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        if let Some((offset, len)) = chopped_location.split_once(':') {
            let offset: usize = offset.parse().ok()?;
            let len: usize = len.parse().ok()?;
            if let Some(content) = source_unit.source.as_ref()
                && offset + len < content.len()
            {
                let required_content = &content[offset..offset + len];
                return Some(required_content.to_string());
            }
        }
        None
    }

    pub fn get_offset_and_length_of_node(&self, node_id: NodeID) -> Option<(usize, usize)> {
        let node = self.nodes.get(&node_id)?;
        let src_location = node.src().unwrap_or("");

        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        if let Some((offset, len)) = chopped_location.split_once(':') {
            let offset: usize = offset.parse().ok()?;
            let len: usize = len.parse().ok()?;
            return Some((offset, len));
        }
        None
    }

    pub fn get_node_sort_key_from_capturable(
        &self,
        capturable: &Capturable,
    ) -> (String, usize, String) {
        capturable.make_key(self)
    }

    pub fn get_node_id_of_capturable(&self, capturable: &Capturable) -> Option<NodeID> {
        capturable.id()
    }

    /// Returns the relative location of nodes in the source code (if they are in same file)
    pub fn get_relative_location_of_nodes(
        &self,
        first: NodeID,
        second: NodeID,
    ) -> Option<Ordering> {
        let f = self.get_node_sort_key_pure(self.nodes.get(&first)?);
        let s = self.get_node_sort_key_pure(self.nodes.get(&second)?);

        // If the nodes aren't in the same file location comparison doesn't make sense
        if f.0 != s.0 {
            return None;
        }

        match f.1.cmp(&s.1) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => {
                // If the nodes are on the same line, we must compare offset in the chopped_location
                let first_character_offset = f.2.split_once(':').unwrap();
                let second_character_offset = s.2.split_once(':').unwrap();
                Some(first_character_offset.0.cmp(second_character_offset.0))
            }
            Ordering::Greater => Some(Ordering::Greater),
        }
    }

    pub fn get_node_sort_key_pure(&self, node: &ASTNode) -> (String, usize, String) {
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();
        let source_line = node
            .src()
            .map(|src| source_unit.source_line(src).unwrap_or(0)) // If `src` is `Some`, get the line number, else return 0
            .unwrap_or(0); // If `src` is `None`, default to 0

        let src_location = node.src().unwrap_or("");

        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        (absolute_path, source_line, chopped_location)
    }

    pub fn get_node_sort_key(&self, node: &ASTNode) -> (String, usize, String) {
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();
        let source_line =
            node.src().map(|src| source_unit.source_line(src).unwrap_or(0)).unwrap_or(0);

        let src_location = match node {
            ASTNode::ContractDefinition(contract_node) => contract_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| contract_node.src.clone(), |loc| loc.clone()),
            ASTNode::FunctionDefinition(function_node) => function_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| function_node.src.clone(), |loc| loc.clone()),
            ASTNode::ModifierDefinition(modifier_node) => modifier_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| modifier_node.src.clone(), |loc| loc.clone()),
            ASTNode::VariableDeclaration(variable_node) => variable_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| variable_node.src.clone(), |loc| loc.clone()),
            _ => node.src().unwrap_or("").to_string(),
        };

        let chopped_location = src_location
            .rfind(':')
            .map(|index| src_location[..index].to_string())
            .unwrap_or(src_location);

        (absolute_path, source_line, chopped_location)
    }
    pub fn get_code_snippet(&self, node: &ASTNode) -> String {
        let (filepath, _, src_location) = self.get_node_sort_key_pure(node);
        let source_unit = self
            .source_units()
            .into_iter()
            .find(|s| s.absolute_path.as_ref().is_some_and(|p| *p == filepath))
            .expect("node not found");

        let source_content = source_unit.source.as_ref().expect("source not found");

        let (byte_offset_str, byte_len_str) = src_location.split_once(':').unwrap();
        let byte_offset: usize = byte_offset_str.parse().unwrap();
        let byte_length: usize = byte_len_str.parse().unwrap();

        let code_snippet = &source_content[byte_offset..byte_offset + byte_length];
        code_snippet.to_owned()
    }
}

impl From<&&ContractDefinition> for Capturable {
    fn from(value: &&ContractDefinition) -> Self {
        #[allow(suspicious_double_ref_op)]
        Self::ContractDefinition(value.clone().clone())
    }
}

impl From<&&ModifierInvocation> for Capturable {
    fn from(value: &&ModifierInvocation) -> Self {
        #[allow(suspicious_double_ref_op)]
        Self::ModifierInvocation(value.clone().clone())
    }
}

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
