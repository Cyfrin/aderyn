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
pub struct ContextLoader {
    last_source_unit_id: NodeID,
    last_contract_definition_id: Option<NodeID>,
    last_function_definition_id: Option<NodeID>,
    last_modifier_definition_id: Option<NodeID>,

    // relative source filepaths
    pub src_filepaths: Vec<String>,
    pub sloc_stats: HashMap<String, usize>,
    pub nodes: HashMap<NodeID, ASTNode>,

    // Hashmaps of all nodes => source_unit_id
    pub array_type_names: HashMap<ArrayTypeName, NodeContext>,
    pub assignments: HashMap<Assignment, NodeContext>,
    pub binary_operations: HashMap<BinaryOperation, NodeContext>,
    pub blocks: HashMap<Block, NodeContext>,
    pub conditionals: HashMap<Conditional, NodeContext>,
    pub contract_definitions: HashMap<ContractDefinition, NodeContext>,
    pub elementary_type_names: HashMap<ElementaryTypeName, NodeContext>,
    pub elementary_type_name_expressions: HashMap<ElementaryTypeNameExpression, NodeContext>,
    pub emit_statements: HashMap<EmitStatement, NodeContext>,
    pub enum_definitions: HashMap<EnumDefinition, NodeContext>,
    pub enum_values: HashMap<EnumValue, NodeContext>,
    pub event_definitions: HashMap<EventDefinition, NodeContext>,
    pub error_definitions: HashMap<ErrorDefinition, NodeContext>,
    pub expression_statements: HashMap<ExpressionStatement, NodeContext>,
    pub function_calls: HashMap<FunctionCall, NodeContext>,
    pub function_call_options: HashMap<FunctionCallOptions, NodeContext>,
    pub function_definitions: HashMap<FunctionDefinition, NodeContext>,
    pub function_type_names: HashMap<FunctionTypeName, NodeContext>,
    pub for_statements: HashMap<ForStatement, NodeContext>,
    pub identifiers: HashMap<Identifier, NodeContext>,
    pub identifier_paths: HashMap<IdentifierPath, NodeContext>,
    pub if_statements: HashMap<IfStatement, NodeContext>,
    pub import_directives: HashMap<ImportDirective, NodeContext>,
    pub index_accesses: HashMap<IndexAccess, NodeContext>,
    pub index_range_accesses: HashMap<IndexRangeAccess, NodeContext>,
    pub inheritance_specifiers: HashMap<InheritanceSpecifier, NodeContext>,
    pub inline_assemblies: HashMap<InlineAssembly, NodeContext>,
    pub literals: HashMap<Literal, NodeContext>,
    pub member_accesses: HashMap<MemberAccess, NodeContext>,
    pub new_expressions: HashMap<NewExpression, NodeContext>,
    pub mappings: HashMap<Mapping, NodeContext>,
    pub modifier_definitions: HashMap<ModifierDefinition, NodeContext>,
    pub modifier_invocations: HashMap<ModifierInvocation, NodeContext>,
    pub override_specifiers: HashMap<OverrideSpecifier, NodeContext>,
    pub parameter_lists: HashMap<ParameterList, NodeContext>,
    pub pragma_directives: HashMap<PragmaDirective, NodeContext>,
    pub returns: HashMap<Return, NodeContext>,
    pub revert_statements: HashMap<RevertStatement, NodeContext>,
    pub source_units: Vec<SourceUnit>,
    pub struct_definitions: HashMap<StructDefinition, NodeContext>,
    pub structured_documentations: HashMap<StructuredDocumentation, NodeContext>,
    pub try_statements: HashMap<TryStatement, NodeContext>,
    pub try_catch_clauses: HashMap<TryCatchClause, NodeContext>,
    pub tuple_expressions: HashMap<TupleExpression, NodeContext>,
    pub unary_operations: HashMap<UnaryOperation, NodeContext>,
    pub user_defined_type_names: HashMap<UserDefinedTypeName, NodeContext>,
    pub user_defined_value_type_definitions: HashMap<UserDefinedValueTypeDefinition, NodeContext>,
    pub using_for_directives: HashMap<UsingForDirective, NodeContext>,
    pub variable_declarations: HashMap<VariableDeclaration, NodeContext>,
    pub variable_declaration_statements: HashMap<VariableDeclarationStatement, NodeContext>,
    pub while_statements: HashMap<WhileStatement, NodeContext>,
}

impl ContextLoader {
    // Setters

    pub fn set_sloc_stats(&mut self, sloc_stats: HashMap<String, usize>) {
        self.sloc_stats = sloc_stats;
    }

    // Getters

    pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
        let source_unit_id = match node {
            ASTNode::ArrayTypeName(node) => self
                .array_type_names
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Assignment(node) => self
                .assignments
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::BinaryOperation(node) => self
                .binary_operations
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Block(node) => self.blocks.get(node).map(|context| context.source_unit_id),
            ASTNode::Conditional(node) => self
                .conditionals
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ContractDefinition(node) => self
                .contract_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ElementaryTypeName(node) => self
                .elementary_type_names
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ElementaryTypeNameExpression(node) => self
                .elementary_type_name_expressions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EmitStatement(node) => self
                .emit_statements
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EnumDefinition(node) => self
                .enum_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EnumValue(node) => self
                .enum_values
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EventDefinition(node) => self
                .event_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ErrorDefinition(node) => self
                .error_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ExpressionStatement(node) => self
                .expression_statements
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionCall(node) => self
                .function_calls
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionCallOptions(node) => self
                .function_call_options
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionDefinition(node) => self
                .function_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionTypeName(node) => self
                .function_type_names
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ForStatement(node) => self
                .for_statements
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Identifier(node) => self
                .identifiers
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IdentifierPath(node) => self
                .identifier_paths
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IfStatement(node) => self
                .if_statements
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ImportDirective(node) => self
                .import_directives
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IndexAccess(node) => self
                .index_accesses
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IndexRangeAccess(node) => self
                .index_range_accesses
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::InheritanceSpecifier(node) => self
                .inheritance_specifiers
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::InlineAssembly(node) => self
                .inline_assemblies
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Literal(node) => self
                .literals
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::MemberAccess(node) => self
                .member_accesses
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::NewExpression(node) => self
                .new_expressions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Mapping(node) => self
                .mappings
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ModifierDefinition(node) => self
                .modifier_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ModifierInvocation(node) => self
                .modifier_invocations
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::OverrideSpecifier(node) => self
                .override_specifiers
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ParameterList(node) => self
                .parameter_lists
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::PragmaDirective(node) => self
                .pragma_directives
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Return(node) => self.returns.get(node).map(|context| context.source_unit_id),
            ASTNode::RevertStatement(node) => self
                .revert_statements
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::SourceUnit(node) => Some(node.id),
            ASTNode::StructDefinition(node) => self
                .struct_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::StructuredDocumentation(node) => self
                .structured_documentations
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TryStatement(node) => self
                .try_statements
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TryCatchClause(node) => self
                .try_catch_clauses
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TupleExpression(node) => self
                .tuple_expressions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UnaryOperation(node) => self
                .unary_operations
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UserDefinedTypeName(node) => self
                .user_defined_type_names
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UserDefinedValueTypeDefinition(node) => self
                .user_defined_value_type_definitions
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UsingForDirective(node) => self
                .using_for_directives
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::VariableDeclaration(node) => self
                .variable_declarations
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::VariableDeclarationStatement(node) => self
                .variable_declaration_statements
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::WhileStatement(node) => self
                .while_statements
                .get(node)
                .map(|context| context.source_unit_id),
        };

        // iterate through self.source_units until the source unit with the id matching `source_unit_id` is found, then return its `absolute_path`

        source_unit_id.and_then(|id| {
            self.source_units
                .iter()
                .find(|source_unit| source_unit.id == id)
        })
    }

    pub fn get_node_sort_key(&self, node: &ASTNode) -> (String, usize) {
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();
        let source_line = node
            .src()
            .map(|src| source_unit.source_line(src).unwrap_or(0)) // If `src` is `Some`, get the line number, else return 0
            .unwrap_or(0); // If `src` is `None`, default to 0

        (absolute_path, source_line)
    }
}

impl ASTConstVisitor for ContextLoader {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.array_type_names.insert(
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
        self.assignments.insert(
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
        self.binary_operations.insert(
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
        self.blocks.insert(
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
        self.conditionals.insert(
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
        self.contract_definitions.insert(
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
        self.elementary_type_names.insert(
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
        self.elementary_type_name_expressions.insert(
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
        self.emit_statements.insert(
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
        self.enum_definitions.insert(
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
        self.enum_values.insert(
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
        self.event_definitions.insert(
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
        self.error_definitions.insert(
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
        self.expression_statements.insert(
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
        self.function_calls.insert(
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
        self.function_call_options.insert(
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
        self.function_definitions.insert(
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
        self.function_type_names.insert(
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
        self.for_statements.insert(
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
        self.identifiers.insert(
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
        self.identifier_paths.insert(
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
        self.if_statements.insert(
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
        self.import_directives.insert(
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
        self.index_accesses.insert(
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
        self.index_range_accesses.insert(
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
        self.inheritance_specifiers.insert(
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
        self.inline_assemblies.insert(
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
        self.literals.insert(
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
        self.member_accesses.insert(
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
        self.new_expressions.insert(
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
        self.mappings.insert(
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
        self.modifier_definitions.insert(
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
        self.modifier_invocations.insert(
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
        self.override_specifiers.insert(
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
        self.parameter_lists.insert(
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
        self.pragma_directives.insert(
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
        self.returns.insert(
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
        self.revert_statements.insert(
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
        self.source_units.push(node.clone());
        self.last_source_unit_id = node.id;
        Ok(true)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::StructDefinition(node.clone()));
        self.struct_definitions.insert(
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
        self.structured_documentations.insert(
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
        self.try_statements.insert(
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
        self.try_catch_clauses.insert(
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
        self.tuple_expressions.insert(
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
        self.unary_operations.insert(
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
        self.user_defined_type_names.insert(
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
        self.user_defined_value_type_definitions.insert(
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
        self.using_for_directives.insert(
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
        self.variable_declarations.insert(
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
        self.variable_declaration_statements.insert(
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
        self.while_statements.insert(
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
}

#[cfg(test)]
mod loader_tests {
    use crate::ast::*;
    use crate::context::loader::ContextLoader;
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
        let mut loader = ContextLoader::default();
        let extended_inheritance = read_compiler_output(
            "../tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        )?;
        let inheritance_base = read_compiler_output(
            "../tests/contract-playground/out/InheritanceBase.sol/InheritanceBase.0.8.21.json",
        )?;
        let i_contract_inheritance = read_compiler_output(
            "../tests/contract-playground/out/IContractInheritance.sol/IContractInheritance.0.8.21.json",
        )?;
        extended_inheritance.ast.accept(&mut loader)?;
        inheritance_base.ast.accept(&mut loader)?;
        i_contract_inheritance.ast.accept(&mut loader)?;

        // Get all for statements, and check if there is a delegate call in the body of each for statement
        let mut delegate_call_in_loop_detector = DelegateCallInLoopDetector::default();
        let for_statements = loader.for_statements.keys();
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
