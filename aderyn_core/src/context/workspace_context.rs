use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::browser::GetImmediateParent;
use super::capturable::Capturable;

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
    DoWhileStatement(DoWhileStatement),
    BreakStatement(Break),
    ContinueStatement(Continue),
    PlaceholderStatement(PlaceholderStatement),
}

impl ASTNode {
    pub fn node_type(&self) -> NodeType {
        match self {
            ASTNode::ArrayTypeName(_) => NodeType::ArrayTypeName,
            ASTNode::Assignment(_) => NodeType::Assignment,
            ASTNode::BinaryOperation(_) => NodeType::BinaryOperation,
            ASTNode::Block(_) => NodeType::Block,
            ASTNode::Conditional(_) => NodeType::Conditional,
            ASTNode::ContractDefinition(_) => NodeType::ContractDefinition,
            ASTNode::ElementaryTypeName(_) => NodeType::ElementaryTypeName,
            ASTNode::ElementaryTypeNameExpression(_) => NodeType::ElementaryTypeNameExpression,
            ASTNode::EmitStatement(_) => NodeType::EmitStatement,
            ASTNode::EnumDefinition(_) => NodeType::EnumDefinition,
            ASTNode::EnumValue(_) => NodeType::EnumValue,
            ASTNode::EventDefinition(_) => NodeType::EventDefinition,
            ASTNode::ErrorDefinition(_) => NodeType::ErrorDefinition,
            ASTNode::ExpressionStatement(_) => NodeType::ExpressionStatement,
            ASTNode::FunctionCall(_) => NodeType::FunctionCall,
            ASTNode::FunctionCallOptions(_) => NodeType::FunctionCallOptions,
            ASTNode::FunctionDefinition(_) => NodeType::FunctionDefinition,
            ASTNode::FunctionTypeName(_) => NodeType::FunctionTypeName,
            ASTNode::ForStatement(_) => NodeType::ForStatement,
            ASTNode::Identifier(_) => NodeType::Identifier,
            ASTNode::IdentifierPath(_) => NodeType::IdentifierPath,
            ASTNode::IfStatement(_) => NodeType::IfStatement,
            ASTNode::ImportDirective(_) => NodeType::ImportDirective,
            ASTNode::IndexAccess(_) => NodeType::IndexAccess,
            ASTNode::IndexRangeAccess(_) => NodeType::IndexRangeAccess,
            ASTNode::InheritanceSpecifier(_) => NodeType::InheritanceSpecifier,
            ASTNode::InlineAssembly(_) => NodeType::InlineAssembly,
            ASTNode::Literal(_) => NodeType::Literal,
            ASTNode::MemberAccess(_) => NodeType::MemberAccess,
            ASTNode::NewExpression(_) => NodeType::NewExpression,
            ASTNode::Mapping(_) => NodeType::Mapping,
            ASTNode::ModifierDefinition(_) => NodeType::ModifierDefinition,
            ASTNode::ModifierInvocation(_) => NodeType::ModifierInvocation,
            ASTNode::OverrideSpecifier(_) => NodeType::OverrideSpecifier,
            ASTNode::ParameterList(_) => NodeType::ParameterList,
            ASTNode::PragmaDirective(_) => NodeType::PragmaDirective,
            ASTNode::Return(_) => NodeType::Return,
            ASTNode::RevertStatement(_) => NodeType::RevertStatement,
            ASTNode::SourceUnit(_) => NodeType::SourceUnit,
            ASTNode::StructDefinition(_) => NodeType::StructDefinition,
            ASTNode::StructuredDocumentation(_) => NodeType::StructuredDocumentation,
            ASTNode::TryStatement(_) => NodeType::TryStatement,
            ASTNode::TryCatchClause(_) => NodeType::TryCatchClause,
            ASTNode::TupleExpression(_) => NodeType::TupleExpression,
            ASTNode::UnaryOperation(_) => NodeType::UnaryOperation,
            ASTNode::UserDefinedTypeName(_) => NodeType::UserDefinedTypeName,
            ASTNode::UserDefinedValueTypeDefinition(_) => NodeType::UserDefinedValueTypeDefinition,
            ASTNode::UsingForDirective(_) => NodeType::UsingForDirective,
            ASTNode::VariableDeclaration(_) => NodeType::VariableDeclaration,
            ASTNode::VariableDeclarationStatement(_) => NodeType::VariableDeclarationStatement,
            ASTNode::WhileStatement(_) => NodeType::WhileStatement,
            ASTNode::DoWhileStatement(_) => NodeType::DoWhileStatement,
            ASTNode::BreakStatement(_) => NodeType::Break,
            ASTNode::ContinueStatement(_) => NodeType::Continue,
            ASTNode::PlaceholderStatement(_) => NodeType::PlaceholderStatement,
        }
    }

    pub fn id(&self) -> Option<NodeID> {
        match self {
            ASTNode::ArrayTypeName(_) => None,
            ASTNode::Assignment(n) => Some(n.id),
            ASTNode::BinaryOperation(n) => Some(n.id),
            ASTNode::Block(n) => Some(n.id),
            ASTNode::Conditional(n) => Some(n.id),
            ASTNode::ContractDefinition(n) => Some(n.id),
            ASTNode::ElementaryTypeName(_) => None,
            ASTNode::ElementaryTypeNameExpression(n) => Some(n.id),
            ASTNode::EmitStatement(_) => None,
            ASTNode::EnumDefinition(n) => Some(n.id),
            ASTNode::EnumValue(n) => Some(n.id),
            ASTNode::EventDefinition(n) => Some(n.id),
            ASTNode::ErrorDefinition(n) => Some(n.id),
            ASTNode::ExpressionStatement(_) => None,
            ASTNode::FunctionCall(n) => Some(n.id),
            ASTNode::FunctionCallOptions(n) => Some(n.id),
            ASTNode::FunctionDefinition(n) => Some(n.id),
            ASTNode::FunctionTypeName(_) => None,
            ASTNode::ForStatement(n) => Some(n.id),
            ASTNode::Identifier(n) => Some(n.id),
            ASTNode::IdentifierPath(n) => Some(n.id),
            ASTNode::IfStatement(n) => Some(n.id),
            ASTNode::ImportDirective(n) => Some(n.id),
            ASTNode::IndexAccess(n) => Some(n.id),
            ASTNode::IndexRangeAccess(n) => Some(n.id),
            ASTNode::InheritanceSpecifier(n) => Some(n.id),
            ASTNode::InlineAssembly(n) => Some(n.id),
            ASTNode::Literal(n) => Some(n.id),
            ASTNode::MemberAccess(n) => Some(n.id),
            ASTNode::NewExpression(n) => Some(n.id),
            ASTNode::Mapping(_n) => None,
            ASTNode::ModifierDefinition(n) => Some(n.id),
            ASTNode::ModifierInvocation(n) => Some(n.id),
            ASTNode::OverrideSpecifier(n) => Some(n.id),
            ASTNode::ParameterList(n) => Some(n.id),
            ASTNode::PragmaDirective(n) => Some(n.id),
            ASTNode::Return(n) => Some(n.id),
            ASTNode::RevertStatement(_) => None,
            ASTNode::SourceUnit(n) => Some(n.id),
            ASTNode::StructDefinition(n) => Some(n.id),
            ASTNode::StructuredDocumentation(n) => Some(n.id),
            ASTNode::TryStatement(_) => None,
            ASTNode::TryCatchClause(_) => None,
            ASTNode::TupleExpression(n) => Some(n.id),
            ASTNode::UnaryOperation(n) => Some(n.id),
            ASTNode::UserDefinedTypeName(_) => None,
            ASTNode::UserDefinedValueTypeDefinition(n) => Some(n.id),
            ASTNode::UsingForDirective(n) => Some(n.id),
            ASTNode::VariableDeclaration(n) => Some(n.id),
            ASTNode::VariableDeclarationStatement(n) => Some(n.id),
            ASTNode::WhileStatement(n) => Some(n.id),
            ASTNode::DoWhileStatement(n) => Some(n.id),
            ASTNode::BreakStatement(n) => Some(n.id),
            ASTNode::ContinueStatement(n) => Some(n.id),
            ASTNode::PlaceholderStatement(n) => Some(n.id),
        }
    }
}

impl Node for ASTNode {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
        match self {
            ASTNode::ArrayTypeName(n) => n.accept(visitor),
            ASTNode::Assignment(n) => n.accept(visitor),
            ASTNode::BinaryOperation(n) => n.accept(visitor),
            ASTNode::Block(n) => n.accept(visitor),
            ASTNode::Conditional(n) => n.accept(visitor),
            ASTNode::ContractDefinition(n) => n.accept(visitor),
            ASTNode::ElementaryTypeName(n) => n.accept(visitor),
            ASTNode::ElementaryTypeNameExpression(n) => n.accept(visitor),
            ASTNode::EmitStatement(n) => n.accept(visitor),
            ASTNode::EnumDefinition(n) => n.accept(visitor),
            ASTNode::EnumValue(n) => n.accept(visitor),
            ASTNode::EventDefinition(n) => n.accept(visitor),
            ASTNode::ErrorDefinition(n) => n.accept(visitor),
            ASTNode::ExpressionStatement(n) => n.accept(visitor),
            ASTNode::FunctionCall(n) => n.accept(visitor),
            ASTNode::FunctionCallOptions(n) => n.accept(visitor),
            ASTNode::FunctionDefinition(n) => n.accept(visitor),
            ASTNode::FunctionTypeName(n) => n.accept(visitor),
            ASTNode::ForStatement(n) => n.accept(visitor),
            ASTNode::Identifier(n) => n.accept(visitor),
            ASTNode::IdentifierPath(n) => n.accept(visitor),
            ASTNode::IfStatement(n) => n.accept(visitor),
            ASTNode::ImportDirective(n) => n.accept(visitor),
            ASTNode::IndexAccess(n) => n.accept(visitor),
            ASTNode::IndexRangeAccess(n) => n.accept(visitor),
            ASTNode::InheritanceSpecifier(n) => n.accept(visitor),
            ASTNode::InlineAssembly(n) => n.accept(visitor),
            ASTNode::Literal(n) => n.accept(visitor),
            ASTNode::MemberAccess(n) => n.accept(visitor),
            ASTNode::NewExpression(n) => n.accept(visitor),
            ASTNode::Mapping(n) => n.accept(visitor),
            ASTNode::ModifierDefinition(n) => n.accept(visitor),
            ASTNode::ModifierInvocation(n) => n.accept(visitor),
            ASTNode::OverrideSpecifier(n) => n.accept(visitor),
            ASTNode::ParameterList(n) => n.accept(visitor),
            ASTNode::PragmaDirective(n) => n.accept(visitor),
            ASTNode::Return(n) => n.accept(visitor),
            ASTNode::RevertStatement(n) => n.accept(visitor),
            ASTNode::SourceUnit(n) => n.accept(visitor),
            ASTNode::StructDefinition(n) => n.accept(visitor),
            ASTNode::StructuredDocumentation(n) => n.accept(visitor),
            ASTNode::TryStatement(n) => n.accept(visitor),
            ASTNode::TryCatchClause(n) => n.accept(visitor),
            ASTNode::TupleExpression(n) => n.accept(visitor),
            ASTNode::UnaryOperation(n) => n.accept(visitor),
            ASTNode::UserDefinedTypeName(n) => n.accept(visitor),
            ASTNode::UserDefinedValueTypeDefinition(n) => n.accept(visitor),
            ASTNode::UsingForDirective(n) => n.accept(visitor),
            ASTNode::VariableDeclaration(n) => n.accept(visitor),
            ASTNode::VariableDeclarationStatement(n) => n.accept(visitor),
            ASTNode::WhileStatement(n) => n.accept(visitor),
            ASTNode::DoWhileStatement(n) => n.accept(visitor),
            ASTNode::BreakStatement(n) => n.accept(visitor),
            ASTNode::ContinueStatement(n) => n.accept(visitor),
            ASTNode::PlaceholderStatement(n) => n.accept(visitor),
        }
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
        match self {
            ASTNode::ArrayTypeName(n) => n.accept_metadata(visitor),
            ASTNode::Assignment(n) => n.accept_metadata(visitor),
            ASTNode::BinaryOperation(n) => n.accept_metadata(visitor),
            ASTNode::Block(n) => n.accept_metadata(visitor),
            ASTNode::Conditional(n) => n.accept_metadata(visitor),
            ASTNode::ContractDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ElementaryTypeName(n) => n.accept_metadata(visitor),
            ASTNode::ElementaryTypeNameExpression(n) => n.accept_metadata(visitor),
            ASTNode::EmitStatement(n) => n.accept_metadata(visitor),
            ASTNode::EnumDefinition(n) => n.accept_metadata(visitor),
            ASTNode::EnumValue(n) => n.accept_metadata(visitor),
            ASTNode::EventDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ErrorDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ExpressionStatement(n) => n.accept_metadata(visitor),
            ASTNode::FunctionCall(n) => n.accept_metadata(visitor),
            ASTNode::FunctionCallOptions(n) => n.accept_metadata(visitor),
            ASTNode::FunctionDefinition(n) => n.accept_metadata(visitor),
            ASTNode::FunctionTypeName(n) => n.accept_metadata(visitor),
            ASTNode::ForStatement(n) => n.accept_metadata(visitor),
            ASTNode::Identifier(n) => n.accept_metadata(visitor),
            ASTNode::IdentifierPath(n) => n.accept_metadata(visitor),
            ASTNode::IfStatement(n) => n.accept_metadata(visitor),
            ASTNode::ImportDirective(n) => n.accept_metadata(visitor),
            ASTNode::IndexAccess(n) => n.accept_metadata(visitor),
            ASTNode::IndexRangeAccess(n) => n.accept_metadata(visitor),
            ASTNode::InheritanceSpecifier(n) => n.accept_metadata(visitor),
            ASTNode::InlineAssembly(n) => n.accept_metadata(visitor),
            ASTNode::Literal(n) => n.accept_metadata(visitor),
            ASTNode::MemberAccess(n) => n.accept_metadata(visitor),
            ASTNode::NewExpression(n) => n.accept_metadata(visitor),
            ASTNode::Mapping(n) => n.accept_metadata(visitor),
            ASTNode::ModifierDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ModifierInvocation(n) => n.accept_metadata(visitor),
            ASTNode::OverrideSpecifier(n) => n.accept_metadata(visitor),
            ASTNode::ParameterList(n) => n.accept_metadata(visitor),
            ASTNode::PragmaDirective(n) => n.accept_metadata(visitor),
            ASTNode::Return(n) => n.accept_metadata(visitor),
            ASTNode::RevertStatement(n) => n.accept_metadata(visitor),
            ASTNode::SourceUnit(n) => n.accept_metadata(visitor),
            ASTNode::StructDefinition(n) => n.accept_metadata(visitor),
            ASTNode::StructuredDocumentation(n) => n.accept_metadata(visitor),
            ASTNode::TryStatement(n) => n.accept_metadata(visitor),
            ASTNode::TryCatchClause(n) => n.accept_metadata(visitor),
            ASTNode::TupleExpression(n) => n.accept_metadata(visitor),
            ASTNode::UnaryOperation(n) => n.accept_metadata(visitor),
            ASTNode::UserDefinedTypeName(n) => n.accept_metadata(visitor),
            ASTNode::UserDefinedValueTypeDefinition(n) => n.accept_metadata(visitor),
            ASTNode::UsingForDirective(n) => n.accept_metadata(visitor),
            ASTNode::VariableDeclaration(n) => n.accept_metadata(visitor),
            ASTNode::VariableDeclarationStatement(n) => n.accept_metadata(visitor),
            ASTNode::WhileStatement(n) => n.accept_metadata(visitor),
            ASTNode::DoWhileStatement(n) => n.accept_metadata(visitor),
            ASTNode::BreakStatement(n) => n.accept_metadata(visitor),
            ASTNode::ContinueStatement(n) => n.accept_metadata(visitor),
            ASTNode::PlaceholderStatement(n) => n.accept_metadata(visitor),
        }
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.id())?;
        Ok(())
    }
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

impl From<DoWhileStatement> for ASTNode {
    fn from(value: DoWhileStatement) -> Self {
        ASTNode::DoWhileStatement(value)
    }
}

impl From<Break> for ASTNode {
    fn from(value: Break) -> Self {
        ASTNode::BreakStatement(value)
    }
}

impl From<Continue> for ASTNode {
    fn from(value: Continue) -> Self {
        ASTNode::ContinueStatement(value)
    }
}

impl From<PlaceholderStatement> for ASTNode {
    fn from(value: PlaceholderStatement) -> Self {
        ASTNode::PlaceholderStatement(value)
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

impl From<&DoWhileStatement> for ASTNode {
    fn from(value: &DoWhileStatement) -> Self {
        ASTNode::DoWhileStatement(value.clone())
    }
}

impl From<&Break> for ASTNode {
    fn from(value: &Break) -> Self {
        ASTNode::BreakStatement(value.clone())
    }
}

impl From<&Continue> for ASTNode {
    fn from(value: &Continue) -> Self {
        ASTNode::ContinueStatement(value.clone())
    }
}

impl From<&PlaceholderStatement> for ASTNode {
    fn from(value: &PlaceholderStatement) -> Self {
        ASTNode::PlaceholderStatement(value.clone())
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
            ASTNode::DoWhileStatement(node) => Some(&node.src),
            ASTNode::BreakStatement(node) => Some(&node.src),
            ASTNode::ContinueStatement(node) => Some(&node.src),
            ASTNode::PlaceholderStatement(node) => Some(&node.src),
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
    pub(crate) do_while_statements_context: HashMap<DoWhileStatement, NodeContext>,
    pub(crate) break_statements_context: HashMap<Break, NodeContext>,
    pub(crate) continue_statements_context: HashMap<Continue, NodeContext>,
    pub(crate) placeholder_statements_context: HashMap<PlaceholderStatement, NodeContext>,
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
                    let requried_content = &content[offset..offset + len];
                    return Some(requried_content.to_string());
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
            ASTNode::DoWhileStatement(node) => self
                .do_while_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::BreakStatement(node) => self
                .break_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ContinueStatement(node) => self
                .continue_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::PlaceholderStatement(node) => self
                .placeholder_statements_context
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
        let source_line = node
            .src()
            .map(|src| source_unit.source_line(src).unwrap_or(0)) // If `src` is `Some`, get the line number, else return 0
            .unwrap_or(0); // If `src` is `None`, default to 0

        // If the node is one of these, and it has a `name_location`, use that instead of the full `src`
        let src_location = match node {
            ASTNode::ContractDefinition(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            ASTNode::FunctionDefinition(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            ASTNode::ModifierDefinition(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            ASTNode::VariableDeclaration(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            _ => node.src().unwrap_or(""),
        };
        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        (absolute_path, source_line, chopped_location)
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

    fn visit_do_while_statement(&mut self, node: &DoWhileStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::DoWhileStatement(node.clone()));
        self.do_while_statements_context.insert(
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

    fn visit_break_statement(&mut self, node: &Break) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::BreakStatement(node.clone()));
        self.break_statements_context.insert(
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

    fn visit_continue_statement(&mut self, node: &Continue) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ContinueStatement(node.clone()));
        self.continue_statements_context.insert(
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

    fn visit_placeholder_statement(&mut self, node: &PlaceholderStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::PlaceholderStatement(node.clone()));
        self.placeholder_statements_context.insert(
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
