use super::{
    browser::GetImmediateParent,
    capturable::Capturable,
    graph::{LegacyWorkspaceCallGraph, WorkspaceCallGraphs},
    macros::generate_get_source_unit,
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

    // relative source filepaths
    pub src_filepaths: Vec<String>,
    pub sloc_stats: HashMap<String, usize>,
    pub ignore_lines_stats: HashMap<String, Vec<IgnoreLine>>,
    pub nodes: HashMap<NodeID, ASTNode>,

    // Legacy callgraphs
    pub inward_callgraph: Option<LegacyWorkspaceCallGraph>,
    pub outward_callgraph: Option<LegacyWorkspaceCallGraph>,

    // Callgraphs
    pub callgraphs: Option<WorkspaceCallGraphs>,

    // Source units
    pub source_units_context: Vec<SourceUnit>,

    // In-scope files
    pub included: HashSet<PathBuf>,

    // Function router
    pub router: Option<Router>,

    // Hashmaps of all nodes => source_unit_id
    pub(crate) array_type_names_context: HashMap<ArrayTypeName, NodeContext>,
    pub(crate) assignments_context: HashMap<Assignment, NodeContext>,
    pub(crate) binary_operations_context: HashMap<BinaryOperation, NodeContext>,
    pub(crate) blocks_context: HashMap<Block, NodeContext>,
    pub(crate) conditionals_context: HashMap<Conditional, NodeContext>,
    pub(crate) contract_definitions_context: HashMap<ContractDefinition, NodeContext>,
    pub(crate) elementary_type_names_context: HashMap<ElementaryTypeName, NodeContext>,
    pub(crate) elementary_type_name_expressions_context:
        HashMap<ElementaryTypeNameExpression, NodeContext>,
    pub(crate) emit_statements_context: HashMap<EmitStatement, NodeContext>,
    pub(crate) enum_definitions_context: HashMap<EnumDefinition, NodeContext>,
    pub(crate) enum_values_context: HashMap<EnumValue, NodeContext>,
    pub(crate) event_definitions_context: HashMap<EventDefinition, NodeContext>,
    pub(crate) error_definitions_context: HashMap<ErrorDefinition, NodeContext>,
    pub(crate) expression_statements_context: HashMap<ExpressionStatement, NodeContext>,
    pub(crate) function_calls_context: HashMap<FunctionCall, NodeContext>,
    pub(crate) function_call_options_context: HashMap<FunctionCallOptions, NodeContext>,
    pub(crate) function_definitions_context: HashMap<FunctionDefinition, NodeContext>,
    pub(crate) function_type_names_context: HashMap<FunctionTypeName, NodeContext>,
    pub(crate) for_statements_context: HashMap<ForStatement, NodeContext>,
    pub(crate) identifiers_context: HashMap<Identifier, NodeContext>,
    pub(crate) identifier_paths_context: HashMap<IdentifierPath, NodeContext>,
    pub(crate) if_statements_context: HashMap<IfStatement, NodeContext>,
    pub(crate) import_directives_context: HashMap<ImportDirective, NodeContext>,
    pub(crate) index_accesses_context: HashMap<IndexAccess, NodeContext>,
    pub(crate) index_range_accesses_context: HashMap<IndexRangeAccess, NodeContext>,
    pub(crate) inheritance_specifiers_context: HashMap<InheritanceSpecifier, NodeContext>,
    pub(crate) inline_assemblies_context: HashMap<InlineAssembly, NodeContext>,
    pub(crate) literals_context: HashMap<Literal, NodeContext>,
    pub(crate) member_accesses_context: HashMap<MemberAccess, NodeContext>,
    pub(crate) new_expressions_context: HashMap<NewExpression, NodeContext>,
    pub(crate) mappings_context: HashMap<Mapping, NodeContext>,
    pub(crate) modifier_definitions_context: HashMap<ModifierDefinition, NodeContext>,
    pub(crate) modifier_invocations_context: HashMap<ModifierInvocation, NodeContext>,
    pub(crate) override_specifiers_context: HashMap<OverrideSpecifier, NodeContext>,
    pub(crate) parameter_lists_context: HashMap<ParameterList, NodeContext>,
    pub(crate) pragma_directives_context: HashMap<PragmaDirective, NodeContext>,
    pub(crate) returns_context: HashMap<Return, NodeContext>,
    pub(crate) revert_statements_context: HashMap<RevertStatement, NodeContext>,
    pub(crate) struct_definitions_context: HashMap<StructDefinition, NodeContext>,
    pub(crate) structured_documentations_context: HashMap<StructuredDocumentation, NodeContext>,
    pub(crate) try_statements_context: HashMap<TryStatement, NodeContext>,
    pub(crate) try_catch_clauses_context: HashMap<TryCatchClause, NodeContext>,
    pub(crate) tuple_expressions_context: HashMap<TupleExpression, NodeContext>,
    pub(crate) unary_operations_context: HashMap<UnaryOperation, NodeContext>,
    pub(crate) unchecked_blocks_context: HashMap<UncheckedBlock, NodeContext>,
    pub(crate) user_defined_type_names_context: HashMap<UserDefinedTypeName, NodeContext>,
    pub(crate) user_defined_value_type_definitions_context:
        HashMap<UserDefinedValueTypeDefinition, NodeContext>,
    pub(crate) using_for_directives_context: HashMap<UsingForDirective, NodeContext>,
    pub(crate) variable_declarations_context: HashMap<VariableDeclaration, NodeContext>,
    pub(crate) variable_declaration_statements_context:
        HashMap<VariableDeclarationStatement, NodeContext>,
    pub(crate) while_statements_context: HashMap<WhileStatement, NodeContext>,
    pub(crate) do_while_statements_context: HashMap<DoWhileStatement, NodeContext>,
    pub(crate) break_statements_context: HashMap<Break, NodeContext>,
    pub(crate) continue_statements_context: HashMap<Continue, NodeContext>,
    pub(crate) placeholder_statements_context: HashMap<PlaceholderStatement, NodeContext>,
    pub(crate) yul_function_calls_context: HashMap<YulFunctionCall, NodeContext>,
    pub(crate) yul_identifiers_context: HashMap<YulIdentifier, NodeContext>,
    pub(crate) yul_literals_context: HashMap<YulLiteral, NodeContext>,
    pub(crate) yul_assignments_context: HashMap<YulAssignment, NodeContext>,
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
            if let Some(content) = source_unit.source.as_ref() {
                if offset + len < content.len() {
                    let required_content = &content[offset..offset + len];
                    return Some(required_content.to_string());
                }
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
}

impl WorkspaceContext {
    pub fn array_type_names(&self) -> Vec<&ArrayTypeName> {
        self.array_type_names_context.keys().collect()
    }
    pub fn assignments(&self) -> Vec<&Assignment> {
        self.assignments_context.keys().collect()
    }
    pub fn binary_operations(&self) -> Vec<&BinaryOperation> {
        self.binary_operations_context.keys().collect()
    }
    pub fn blocks(&self) -> Vec<&Block> {
        self.blocks_context.keys().collect()
    }
    pub fn conditionals(&self) -> Vec<&Conditional> {
        self.conditionals_context.keys().collect()
    }
    pub fn contract_definitions(&self) -> Vec<&ContractDefinition> {
        self.contract_definitions_context.keys().collect()
    }
    pub fn elementary_type_names(&self) -> Vec<&ElementaryTypeName> {
        self.elementary_type_names_context.keys().collect()
    }
    pub fn elementary_type_name_expressions(&self) -> Vec<&ElementaryTypeNameExpression> {
        self.elementary_type_name_expressions_context.keys().collect()
    }
    pub fn emit_statements(&self) -> Vec<&EmitStatement> {
        self.emit_statements_context.keys().collect()
    }
    pub fn enum_definitions(&self) -> Vec<&EnumDefinition> {
        self.enum_definitions_context.keys().collect()
    }
    pub fn enum_values(&self) -> Vec<&EnumValue> {
        self.enum_values_context.keys().collect()
    }
    pub fn event_definitions(&self) -> Vec<&EventDefinition> {
        self.event_definitions_context.keys().collect()
    }
    pub fn error_definitions(&self) -> Vec<&ErrorDefinition> {
        self.error_definitions_context.keys().collect()
    }
    pub fn expression_statements(&self) -> Vec<&ExpressionStatement> {
        self.expression_statements_context.keys().collect()
    }
    pub fn function_calls(&self) -> Vec<&FunctionCall> {
        self.function_calls_context.keys().collect()
    }
    pub fn function_call_options(&self) -> Vec<&FunctionCallOptions> {
        self.function_call_options_context.keys().collect()
    }
    pub fn function_definitions(&self) -> Vec<&FunctionDefinition> {
        self.function_definitions_context.keys().collect()
    }
    pub fn function_type_names(&self) -> Vec<&FunctionTypeName> {
        self.function_type_names_context.keys().collect()
    }
    pub fn for_statements(&self) -> Vec<&ForStatement> {
        self.for_statements_context.keys().collect()
    }
    pub fn identifiers(&self) -> Vec<&Identifier> {
        self.identifiers_context.keys().collect()
    }
    pub fn identifier_paths(&self) -> Vec<&IdentifierPath> {
        self.identifier_paths_context.keys().collect()
    }
    pub fn if_statements(&self) -> Vec<&IfStatement> {
        self.if_statements_context.keys().collect()
    }
    pub fn import_directives(&self) -> Vec<&ImportDirective> {
        self.import_directives_context.keys().collect()
    }
    pub fn index_accesses(&self) -> Vec<&IndexAccess> {
        self.index_accesses_context.keys().collect()
    }
    pub fn index_range_accesses(&self) -> Vec<&IndexRangeAccess> {
        self.index_range_accesses_context.keys().collect()
    }
    pub fn inheritance_specifiers(&self) -> Vec<&InheritanceSpecifier> {
        self.inheritance_specifiers_context.keys().collect()
    }
    pub fn inline_assemblies(&self) -> Vec<&InlineAssembly> {
        self.inline_assemblies_context.keys().collect()
    }
    pub fn literals(&self) -> Vec<&Literal> {
        self.literals_context.keys().collect()
    }
    pub fn member_accesses(&self) -> Vec<&MemberAccess> {
        self.member_accesses_context.keys().collect()
    }
    pub fn new_expressions(&self) -> Vec<&NewExpression> {
        self.new_expressions_context.keys().collect()
    }
    pub fn mappings(&self) -> Vec<&Mapping> {
        self.mappings_context.keys().collect()
    }
    pub fn modifier_definitions(&self) -> Vec<&ModifierDefinition> {
        self.modifier_definitions_context.keys().collect()
    }
    pub fn modifier_invocations(&self) -> Vec<&ModifierInvocation> {
        self.modifier_invocations_context.keys().collect()
    }
    pub fn override_specifiers(&self) -> Vec<&OverrideSpecifier> {
        self.override_specifiers_context.keys().collect()
    }
    pub fn parameter_lists(&self) -> Vec<&ParameterList> {
        self.parameter_lists_context.keys().collect()
    }
    pub fn pragma_directives(&self) -> Vec<&PragmaDirective> {
        self.pragma_directives_context.keys().collect()
    }
    pub fn returns(&self) -> Vec<&Return> {
        self.returns_context.keys().collect()
    }
    pub fn revert_statements(&self) -> Vec<&RevertStatement> {
        self.revert_statements_context.keys().collect()
    }
    pub fn source_units(&self) -> Vec<&SourceUnit> {
        self.source_units_context.iter().collect()
    }
    pub fn struct_definitions(&self) -> Vec<&StructDefinition> {
        self.struct_definitions_context.keys().collect()
    }
    pub fn structured_documentations(&self) -> Vec<&StructuredDocumentation> {
        self.structured_documentations_context.keys().collect()
    }
    pub fn try_statements(&self) -> Vec<&TryStatement> {
        self.try_statements_context.keys().collect()
    }
    pub fn try_catch_clauses(&self) -> Vec<&TryCatchClause> {
        self.try_catch_clauses_context.keys().collect()
    }
    pub fn tuple_expressions(&self) -> Vec<&TupleExpression> {
        self.tuple_expressions_context.keys().collect()
    }
    pub fn unary_operations(&self) -> Vec<&UnaryOperation> {
        self.unary_operations_context.keys().collect()
    }

    pub fn unchecked_blocks(&self) -> Vec<&UncheckedBlock> {
        self.unchecked_blocks_context.keys().collect()
    }

    pub fn user_defined_type_names(&self) -> Vec<&UserDefinedTypeName> {
        self.user_defined_type_names_context.keys().collect()
    }
    pub fn user_defined_value_type_definitions(&self) -> Vec<&UserDefinedValueTypeDefinition> {
        self.user_defined_value_type_definitions_context.keys().collect()
    }
    pub fn using_for_directives(&self) -> Vec<&UsingForDirective> {
        self.using_for_directives_context.keys().collect()
    }
    pub fn variable_declarations(&self) -> Vec<&VariableDeclaration> {
        self.variable_declarations_context.keys().collect()
    }
    pub fn variable_declaration_statements(&self) -> Vec<&VariableDeclarationStatement> {
        self.variable_declaration_statements_context.keys().collect()
    }
    pub fn while_statements(&self) -> Vec<&WhileStatement> {
        self.while_statements_context.keys().collect()
    }

    pub fn do_while_statements(&self) -> Vec<&DoWhileStatement> {
        self.do_while_statements_context.keys().collect()
    }

    pub fn break_statements(&self) -> Vec<&Break> {
        self.break_statements_context.keys().collect()
    }

    pub fn continue_statements(&self) -> Vec<&Continue> {
        self.continue_statements_context.keys().collect()
    }

    pub fn placeholder_statements(&self) -> Vec<&PlaceholderStatement> {
        self.placeholder_statements_context.keys().collect()
    }

    pub fn yul_function_calls(&self) -> Vec<&YulFunctionCall> {
        self.yul_function_calls_context.keys().collect()
    }

    pub fn yul_identifiers(&self) -> Vec<&YulIdentifier> {
        self.yul_identifiers_context.keys().collect()
    }

    pub fn yul_assignments(&self) -> Vec<&YulAssignment> {
        self.yul_assignments_context.keys().collect()
    }

    pub fn yul_literals(&self) -> Vec<&YulLiteral> {
        self.yul_literals_context.keys().collect()
    }
}

generate_get_source_unit! {
    ArrayTypeName => array_type_names_context,
    Assignment => assignments_context,
    BinaryOperation => binary_operations_context,
    Block => blocks_context,
    Conditional => conditionals_context,
    ContractDefinition => contract_definitions_context,
    ElementaryTypeName => elementary_type_names_context,
    ElementaryTypeNameExpression => elementary_type_name_expressions_context,
    EmitStatement => emit_statements_context,
    EnumDefinition => enum_definitions_context,
    EnumValue => enum_values_context,
    EventDefinition => event_definitions_context,
    ErrorDefinition => error_definitions_context,
    ExpressionStatement => expression_statements_context,
    FunctionCall => function_calls_context,
    FunctionCallOptions => function_call_options_context,
    FunctionDefinition => function_definitions_context,
    FunctionTypeName => function_type_names_context,
    ForStatement => for_statements_context,
    Identifier => identifiers_context,
    IdentifierPath => identifier_paths_context,
    IfStatement => if_statements_context,
    ImportDirective => import_directives_context,
    IndexAccess => index_accesses_context,
    IndexRangeAccess => index_range_accesses_context,
    InheritanceSpecifier => inheritance_specifiers_context,
    InlineAssembly => inline_assemblies_context,
    Literal => literals_context,
    MemberAccess => member_accesses_context,
    NewExpression => new_expressions_context,
    Mapping => mappings_context,
    ModifierDefinition => modifier_definitions_context,
    ModifierInvocation => modifier_invocations_context,
    OverrideSpecifier => override_specifiers_context,
    ParameterList => parameter_lists_context,
    PragmaDirective => pragma_directives_context,
    Return => returns_context,
    RevertStatement => revert_statements_context,
    StructDefinition => struct_definitions_context,
    StructuredDocumentation => structured_documentations_context,
    TryStatement => try_statements_context,
    TryCatchClause => try_catch_clauses_context,
    TupleExpression => tuple_expressions_context,
    UnaryOperation => unary_operations_context,
    UncheckedBlock => unchecked_blocks_context,
    UserDefinedTypeName => user_defined_type_names_context,
    UserDefinedValueTypeDefinition => user_defined_value_type_definitions_context,
    UsingForDirective => using_for_directives_context,
    VariableDeclaration => variable_declarations_context,
    VariableDeclarationStatement => variable_declaration_statements_context,
    WhileStatement => while_statements_context,
    DoWhileStatement => do_while_statements_context,
    Break => break_statements_context,
    Continue => continue_statements_context,
    PlaceholderStatement => placeholder_statements_context,
    YulFunctionCall => yul_function_calls_context,
    YulIdentifier => yul_identifiers_context,
    YulLiteral => yul_literals_context,
}
