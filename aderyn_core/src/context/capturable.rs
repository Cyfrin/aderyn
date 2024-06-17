use crate::ast::{ContractDefinition, *};

use super::workspace_context::{ASTNode, WorkspaceContext};

#[derive(Clone)]
pub enum Capturable {
    ASTNode(ASTNode),
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Block(Block),
    Conditional(Conditional),
    ContractDefinition(contracts::ContractDefinition),
    ASTContractDefinition(ContractDefinition),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    FunctionDefinition(FunctionDefinition),
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
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ParameterList(ParameterList),
    PragmaDirective(PragmaDirective),
    Return(Return),
    SourceUnit(SourceUnit),
    StructDefinition(StructDefinition),
    StructuredDocumentation(StructuredDocumentation),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    BreakStatement(Break),
    ContinueStatement(Continue),
    PlaceholderStatement(PlaceholderStatement),
}

impl Capturable {
    pub fn make_key(&self, context: &WorkspaceContext) -> (String, usize, String) {
        match self {
            Self::ASTNode(node) => context.get_node_sort_key(node),
            Self::Assignment(n) => context.get_node_sort_key(&n.into()),
            Self::BinaryOperation(n) => context.get_node_sort_key(&n.into()),
            Self::Block(n) => context.get_node_sort_key(&n.into()),
            Self::Conditional(n) => context.get_node_sort_key(&n.into()),
            Self::ContractDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::ASTContractDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::ElementaryTypeNameExpression(n) => context.get_node_sort_key(&n.into()),
            Self::EnumDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::EnumValue(n) => context.get_node_sort_key(&n.into()),
            Self::EventDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::ErrorDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::FunctionCall(n) => context.get_node_sort_key(&n.into()),
            Self::FunctionCallOptions(n) => context.get_node_sort_key(&n.into()),
            Self::FunctionDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::ForStatement(n) => context.get_node_sort_key(&n.into()),
            Self::Identifier(n) => context.get_node_sort_key(&n.into()),
            Self::IdentifierPath(n) => context.get_node_sort_key(&n.into()),
            Self::IfStatement(n) => context.get_node_sort_key(&n.into()),
            Self::ImportDirective(n) => context.get_node_sort_key(&n.into()),
            Self::IndexAccess(n) => context.get_node_sort_key(&n.into()),
            Self::IndexRangeAccess(n) => context.get_node_sort_key(&n.into()),
            Self::InheritanceSpecifier(n) => context.get_node_sort_key(&n.into()),
            Self::InlineAssembly(n) => context.get_node_sort_key(&n.into()),
            Self::Literal(n) => context.get_node_sort_key(&n.into()),
            Self::MemberAccess(n) => context.get_node_sort_key(&n.into()),
            Self::NewExpression(n) => context.get_node_sort_key(&n.into()),
            Self::ModifierDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::ModifierInvocation(n) => context.get_node_sort_key(&n.into()),
            Self::OverrideSpecifier(n) => context.get_node_sort_key(&n.into()),
            Self::ParameterList(n) => context.get_node_sort_key(&n.into()),
            Self::PragmaDirective(n) => context.get_node_sort_key(&n.into()),
            Self::Return(n) => context.get_node_sort_key(&n.into()),
            Self::SourceUnit(n) => context.get_node_sort_key(&n.into()),
            Self::StructDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::StructuredDocumentation(n) => context.get_node_sort_key(&n.into()),
            Self::TupleExpression(n) => context.get_node_sort_key(&n.into()),
            Self::UnaryOperation(n) => context.get_node_sort_key(&n.into()),
            Self::UserDefinedValueTypeDefinition(n) => context.get_node_sort_key(&n.into()),
            Self::UsingForDirective(n) => context.get_node_sort_key(&n.into()),
            Self::VariableDeclaration(n) => context.get_node_sort_key(&n.into()),
            Self::VariableDeclarationStatement(n) => context.get_node_sort_key(&n.into()),
            Self::WhileStatement(n) => context.get_node_sort_key(&n.into()),
            Self::DoWhileStatement(n) => context.get_node_sort_key(&n.into()),
            Self::BreakStatement(n) => context.get_node_sort_key(&n.into()),
            Self::ContinueStatement(n) => context.get_node_sort_key(&n.into()),
            Self::PlaceholderStatement(n) => context.get_node_sort_key(&n.into()),
        }
    }

    pub fn id(&self) -> Option<NodeID> {
        match self {
            Self::ASTNode(ast_node) => ast_node.id(),
            Self::Assignment(n) => Some(n.id),
            Self::BinaryOperation(n) => Some(n.id),
            Self::Block(n) => Some(n.id),
            Self::Conditional(n) => Some(n.id),
            Self::ContractDefinition(n) => Some(n.id),
            Self::ASTContractDefinition(n) => Some(n.id),
            Self::ElementaryTypeNameExpression(n) => Some(n.id),
            Self::EnumDefinition(n) => Some(n.id),
            Self::EnumValue(n) => Some(n.id),
            Self::EventDefinition(n) => Some(n.id),
            Self::ErrorDefinition(n) => Some(n.id),
            Self::FunctionCall(n) => Some(n.id),
            Self::FunctionCallOptions(n) => Some(n.id),
            Self::FunctionDefinition(n) => Some(n.id),
            Self::ForStatement(n) => Some(n.id),
            Self::Identifier(n) => Some(n.id),
            Self::IdentifierPath(n) => Some(n.id),
            Self::IfStatement(n) => Some(n.id),
            Self::ImportDirective(n) => Some(n.id),
            Self::IndexAccess(n) => Some(n.id),
            Self::IndexRangeAccess(n) => Some(n.id),
            Self::InheritanceSpecifier(n) => Some(n.id),
            Self::InlineAssembly(n) => Some(n.id),
            Self::Literal(n) => Some(n.id),
            Self::MemberAccess(n) => Some(n.id),
            Self::NewExpression(n) => Some(n.id),
            Self::ModifierDefinition(n) => Some(n.id),
            Self::ModifierInvocation(n) => Some(n.id),
            Self::OverrideSpecifier(n) => Some(n.id),
            Self::ParameterList(n) => Some(n.id),
            Self::PragmaDirective(n) => Some(n.id),
            Self::Return(n) => Some(n.id),
            Self::SourceUnit(n) => Some(n.id),
            Self::StructDefinition(n) => Some(n.id),
            Self::StructuredDocumentation(n) => Some(n.id),
            Self::TupleExpression(n) => Some(n.id),
            Self::UnaryOperation(n) => Some(n.id),
            Self::UserDefinedValueTypeDefinition(n) => Some(n.id),
            Self::UsingForDirective(n) => Some(n.id),
            Self::VariableDeclaration(n) => Some(n.id),
            Self::VariableDeclarationStatement(n) => Some(n.id),
            Self::WhileStatement(n) => Some(n.id),
            Self::DoWhileStatement(n) => Some(n.id),
            Self::BreakStatement(n) => Some(n.id),
            Self::ContinueStatement(n) => Some(n.id),
            Self::PlaceholderStatement(n) => Some(n.id),
        }
    }
}

impl From<Assignment> for Capturable {
    fn from(value: Assignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<&Assignment> for Capturable {
    fn from(value: &Assignment) -> Self {
        Self::Assignment(value.clone())
    }
}

impl From<BinaryOperation> for Capturable {
    fn from(value: BinaryOperation) -> Self {
        Self::BinaryOperation(value)
    }
}

impl From<&BinaryOperation> for Capturable {
    fn from(value: &BinaryOperation) -> Self {
        Self::BinaryOperation(value.clone())
    }
}

impl From<Block> for Capturable {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}

impl From<&Block> for Capturable {
    fn from(value: &Block) -> Self {
        Self::Block(value.clone())
    }
}

impl From<Conditional> for Capturable {
    fn from(value: Conditional) -> Self {
        Self::Conditional(value)
    }
}

impl From<&Conditional> for Capturable {
    fn from(value: &Conditional) -> Self {
        Self::Conditional(value.clone())
    }
}

impl From<contracts::ContractDefinition> for Capturable {
    fn from(value: contracts::ContractDefinition) -> Self {
        Self::ContractDefinition(value)
    }
}

impl From<&contracts::ContractDefinition> for Capturable {
    fn from(value: &contracts::ContractDefinition) -> Self {
        Self::ContractDefinition(value.clone())
    }
}

impl From<&&contracts::ContractDefinition> for Capturable {
    fn from(value: &&contracts::ContractDefinition) -> Self {
        #[allow(suspicious_double_ref_op)]
        Self::ContractDefinition(value.clone().clone())
    }
}

impl From<ElementaryTypeNameExpression> for Capturable {
    fn from(value: ElementaryTypeNameExpression) -> Self {
        Self::ElementaryTypeNameExpression(value)
    }
}

impl From<&ElementaryTypeNameExpression> for Capturable {
    fn from(value: &ElementaryTypeNameExpression) -> Self {
        Self::ElementaryTypeNameExpression(value.clone())
    }
}

impl From<EnumDefinition> for Capturable {
    fn from(value: EnumDefinition) -> Self {
        Self::EnumDefinition(value)
    }
}

impl From<&EnumDefinition> for Capturable {
    fn from(value: &EnumDefinition) -> Self {
        Self::EnumDefinition(value.clone())
    }
}

impl From<EnumValue> for Capturable {
    fn from(value: EnumValue) -> Self {
        Self::EnumValue(value)
    }
}

impl From<&EnumValue> for Capturable {
    fn from(value: &EnumValue) -> Self {
        Self::EnumValue(value.clone())
    }
}

impl From<EventDefinition> for Capturable {
    fn from(value: EventDefinition) -> Self {
        Self::EventDefinition(value)
    }
}

impl From<&EventDefinition> for Capturable {
    fn from(value: &EventDefinition) -> Self {
        Self::EventDefinition(value.clone())
    }
}

impl From<ErrorDefinition> for Capturable {
    fn from(value: ErrorDefinition) -> Self {
        Self::ErrorDefinition(value)
    }
}

impl From<&ErrorDefinition> for Capturable {
    fn from(value: &ErrorDefinition) -> Self {
        Self::ErrorDefinition(value.clone())
    }
}

impl From<FunctionCall> for Capturable {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}

impl From<&FunctionCall> for Capturable {
    fn from(value: &FunctionCall) -> Self {
        Self::FunctionCall(value.clone())
    }
}

impl From<FunctionCallOptions> for Capturable {
    fn from(value: FunctionCallOptions) -> Self {
        Self::FunctionCallOptions(value)
    }
}

impl From<&FunctionCallOptions> for Capturable {
    fn from(value: &FunctionCallOptions) -> Self {
        Self::FunctionCallOptions(value.clone())
    }
}

impl From<FunctionDefinition> for Capturable {
    fn from(value: FunctionDefinition) -> Self {
        Self::FunctionDefinition(value)
    }
}

impl From<&FunctionDefinition> for Capturable {
    fn from(value: &FunctionDefinition) -> Self {
        Self::FunctionDefinition(value.clone())
    }
}

impl From<ForStatement> for Capturable {
    fn from(value: ForStatement) -> Self {
        Self::ForStatement(value)
    }
}

impl From<&ForStatement> for Capturable {
    fn from(value: &ForStatement) -> Self {
        Self::ForStatement(value.clone())
    }
}

impl From<Identifier> for Capturable {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}

impl From<&Identifier> for Capturable {
    fn from(value: &Identifier) -> Self {
        Self::Identifier(value.clone())
    }
}

impl From<IdentifierPath> for Capturable {
    fn from(value: IdentifierPath) -> Self {
        Self::IdentifierPath(value)
    }
}

impl From<&IdentifierPath> for Capturable {
    fn from(value: &IdentifierPath) -> Self {
        Self::IdentifierPath(value.clone())
    }
}

impl From<IfStatement> for Capturable {
    fn from(value: IfStatement) -> Self {
        Self::IfStatement(value)
    }
}

impl From<&IfStatement> for Capturable {
    fn from(value: &IfStatement) -> Self {
        Self::IfStatement(value.clone())
    }
}

impl From<ImportDirective> for Capturable {
    fn from(value: ImportDirective) -> Self {
        Self::ImportDirective(value)
    }
}

impl From<&ImportDirective> for Capturable {
    fn from(value: &ImportDirective) -> Self {
        Self::ImportDirective(value.clone())
    }
}

impl From<IndexAccess> for Capturable {
    fn from(value: IndexAccess) -> Self {
        Self::IndexAccess(value)
    }
}

impl From<&IndexAccess> for Capturable {
    fn from(value: &IndexAccess) -> Self {
        Self::IndexAccess(value.clone())
    }
}

impl From<IndexRangeAccess> for Capturable {
    fn from(value: IndexRangeAccess) -> Self {
        Self::IndexRangeAccess(value)
    }
}

impl From<&IndexRangeAccess> for Capturable {
    fn from(value: &IndexRangeAccess) -> Self {
        Self::IndexRangeAccess(value.clone())
    }
}

impl From<InheritanceSpecifier> for Capturable {
    fn from(value: InheritanceSpecifier) -> Self {
        Self::InheritanceSpecifier(value)
    }
}

impl From<&InheritanceSpecifier> for Capturable {
    fn from(value: &InheritanceSpecifier) -> Self {
        Self::InheritanceSpecifier(value.clone())
    }
}

impl From<InlineAssembly> for Capturable {
    fn from(value: InlineAssembly) -> Self {
        Self::InlineAssembly(value)
    }
}

impl From<&InlineAssembly> for Capturable {
    fn from(value: &InlineAssembly) -> Self {
        Self::InlineAssembly(value.clone())
    }
}

impl From<Literal> for Capturable {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl From<&Literal> for Capturable {
    fn from(value: &Literal) -> Self {
        Self::Literal(value.clone())
    }
}

impl From<MemberAccess> for Capturable {
    fn from(value: MemberAccess) -> Self {
        Self::MemberAccess(value)
    }
}

impl From<&MemberAccess> for Capturable {
    fn from(value: &MemberAccess) -> Self {
        Self::MemberAccess(value.clone())
    }
}

impl From<NewExpression> for Capturable {
    fn from(value: NewExpression) -> Self {
        Self::NewExpression(value)
    }
}

impl From<&NewExpression> for Capturable {
    fn from(value: &NewExpression) -> Self {
        Self::NewExpression(value.clone())
    }
}

impl From<ModifierDefinition> for Capturable {
    fn from(value: ModifierDefinition) -> Self {
        Self::ModifierDefinition(value)
    }
}

impl From<&ModifierDefinition> for Capturable {
    fn from(value: &ModifierDefinition) -> Self {
        Self::ModifierDefinition(value.clone())
    }
}

impl From<&&modifiers::ModifierInvocation> for Capturable {
    fn from(value: &&modifiers::ModifierInvocation) -> Self {
        #[allow(suspicious_double_ref_op)]
        Self::ModifierInvocation(value.clone().clone())
    }
}

impl From<ModifierInvocation> for Capturable {
    fn from(value: ModifierInvocation) -> Self {
        Self::ModifierInvocation(value)
    }
}

impl From<&ModifierInvocation> for Capturable {
    fn from(value: &ModifierInvocation) -> Self {
        Self::ModifierInvocation(value.clone())
    }
}

impl From<OverrideSpecifier> for Capturable {
    fn from(value: OverrideSpecifier) -> Self {
        Self::OverrideSpecifier(value)
    }
}

impl From<&OverrideSpecifier> for Capturable {
    fn from(value: &OverrideSpecifier) -> Self {
        Self::OverrideSpecifier(value.clone())
    }
}

impl From<ParameterList> for Capturable {
    fn from(value: ParameterList) -> Self {
        Self::ParameterList(value)
    }
}

impl From<&ParameterList> for Capturable {
    fn from(value: &ParameterList) -> Self {
        Self::ParameterList(value.clone())
    }
}

impl From<PragmaDirective> for Capturable {
    fn from(value: PragmaDirective) -> Self {
        Self::PragmaDirective(value)
    }
}

impl From<&PragmaDirective> for Capturable {
    fn from(value: &PragmaDirective) -> Self {
        Self::PragmaDirective(value.clone())
    }
}

impl From<Return> for Capturable {
    fn from(value: Return) -> Self {
        Self::Return(value)
    }
}

impl From<&Return> for Capturable {
    fn from(value: &Return) -> Self {
        Self::Return(value.clone())
    }
}

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

impl From<StructDefinition> for Capturable {
    fn from(value: StructDefinition) -> Self {
        Self::StructDefinition(value)
    }
}

impl From<&StructDefinition> for Capturable {
    fn from(value: &StructDefinition) -> Self {
        Self::StructDefinition(value.clone())
    }
}

impl From<StructuredDocumentation> for Capturable {
    fn from(value: StructuredDocumentation) -> Self {
        Self::StructuredDocumentation(value)
    }
}

impl From<&StructuredDocumentation> for Capturable {
    fn from(value: &StructuredDocumentation) -> Self {
        Self::StructuredDocumentation(value.clone())
    }
}

impl From<TupleExpression> for Capturable {
    fn from(value: TupleExpression) -> Self {
        Self::TupleExpression(value)
    }
}

impl From<&TupleExpression> for Capturable {
    fn from(value: &TupleExpression) -> Self {
        Self::TupleExpression(value.clone())
    }
}

impl From<UnaryOperation> for Capturable {
    fn from(value: UnaryOperation) -> Self {
        Self::UnaryOperation(value)
    }
}

impl From<&UnaryOperation> for Capturable {
    fn from(value: &UnaryOperation) -> Self {
        Self::UnaryOperation(value.clone())
    }
}

impl From<UserDefinedValueTypeDefinition> for Capturable {
    fn from(value: UserDefinedValueTypeDefinition) -> Self {
        Self::UserDefinedValueTypeDefinition(value)
    }
}

impl From<&UserDefinedValueTypeDefinition> for Capturable {
    fn from(value: &UserDefinedValueTypeDefinition) -> Self {
        Self::UserDefinedValueTypeDefinition(value.clone())
    }
}

impl From<UsingForDirective> for Capturable {
    fn from(value: UsingForDirective) -> Self {
        Self::UsingForDirective(value)
    }
}

impl From<&UsingForDirective> for Capturable {
    fn from(value: &UsingForDirective) -> Self {
        Self::UsingForDirective(value.clone())
    }
}

impl From<VariableDeclaration> for Capturable {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}

impl From<&VariableDeclaration> for Capturable {
    fn from(value: &VariableDeclaration) -> Self {
        Self::VariableDeclaration(value.clone())
    }
}

impl From<VariableDeclarationStatement> for Capturable {
    fn from(value: VariableDeclarationStatement) -> Self {
        Self::VariableDeclarationStatement(value)
    }
}

impl From<&VariableDeclarationStatement> for Capturable {
    fn from(value: &VariableDeclarationStatement) -> Self {
        Self::VariableDeclarationStatement(value.clone())
    }
}

impl From<WhileStatement> for Capturable {
    fn from(value: WhileStatement) -> Self {
        Self::WhileStatement(value)
    }
}

impl From<&WhileStatement> for Capturable {
    fn from(value: &WhileStatement) -> Self {
        Self::WhileStatement(value.clone())
    }
}

impl From<DoWhileStatement> for Capturable {
    fn from(value: DoWhileStatement) -> Self {
        Self::DoWhileStatement(value)
    }
}

impl From<&DoWhileStatement> for Capturable {
    fn from(value: &DoWhileStatement) -> Self {
        Self::DoWhileStatement(value.clone())
    }
}

impl From<Break> for Capturable {
    fn from(value: Break) -> Self {
        Self::BreakStatement(value)
    }
}

impl From<&Break> for Capturable {
    fn from(value: &Break) -> Self {
        Self::BreakStatement(value.clone())
    }
}

impl From<Continue> for Capturable {
    fn from(value: Continue) -> Self {
        Self::ContinueStatement(value)
    }
}

impl From<&Continue> for Capturable {
    fn from(value: &Continue) -> Self {
        Self::ContinueStatement(value.clone())
    }
}

impl From<PlaceholderStatement> for Capturable {
    fn from(value: PlaceholderStatement) -> Self {
        Self::PlaceholderStatement(value)
    }
}

impl From<&PlaceholderStatement> for Capturable {
    fn from(value: &PlaceholderStatement) -> Self {
        Self::PlaceholderStatement(value.clone())
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
