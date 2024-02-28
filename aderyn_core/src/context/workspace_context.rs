use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    ArrayTypeName(ArrayTypeName),
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Block(Block),
    Conditional(Conditional),
    ContractDefinition(ContractDefinition),
    ElementaryTypeName(ElementaryTypeName),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    EmitStatement(EmitStatement),
    EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    ExpressionStatement(ExpressionStatement),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    FunctionDefinition(FunctionDefinition),
    FunctionTypeName(FunctionTypeName),
    ForStatement(ForStatement),
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
    IfStatement(IfStatement),
    ImportDirective(ImportDirective),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    InheritanceSpecifier(InheritanceSpecifier),
    InlineAssembly(InlineAssembly),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    Mapping(Mapping),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ParameterList(ParameterList),
    PragmaDirective(PragmaDirective),
    Return(Return),
    RevertStatement(RevertStatement),
    SourceUnit(SourceUnit),
    StructDefinition(StructDefinition),
    StructuredDocumentation(StructuredDocumentation),
    TryStatement(TryStatement),
    TryCatchClause(TryCatchClause),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
}

impl From<ArrayTypeName> for ASTNode {
    fn from(value: ArrayTypeName) -> Self {
        ASTNode::ArrayTypeName(value)
    }
}

impl From<Assignment> for ASTNode {
    fn from(value: Assignment) -> Self {
        ASTNode::Assignment(value)
    }
}

impl From<BinaryOperation> for ASTNode {
    fn from(value: BinaryOperation) -> Self {
        ASTNode::BinaryOperation(value)
    }
}

impl From<Block> for ASTNode {
    fn from(value: Block) -> Self {
        ASTNode::Block(value)
    }
}

impl From<Conditional> for ASTNode {
    fn from(value: Conditional) -> Self {
        ASTNode::Conditional(value)
    }
}

impl From<ContractDefinition> for ASTNode {
    fn from(value: ContractDefinition) -> Self {
        ASTNode::ContractDefinition(value)
    }
}

impl From<ElementaryTypeName> for ASTNode {
    fn from(value: ElementaryTypeName) -> Self {
        ASTNode::ElementaryTypeName(value)
    }
}

impl From<ElementaryTypeNameExpression> for ASTNode {
    fn from(value: ElementaryTypeNameExpression) -> Self {
        ASTNode::ElementaryTypeNameExpression(value)
    }
}

impl From<EmitStatement> for ASTNode {
    fn from(value: EmitStatement) -> Self {
        ASTNode::EmitStatement(value)
    }
}

impl From<EnumDefinition> for ASTNode {
    fn from(value: EnumDefinition) -> Self {
        ASTNode::EnumDefinition(value)
    }
}

impl From<EnumValue> for ASTNode {
    fn from(value: EnumValue) -> Self {
        ASTNode::EnumValue(value)
    }
}

impl From<EventDefinition> for ASTNode {
    fn from(value: EventDefinition) -> Self {
        ASTNode::EventDefinition(value)
    }
}

impl From<ErrorDefinition> for ASTNode {
    fn from(value: ErrorDefinition) -> Self {
        ASTNode::ErrorDefinition(value)
    }
}

impl From<ExpressionStatement> for ASTNode {
    fn from(value: ExpressionStatement) -> Self {
        ASTNode::ExpressionStatement(value)
    }
}

impl From<FunctionCall> for ASTNode {
    fn from(value: FunctionCall) -> Self {
        ASTNode::FunctionCall(value)
    }
}

impl From<FunctionCallOptions> for ASTNode {
    fn from(value: FunctionCallOptions) -> Self {
        ASTNode::FunctionCallOptions(value)
    }
}

impl From<FunctionDefinition> for ASTNode {
    fn from(value: FunctionDefinition) -> Self {
        ASTNode::FunctionDefinition(value)
    }
}

impl From<FunctionTypeName> for ASTNode {
    fn from(value: FunctionTypeName) -> Self {
        ASTNode::FunctionTypeName(value)
    }
}

impl From<ForStatement> for ASTNode {
    fn from(value: ForStatement) -> Self {
        ASTNode::ForStatement(value)
    }
}

impl From<Identifier> for ASTNode {
    fn from(value: Identifier) -> Self {
        ASTNode::Identifier(value)
    }
}

impl From<IdentifierPath> for ASTNode {
    fn from(value: IdentifierPath) -> Self {
        ASTNode::IdentifierPath(value)
    }
}

impl From<IfStatement> for ASTNode {
    fn from(value: IfStatement) -> Self {
        ASTNode::IfStatement(value)
    }
}

impl From<ImportDirective> for ASTNode {
    fn from(value: ImportDirective) -> Self {
        ASTNode::ImportDirective(value)
    }
}

impl From<IndexAccess> for ASTNode {
    fn from(value: IndexAccess) -> Self {
        ASTNode::IndexAccess(value)
    }
}

impl From<IndexRangeAccess> for ASTNode {
    fn from(value: IndexRangeAccess) -> Self {
        ASTNode::IndexRangeAccess(value)
    }
}

impl From<InheritanceSpecifier> for ASTNode {
    fn from(value: InheritanceSpecifier) -> Self {
        ASTNode::InheritanceSpecifier(value)
    }
}

impl From<InlineAssembly> for ASTNode {
    fn from(value: InlineAssembly) -> Self {
        ASTNode::InlineAssembly(value)
    }
}

impl From<Literal> for ASTNode {
    fn from(value: Literal) -> Self {
        ASTNode::Literal(value)
    }
}

impl From<MemberAccess> for ASTNode {
    fn from(value: MemberAccess) -> Self {
        ASTNode::MemberAccess(value)
    }
}

impl From<NewExpression> for ASTNode {
    fn from(value: NewExpression) -> Self {
        ASTNode::NewExpression(value)
    }
}

impl From<Mapping> for ASTNode {
    fn from(value: Mapping) -> Self {
        ASTNode::Mapping(value)
    }
}

impl From<ModifierDefinition> for ASTNode {
    fn from(value: ModifierDefinition) -> Self {
        ASTNode::ModifierDefinition(value)
    }
}

impl From<ModifierInvocation> for ASTNode {
    fn from(value: ModifierInvocation) -> Self {
        ASTNode::ModifierInvocation(value)
    }
}

impl From<OverrideSpecifier> for ASTNode {
    fn from(value: OverrideSpecifier) -> Self {
        ASTNode::OverrideSpecifier(value)
    }
}

impl From<ParameterList> for ASTNode {
    fn from(value: ParameterList) -> Self {
        ASTNode::ParameterList(value)
    }
}

impl From<PragmaDirective> for ASTNode {
    fn from(value: PragmaDirective) -> Self {
        ASTNode::PragmaDirective(value)
    }
}

impl From<Return> for ASTNode {
    fn from(value: Return) -> Self {
        ASTNode::Return(value)
    }
}

impl From<RevertStatement> for ASTNode {
    fn from(value: RevertStatement) -> Self {
        ASTNode::RevertStatement(value)
    }
}

impl From<SourceUnit> for ASTNode {
    fn from(value: SourceUnit) -> Self {
        ASTNode::SourceUnit(value)
    }
}

impl From<StructDefinition> for ASTNode {
    fn from(value: StructDefinition) -> Self {
        ASTNode::StructDefinition(value)
    }
}

impl From<StructuredDocumentation> for ASTNode {
    fn from(value: StructuredDocumentation) -> Self {
        ASTNode::StructuredDocumentation(value)
    }
}

impl From<TryStatement> for ASTNode {
    fn from(value: TryStatement) -> Self {
        ASTNode::TryStatement(value)
    }
}

impl From<TryCatchClause> for ASTNode {
    fn from(value: TryCatchClause) -> Self {
        ASTNode::TryCatchClause(value)
    }
}

impl From<TupleExpression> for ASTNode {
    fn from(value: TupleExpression) -> Self {
        ASTNode::TupleExpression(value)
    }
}

impl From<UnaryOperation> for ASTNode {
    fn from(value: UnaryOperation) -> Self {
        ASTNode::UnaryOperation(value)
    }
}

impl From<UserDefinedTypeName> for ASTNode {
    fn from(value: UserDefinedTypeName) -> Self {
        ASTNode::UserDefinedTypeName(value)
    }
}

impl From<UserDefinedValueTypeDefinition> for ASTNode {
    fn from(value: UserDefinedValueTypeDefinition) -> Self {
        ASTNode::UserDefinedValueTypeDefinition(value)
    }
}

impl From<UsingForDirective> for ASTNode {
    fn from(value: UsingForDirective) -> Self {
        ASTNode::UsingForDirective(value)
    }
}

impl From<VariableDeclaration> for ASTNode {
    fn from(value: VariableDeclaration) -> Self {
        ASTNode::VariableDeclaration(value)
    }
}

impl From<VariableDeclarationStatement> for ASTNode {
    fn from(value: VariableDeclarationStatement) -> Self {
        ASTNode::VariableDeclarationStatement(value)
    }
}

impl From<WhileStatement> for ASTNode {
    fn from(value: WhileStatement) -> Self {
        ASTNode::WhileStatement(value)
    }
}

impl From<&ArrayTypeName> for ASTNode {
    fn from(value: &ArrayTypeName) -> Self {
        ASTNode::ArrayTypeName(value.clone())
    }
}

impl From<&Assignment> for ASTNode {
    fn from(value: &Assignment) -> Self {
        ASTNode::Assignment(value.clone())
    }
}

impl From<&BinaryOperation> for ASTNode {
    fn from(value: &BinaryOperation) -> Self {
        ASTNode::BinaryOperation(value.clone())
    }
}

impl From<&Block> for ASTNode {
    fn from(value: &Block) -> Self {
        ASTNode::Block(value.clone())
    }
}

impl From<&Conditional> for ASTNode {
    fn from(value: &Conditional) -> Self {
        ASTNode::Conditional(value.clone())
    }
}

impl From<&ContractDefinition> for ASTNode {
    fn from(value: &ContractDefinition) -> Self {
        ASTNode::ContractDefinition(value.clone())
    }
}

impl From<&ElementaryTypeName> for ASTNode {
    fn from(value: &ElementaryTypeName) -> Self {
        ASTNode::ElementaryTypeName(value.clone())
    }
}

impl From<&ElementaryTypeNameExpression> for ASTNode {
    fn from(value: &ElementaryTypeNameExpression) -> Self {
        ASTNode::ElementaryTypeNameExpression(value.clone())
    }
}

impl From<&EmitStatement> for ASTNode {
    fn from(value: &EmitStatement) -> Self {
        ASTNode::EmitStatement(value.clone())
    }
}

impl From<&EnumDefinition> for ASTNode {
    fn from(value: &EnumDefinition) -> Self {
        ASTNode::EnumDefinition(value.clone())
    }
}

impl From<&EnumValue> for ASTNode {
    fn from(value: &EnumValue) -> Self {
        ASTNode::EnumValue(value.clone())
    }
}

impl From<&EventDefinition> for ASTNode {
    fn from(value: &EventDefinition) -> Self {
        ASTNode::EventDefinition(value.clone())
    }
}

impl From<&ErrorDefinition> for ASTNode {
    fn from(value: &ErrorDefinition) -> Self {
        ASTNode::ErrorDefinition(value.clone())
    }
}

impl From<&ExpressionStatement> for ASTNode {
    fn from(value: &ExpressionStatement) -> Self {
        ASTNode::ExpressionStatement(value.clone())
    }
}

impl From<&FunctionCall> for ASTNode {
    fn from(value: &FunctionCall) -> Self {
        ASTNode::FunctionCall(value.clone())
    }
}

impl From<&FunctionCallOptions> for ASTNode {
    fn from(value: &FunctionCallOptions) -> Self {
        ASTNode::FunctionCallOptions(value.clone())
    }
}

impl From<&FunctionDefinition> for ASTNode {
    fn from(value: &FunctionDefinition) -> Self {
        ASTNode::FunctionDefinition(value.clone())
    }
}

impl From<&FunctionTypeName> for ASTNode {
    fn from(value: &FunctionTypeName) -> Self {
        ASTNode::FunctionTypeName(value.clone())
    }
}

impl From<&ForStatement> for ASTNode {
    fn from(value: &ForStatement) -> Self {
        ASTNode::ForStatement(value.clone())
    }
}

impl From<&Identifier> for ASTNode {
    fn from(value: &Identifier) -> Self {
        ASTNode::Identifier(value.clone())
    }
}

impl From<&IdentifierPath> for ASTNode {
    fn from(value: &IdentifierPath) -> Self {
        ASTNode::IdentifierPath(value.clone())
    }
}

impl From<&IfStatement> for ASTNode {
    fn from(value: &IfStatement) -> Self {
        ASTNode::IfStatement(value.clone())
    }
}

impl From<&ImportDirective> for ASTNode {
    fn from(value: &ImportDirective) -> Self {
        ASTNode::ImportDirective(value.clone())
    }
}

impl From<&IndexAccess> for ASTNode {
    fn from(value: &IndexAccess) -> Self {
        ASTNode::IndexAccess(value.clone())
    }
}

impl From<&IndexRangeAccess> for ASTNode {
    fn from(value: &IndexRangeAccess) -> Self {
        ASTNode::IndexRangeAccess(value.clone())
    }
}

impl From<&InheritanceSpecifier> for ASTNode {
    fn from(value: &InheritanceSpecifier) -> Self {
        ASTNode::InheritanceSpecifier(value.clone())
    }
}

impl From<&InlineAssembly> for ASTNode {
    fn from(value: &InlineAssembly) -> Self {
        ASTNode::InlineAssembly(value.clone())
    }
}

impl From<&Literal> for ASTNode {
    fn from(value: &Literal) -> Self {
        ASTNode::Literal(value.clone())
    }
}

impl From<&MemberAccess> for ASTNode {
    fn from(value: &MemberAccess) -> Self {
        ASTNode::MemberAccess(value.clone())
    }
}

impl From<&NewExpression> for ASTNode {
    fn from(value: &NewExpression) -> Self {
        ASTNode::NewExpression(value.clone())
    }
}

impl From<&Mapping> for ASTNode {
    fn from(value: &Mapping) -> Self {
        ASTNode::Mapping(value.clone())
    }
}

impl From<&ModifierDefinition> for ASTNode {
    fn from(value: &ModifierDefinition) -> Self {
        ASTNode::ModifierDefinition(value.clone())
    }
}

impl From<&ModifierInvocation> for ASTNode {
    fn from(value: &ModifierInvocation) -> Self {
        ASTNode::ModifierInvocation(value.clone())
    }
}

impl From<&OverrideSpecifier> for ASTNode {
    fn from(value: &OverrideSpecifier) -> Self {
        ASTNode::OverrideSpecifier(value.clone())
    }
}

impl From<&ParameterList> for ASTNode {
    fn from(value: &ParameterList) -> Self {
        ASTNode::ParameterList(value.clone())
    }
}

impl From<&PragmaDirective> for ASTNode {
    fn from(value: &PragmaDirective) -> Self {
        ASTNode::PragmaDirective(value.clone())
    }
}

impl From<&Return> for ASTNode {
    fn from(value: &Return) -> Self {
        ASTNode::Return(value.clone())
    }
}

impl From<&RevertStatement> for ASTNode {
    fn from(value: &RevertStatement) -> Self {
        ASTNode::RevertStatement(value.clone())
    }
}

impl From<&SourceUnit> for ASTNode {
    fn from(value: &SourceUnit) -> Self {
        ASTNode::SourceUnit(value.clone())
    }
}

impl From<&StructDefinition> for ASTNode {
    fn from(value: &StructDefinition) -> Self {
        ASTNode::StructDefinition(value.clone())
    }
}

impl From<&StructuredDocumentation> for ASTNode {
    fn from(value: &StructuredDocumentation) -> Self {
        ASTNode::StructuredDocumentation(value.clone())
    }
}

impl From<&TryStatement> for ASTNode {
    fn from(value: &TryStatement) -> Self {
        ASTNode::TryStatement(value.clone())
    }
}

impl From<&TryCatchClause> for ASTNode {
    fn from(value: &TryCatchClause) -> Self {
        ASTNode::TryCatchClause(value.clone())
    }
}

impl From<&TupleExpression> for ASTNode {
    fn from(value: &TupleExpression) -> Self {
        ASTNode::TupleExpression(value.clone())
    }
}

impl From<&UnaryOperation> for ASTNode {
    fn from(value: &UnaryOperation) -> Self {
        ASTNode::UnaryOperation(value.clone())
    }
}

impl From<&UserDefinedTypeName> for ASTNode {
    fn from(value: &UserDefinedTypeName) -> Self {
        ASTNode::UserDefinedTypeName(value.clone())
    }
}

impl From<&UserDefinedValueTypeDefinition> for ASTNode {
    fn from(value: &UserDefinedValueTypeDefinition) -> Self {
        ASTNode::UserDefinedValueTypeDefinition(value.clone())
    }
}

impl From<&UsingForDirective> for ASTNode {
    fn from(value: &UsingForDirective) -> Self {
        ASTNode::UsingForDirective(value.clone())
    }
}

impl From<&VariableDeclaration> for ASTNode {
    fn from(value: &VariableDeclaration) -> Self {
        ASTNode::VariableDeclaration(value.clone())
    }
}

impl From<&VariableDeclarationStatement> for ASTNode {
    fn from(value: &VariableDeclarationStatement) -> Self {
        ASTNode::VariableDeclarationStatement(value.clone())
    }
}

impl From<&WhileStatement> for ASTNode {
    fn from(value: &WhileStatement) -> Self {
        ASTNode::WhileStatement(value.clone())
    }
}

impl ASTNode {
    pub fn src(&self) -> Option<&str> {
        match self {
            ASTNode::ArrayTypeName(_) => None,
            ASTNode::Assignment(node) => Some(&node.src),
            ASTNode::BinaryOperation(node) => Some(&node.src),
            ASTNode::Block(node) => Some(&node.src),
            ASTNode::Conditional(node) => Some(&node.src),
            ASTNode::ContractDefinition(node) => Some(&node.src),
            ASTNode::ElementaryTypeName(_) => None,
            ASTNode::ElementaryTypeNameExpression(node) => Some(&node.src),
            ASTNode::EmitStatement(_) => None,
            ASTNode::EnumDefinition(node) => Some(&node.src),
            ASTNode::EnumValue(node) => Some(&node.src),
            ASTNode::EventDefinition(node) => Some(&node.src),
            ASTNode::ErrorDefinition(node) => Some(&node.src),
            ASTNode::ExpressionStatement(_) => None,
            ASTNode::FunctionCall(node) => Some(&node.src),
            ASTNode::FunctionCallOptions(node) => Some(&node.src),
            ASTNode::FunctionDefinition(node) => Some(&node.src),
            ASTNode::FunctionTypeName(_) => None,
            ASTNode::ForStatement(node) => Some(&node.src),
            ASTNode::Identifier(node) => Some(&node.src),
            ASTNode::IdentifierPath(node) => Some(&node.src),
            ASTNode::IfStatement(node) => Some(&node.src),
            ASTNode::ImportDirective(node) => Some(&node.src),
            ASTNode::IndexAccess(node) => Some(&node.src),
            ASTNode::IndexRangeAccess(node) => Some(&node.src),
            ASTNode::InheritanceSpecifier(node) => Some(&node.src),
            ASTNode::InlineAssembly(node) => Some(&node.src),
            ASTNode::Literal(node) => Some(&node.src),
            ASTNode::MemberAccess(node) => Some(&node.src),
            ASTNode::NewExpression(node) => Some(&node.src),
            ASTNode::Mapping(_) => None,
            ASTNode::ModifierDefinition(node) => Some(&node.src),
            ASTNode::ModifierInvocation(node) => Some(&node.src),
            ASTNode::OverrideSpecifier(node) => Some(&node.src),
            ASTNode::ParameterList(node) => Some(&node.src),
            ASTNode::PragmaDirective(node) => Some(&node.src),
            ASTNode::Return(node) => Some(&node.src),
            ASTNode::RevertStatement(_) => None,
            ASTNode::SourceUnit(_) => None,
            ASTNode::StructDefinition(node) => Some(&node.src),
            ASTNode::StructuredDocumentation(node) => Some(&node.src),
            ASTNode::TryStatement(_) => None,
            ASTNode::TryCatchClause(_) => None,
            ASTNode::TupleExpression(node) => Some(&node.src),
            ASTNode::UnaryOperation(node) => Some(&node.src),
            ASTNode::UserDefinedTypeName(_) => None,
            ASTNode::UserDefinedValueTypeDefinition(node) => Some(&node.src),
            ASTNode::UsingForDirective(node) => Some(&node.src),
            ASTNode::VariableDeclaration(node) => Some(&node.src),
            ASTNode::VariableDeclarationStatement(node) => Some(&node.src),
            ASTNode::WhileStatement(node) => Some(&node.src),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeContext {
    pub source_unit_id: NodeID,
    pub contract_definition_id: Option<NodeID>,
    pub function_definition_id: Option<NodeID>,
    pub modifier_definition_id: Option<NodeID>,
}

#[derive(Default, Debug)]
pub struct WorkspaceContext {
    last_source_unit_id: NodeID,
    last_contract_definition_id: Option<NodeID>,
    last_function_definition_id: Option<NodeID>,
    last_modifier_definition_id: Option<NodeID>,
    pub parent_link: HashMap<NodeID, NodeID>,

    // relative source filepaths
    pub src_filepaths: Vec<String>,
    pub sloc_stats: HashMap<String, usize>,
    pub nodes: HashMap<NodeID, ASTNode>,

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
    pub(crate) source_units_context: Vec<SourceUnit>,
    pub(crate) struct_definitions_context: HashMap<StructDefinition, NodeContext>,
    pub(crate) structured_documentations_context: HashMap<StructuredDocumentation, NodeContext>,
    pub(crate) try_statements_context: HashMap<TryStatement, NodeContext>,
    pub(crate) try_catch_clauses_context: HashMap<TryCatchClause, NodeContext>,
    pub(crate) tuple_expressions_context: HashMap<TupleExpression, NodeContext>,
    pub(crate) unary_operations_context: HashMap<UnaryOperation, NodeContext>,
    pub(crate) user_defined_type_names_context: HashMap<UserDefinedTypeName, NodeContext>,
    pub(crate) user_defined_value_type_definitions_context:
        HashMap<UserDefinedValueTypeDefinition, NodeContext>,
    pub(crate) using_for_directives_context: HashMap<UsingForDirective, NodeContext>,
    pub(crate) variable_declarations_context: HashMap<VariableDeclaration, NodeContext>,
    pub(crate) variable_declaration_statements_context:
        HashMap<VariableDeclarationStatement, NodeContext>,
    pub(crate) while_statements_context: HashMap<WhileStatement, NodeContext>,
}

impl WorkspaceContext {
    // Setters

    pub fn set_sloc_stats(&mut self, sloc_stats: HashMap<String, usize>) {
        self.sloc_stats = sloc_stats;
    }

    // Getters

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
        self.elementary_type_name_expressions_context
            .keys()
            .collect()
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
    pub fn user_defined_type_names(&self) -> Vec<&UserDefinedTypeName> {
        self.user_defined_type_names_context.keys().collect()
    }
    pub fn user_defined_value_type_definitions(&self) -> Vec<&UserDefinedValueTypeDefinition> {
        self.user_defined_value_type_definitions_context
            .keys()
            .collect()
    }
    pub fn using_for_directives(&self) -> Vec<&UsingForDirective> {
        self.using_for_directives_context.keys().collect()
    }
    pub fn variable_declarations(&self) -> Vec<&VariableDeclaration> {
        self.variable_declarations_context.keys().collect()
    }
    pub fn variable_declaration_statements(&self) -> Vec<&VariableDeclarationStatement> {
        self.variable_declaration_statements_context
            .keys()
            .collect()
    }
    pub fn while_statements(&self) -> Vec<&WhileStatement> {
        self.while_statements_context.keys().collect()
    }

    pub fn get_parent(&self, node_id: NodeID) -> Option<&ASTNode> {
        self.nodes.get(self.parent_link.get(&node_id)?)
    }

    pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
        let source_unit_id = match node {
            ASTNode::ArrayTypeName(node) => self
                .array_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Assignment(node) => self
                .assignments_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::BinaryOperation(node) => self
                .binary_operations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Block(node) => self
                .blocks_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Conditional(node) => self
                .conditionals_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ContractDefinition(node) => self
                .contract_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ElementaryTypeName(node) => self
                .elementary_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ElementaryTypeNameExpression(node) => self
                .elementary_type_name_expressions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EmitStatement(node) => self
                .emit_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EnumDefinition(node) => self
                .enum_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EnumValue(node) => self
                .enum_values_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EventDefinition(node) => self
                .event_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ErrorDefinition(node) => self
                .error_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ExpressionStatement(node) => self
                .expression_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionCall(node) => self
                .function_calls_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionCallOptions(node) => self
                .function_call_options_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionDefinition(node) => self
                .function_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionTypeName(node) => self
                .function_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ForStatement(node) => self
                .for_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Identifier(node) => self
                .identifiers_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IdentifierPath(node) => self
                .identifier_paths_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IfStatement(node) => self
                .if_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ImportDirective(node) => self
                .import_directives_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IndexAccess(node) => self
                .index_accesses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IndexRangeAccess(node) => self
                .index_range_accesses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::InheritanceSpecifier(node) => self
                .inheritance_specifiers_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::InlineAssembly(node) => self
                .inline_assemblies_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Literal(node) => self
                .literals_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::MemberAccess(node) => self
                .member_accesses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::NewExpression(node) => self
                .new_expressions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Mapping(node) => self
                .mappings_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ModifierDefinition(node) => self
                .modifier_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ModifierInvocation(node) => self
                .modifier_invocations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::OverrideSpecifier(node) => self
                .override_specifiers_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ParameterList(node) => self
                .parameter_lists_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::PragmaDirective(node) => self
                .pragma_directives_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Return(node) => self
                .returns_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::RevertStatement(node) => self
                .revert_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::SourceUnit(node) => Some(node.id),
            ASTNode::StructDefinition(node) => self
                .struct_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::StructuredDocumentation(node) => self
                .structured_documentations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TryStatement(node) => self
                .try_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TryCatchClause(node) => self
                .try_catch_clauses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TupleExpression(node) => self
                .tuple_expressions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UnaryOperation(node) => self
                .unary_operations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UserDefinedTypeName(node) => self
                .user_defined_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UserDefinedValueTypeDefinition(node) => self
                .user_defined_value_type_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UsingForDirective(node) => self
                .using_for_directives_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::VariableDeclaration(node) => self
                .variable_declarations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::VariableDeclarationStatement(node) => self
                .variable_declaration_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::WhileStatement(node) => self
                .while_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
        };

        // iterate through self.source_units until the source unit with the id matching `source_unit_id` is found, then return its `absolute_path`

        source_unit_id.and_then(|id| {
            self.source_units_context
                .iter()
                .find(|source_unit| source_unit.id == id)
        })
    }

    pub fn get_node_sort_key(&self, node: &ASTNode) -> (String, usize, String) {
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();
        let source_line = node
            .src()
            .map(|src| source_unit.source_line(src).unwrap_or(0)) // If `src` is `Some`, get the line number, else return 0
            .unwrap_or(0); // If `src` is `None`, default to 0
        let src_location = node.src().unwrap_or("").to_string();

        (absolute_path, source_line, src_location)
    }
}

impl ASTConstVisitor for WorkspaceContext {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.array_type_names_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::Assignment(node.clone()));
        self.assignments_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::BinaryOperation(node.clone()));
        self.binary_operations_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Block(node.clone()));
        self.blocks_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::Conditional(node.clone()));
        self.conditionals_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ContractDefinition(node.clone()));
        self.contract_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        self.last_contract_definition_id = Some(node.id);
        Ok(true)
    }

    fn end_visit_contract_definition(&mut self, _: &ContractDefinition) -> Result<()> {
        self.last_contract_definition_id = None;
        Ok(())
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.elementary_type_names_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ElementaryTypeNameExpression(node.clone()));
        self.elementary_type_name_expressions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        self.emit_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::EnumDefinition(node.clone()));
        self.enum_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EnumValue(node.clone()));
        self.enum_values_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::EventDefinition(node.clone()));
        self.event_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ErrorDefinition(node.clone()));
        self.error_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        self.expression_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::FunctionCall(node.clone()));
        self.function_calls_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::FunctionCallOptions(node.clone()));
        self.function_call_options_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::FunctionDefinition(node.clone()));
        self.function_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        self.last_function_definition_id = Some(node.id);
        Ok(true)
    }

    fn end_visit_function_definition(&mut self, _: &FunctionDefinition) -> Result<()> {
        self.last_function_definition_id = None;
        Ok(())
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.function_type_names_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ForStatement(node.clone()));
        self.for_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::Identifier(node.clone()));
        self.identifiers_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IdentifierPath(node.clone()));
        self.identifier_paths_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IfStatement(node.clone()));
        self.if_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ImportDirective(node.clone()));
        self.import_directives_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IndexAccess(node.clone()));
        self.index_accesses_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IndexRangeAccess(node.clone()));
        self.index_range_accesses_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::InheritanceSpecifier(node.clone()));
        self.inheritance_specifiers_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::InlineAssembly(node.clone()));
        self.inline_assemblies_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Literal(node.clone()));
        self.literals_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::MemberAccess(node.clone()));
        self.member_accesses_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::NewExpression(node.clone()));
        self.new_expressions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.mappings_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ModifierDefinition(node.clone()));
        self.modifier_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        self.last_modifier_definition_id = Some(node.id);
        Ok(true)
    }

    fn end_visit_modifier_definition(&mut self, _: &ModifierDefinition) -> Result<()> {
        self.last_modifier_definition_id = None;
        Ok(())
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ModifierInvocation(node.clone()));
        self.modifier_invocations_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::OverrideSpecifier(node.clone()));
        self.override_specifiers_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ParameterList(node.clone()));
        self.parameter_lists_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::PragmaDirective(node.clone()));
        self.pragma_directives_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Return(node.clone()));
        self.returns_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        self.revert_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::SourceUnit(node.clone()));
        self.source_units_context.push(node.clone());
        self.last_source_unit_id = node.id;
        Ok(true)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::StructDefinition(node.clone()));
        self.struct_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::StructuredDocumentation(node.clone()));
        self.structured_documentations_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        self.try_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        self.try_catch_clauses_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::TupleExpression(node.clone()));
        self.tuple_expressions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::UnaryOperation(node.clone()));
        self.unary_operations_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.user_defined_type_names_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.nodes.insert(
            node.id,
            ASTNode::UserDefinedValueTypeDefinition(node.clone()),
        );
        self.user_defined_value_type_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::UsingForDirective(node.clone()));
        self.using_for_directives_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::VariableDeclaration(node.clone()));
        self.variable_declarations_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::VariableDeclarationStatement(node.clone()));
        self.variable_declaration_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::WhileStatement(node.clone()));
        self.while_statements_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
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

#[cfg(test)]
mod context_tests {
    use crate::ast::*;
    use crate::context::workspace_context::WorkspaceContext;
    use crate::framework::foundry::FoundryOutput;
    use crate::visitor::ast_visitor::*;
    use eyre::Result;

    fn read_compiler_output(filepath: &str) -> Result<FoundryOutput> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(filepath)?,
        ))?)
    }

    #[derive(Default, Debug)]
    pub struct DelegateCallInLoopDetector {
        pub found_delegate_call_in_loop: Vec<MemberAccess>,
    }

    impl ASTConstVisitor for DelegateCallInLoopDetector {
        fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
            if node.member_name == "delegatecall" {
                self.found_delegate_call_in_loop.push(node.clone());
            }
            Ok(true)
        }
    }

    #[test]
    fn test_delegate_call_in_loops() -> Result<()> {
        let mut context = WorkspaceContext::default();
        let extended_inheritance = read_compiler_output(
            "../tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        )?;
        let inheritance_base = read_compiler_output(
            "../tests/contract-playground/out/InheritanceBase.sol/InheritanceBase.0.8.21.json",
        )?;
        let i_contract_inheritance = read_compiler_output(
            "../tests/contract-playground/out/IContractInheritance.sol/IContractInheritance.0.8.21.json",
        )?;
        extended_inheritance.ast.accept(&mut context)?;
        inheritance_base.ast.accept(&mut context)?;
        i_contract_inheritance.ast.accept(&mut context)?;

        // Get all for statements, and check if there is a delegate call in the body of each for statement
        let mut delegate_call_in_loop_detector = DelegateCallInLoopDetector::default();
        let for_statements = context.for_statements_context.keys();
        for for_statement in for_statements {
            for_statement.accept(&mut delegate_call_in_loop_detector)?;
        }
        println!(
            "Found delegate call in loop: {:?}",
            delegate_call_in_loop_detector.found_delegate_call_in_loop
        );

        Ok(())
    }
}
