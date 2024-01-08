use crate::{
    ast::*,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

// ArrayTypeNameExtractor is a browser that extracts all ArrayTypeName nodes from a node.
#[derive(Default)]
pub struct ArrayTypeNameExtractor {
    pub extracted: Vec<ArrayTypeName>,
}

impl ArrayTypeNameExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut browser: ArrayTypeNameExtractor = Self::default();
        node.accept(&mut browser).unwrap_or_default();
        browser
    }
}

impl ASTConstVisitor for ArrayTypeNameExtractor {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// AssignmentExtractor is a browser that extracts all Assignments nodes from a node.
#[derive(Default)]
pub struct AssignmentExtractor {
    pub extracted: Vec<Assignment>,
}

impl AssignmentExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut browser: AssignmentExtractor = Self::default();
        node.accept(&mut browser).unwrap_or_default();
        browser
    }
}

impl ASTConstVisitor for AssignmentExtractor {
    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// BinaryOperationExtractor is a browser that extracts all BinaryOperations nodes from a node.
#[derive(Default)]
pub struct BinaryOperationExtractor {
    pub extracted: Vec<BinaryOperation>,
}

impl BinaryOperationExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut browser: BinaryOperationExtractor = Self::default();
        node.accept(&mut browser).unwrap_or_default();
        browser
    }
}

impl ASTConstVisitor for BinaryOperationExtractor {
    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// BlockExtractor extracts all Block nodes from a given node.
#[derive(Default)]
pub struct BlockExtractor {
    pub extracted: Vec<Block>,
}

impl BlockExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for BlockExtractor {
    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ConditionalExtractor extracts all Conditional nodes from a given node.
#[derive(Default)]
pub struct ConditionalExtractor {
    pub extracted: Vec<Conditional>,
}

impl ConditionalExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ConditionalExtractor {
    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ContractDefinitionExtractor extracts all ContractDefinition nodes from a given node.
#[derive(Default)]
pub struct ContractDefinitionExtractor {
    pub extracted: Vec<ContractDefinition>,
}

impl ContractDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ContractDefinitionExtractor {
    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ElementaryTypeNameExtractor extracts all ElementaryTypeName nodes from a given node.
#[derive(Default)]
pub struct ElementaryTypeNameExtractor {
    pub extracted: Vec<ElementaryTypeName>,
}

impl ElementaryTypeNameExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ElementaryTypeNameExtractor {
    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ElementaryTypeNameExpressionExtractor extracts all ElementaryTypeNameExpression nodes from a given node.
#[derive(Default)]
pub struct ElementaryTypeNameExpressionExtractor {
    pub extracted: Vec<ElementaryTypeNameExpression>,
}

impl ElementaryTypeNameExpressionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ElementaryTypeNameExpressionExtractor {
    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// EmitStatementExtractor extracts all EmitStatement nodes from a given node.
#[derive(Default)]
pub struct EmitStatementExtractor {
    pub extracted: Vec<EmitStatement>,
}

impl EmitStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for EmitStatementExtractor {
    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// EnumDefinitionExtractor extracts all EnumDefinition nodes from a given node.
#[derive(Default)]
pub struct EnumDefinitionExtractor {
    pub extracted: Vec<EnumDefinition>,
}

impl EnumDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for EnumDefinitionExtractor {
    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// EnumValueExtractor extracts all EnumValue nodes from a given node.
#[derive(Default)]
pub struct EnumValueExtractor {
    pub extracted: Vec<EnumValue>,
}

impl EnumValueExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for EnumValueExtractor {
    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// EventDefinitionExtractor extracts all EventDefinition nodes from a given node.
#[derive(Default)]
pub struct EventDefinitionExtractor {
    pub extracted: Vec<EventDefinition>,
}

impl EventDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for EventDefinitionExtractor {
    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ErrorDefinitionExtractor extracts all ErrorDefinition nodes from a given node.
#[derive(Default)]
pub struct ErrorDefinitionExtractor {
    pub extracted: Vec<ErrorDefinition>,
}

impl ErrorDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ErrorDefinitionExtractor {
    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExpressionStatementExtractor extracts all ExpressionStatement nodes from a given node.
#[derive(Default)]
pub struct ExpressionStatementExtractor {
    pub extracted: Vec<ExpressionStatement>,
}

impl ExpressionStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExpressionStatementExtractor {
    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// FunctionCallExtractor extracts all FunctionCall nodes from a given node.
#[derive(Default)]
pub struct FunctionCallExtractor {
    pub extracted: Vec<FunctionCall>,
}

impl FunctionCallExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for FunctionCallExtractor {
    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// FunctionDefinitionExtractor extracts all FunctionDefinition nodes from a given node.
#[derive(Default)]

pub struct FunctionDefinitionExtractor {
    pub extracted: Vec<FunctionDefinition>,
}

impl FunctionDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for FunctionDefinitionExtractor {
    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// FunctionTypeNameExtractor extracts all FunctionTypeName nodes from a given node.
#[derive(Default)]

pub struct FunctionTypeNameExtractor {
    pub extracted: Vec<FunctionTypeName>,
}

impl FunctionTypeNameExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for FunctionTypeNameExtractor {
    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ForStatementExtractor extracts all ForStatement nodes from a given node.
#[derive(Default)]

pub struct ForStatementExtractor {
    pub extracted: Vec<ForStatement>,
}

impl ForStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ForStatementExtractor {
    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// IdentifierExtractor extracts all Identifier nodes from a given node.
#[derive(Default)]

pub struct IdentifierExtractor {
    pub extracted: Vec<Identifier>,
}

impl IdentifierExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for IdentifierExtractor {
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// IdentifierPathExtractor extracts all IdentifierPath nodes from a given node.
#[derive(Default)]

pub struct IdentifierPathExtractor {
    pub extracted: Vec<IdentifierPath>,
}

impl IdentifierPathExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for IdentifierPathExtractor {
    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// IfStatementExtractor extracts all IfStatement nodes from a given node.
#[derive(Default)]
pub struct IfStatementExtractor {
    pub extracted: Vec<IfStatement>,
}

impl IfStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: IfStatementExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for IfStatementExtractor {
    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ImportDirectiveExtractor extracts all ImportDirective nodes from a given node.
#[derive(Default)]
pub struct ImportDirectiveExtractor {
    pub extracted: Vec<ImportDirective>,
}

impl ImportDirectiveExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ImportDirectiveExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ImportDirectiveExtractor {
    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// IndexAccessExtractor extracts all IndexAccess nodes from a given node.
#[derive(Default)]
pub struct IndexAccessExtractor {
    pub extracted: Vec<IndexAccess>,
}

impl IndexAccessExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: IndexAccessExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for IndexAccessExtractor {
    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// IndexRangeAccessExtractor extracts all IndexRangeAccess nodes from a given node.
#[derive(Default)]
pub struct IndexRangeAccessExtractor {
    pub extracted: Vec<IndexRangeAccess>,
}

impl IndexRangeAccessExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: IndexRangeAccessExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for IndexRangeAccessExtractor {
    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// InheritanceSpecifierExtractor extracts all InheritanceSpecifier nodes from a given node.
#[derive(Default)]
pub struct InheritanceSpecifierExtractor {
    pub extracted: Vec<InheritanceSpecifier>,
}

impl InheritanceSpecifierExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: InheritanceSpecifierExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for InheritanceSpecifierExtractor {
    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// InlineAssemblyExtractor extracts all InlineAssembly nodes from a given node.
#[derive(Default)]
pub struct InlineAssemblyExtractor {
    pub extracted: Vec<InlineAssembly>,
}

impl InlineAssemblyExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: InlineAssemblyExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for InlineAssemblyExtractor {
    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// LiteralExtractor extracts all Literal nodes from a given node.
#[derive(Default)]
pub struct LiteralExtractor {
    pub extracted: Vec<Literal>,
}

impl LiteralExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: LiteralExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for LiteralExtractor {
    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// MemberAccessExtractor is a browser that extracts all MemberAccess nodes from a node.
#[derive(Default)]
pub struct MemberAccessExtractor {
    pub extracted: Vec<MemberAccess>,
}

impl MemberAccessExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut browser: MemberAccessExtractor = Self::default();
        node.accept(&mut browser).unwrap_or_default();
        browser
    }
}

impl ASTConstVisitor for MemberAccessExtractor {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// NewExpressionExtractor extracts all NewExpression nodes from a given node.
#[derive(Default)]
pub struct NewExpressionExtractor {
    pub extracted: Vec<NewExpression>,
}

impl NewExpressionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: NewExpressionExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for NewExpressionExtractor {
    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// MappingExtractor extracts all Mapping nodes from a given node.
#[derive(Default)]
pub struct MappingExtractor {
    pub extracted: Vec<Mapping>,
}

impl MappingExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: MappingExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for MappingExtractor {
    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ModifierDefinitionExtractor extracts all ModifierDefinition nodes from a given node.
#[derive(Default)]
pub struct ModifierDefinitionExtractor {
    pub extracted: Vec<ModifierDefinition>,
}

impl ModifierDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ModifierDefinitionExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ModifierDefinitionExtractor {
    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ModifierInvocationExtractor extracts all ModifierInvocation nodes from a given node.
#[derive(Default)]
pub struct ModifierInvocationExtractor {
    pub extracted: Vec<ModifierInvocation>,
}

impl ModifierInvocationExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ModifierInvocationExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ModifierInvocationExtractor {
    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// OverrideSpecifierExtractor extracts all OverrideSpecifier nodes from a given node.
#[derive(Default)]
pub struct OverrideSpecifierExtractor {
    pub extracted: Vec<OverrideSpecifier>,
}

impl OverrideSpecifierExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: OverrideSpecifierExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for OverrideSpecifierExtractor {
    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ParameterListExtractor extracts all ParameterList nodes from a given node.
#[derive(Default)]
pub struct ParameterListExtractor {
    pub extracted: Vec<ParameterList>,
}

impl ParameterListExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ParameterListExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ParameterListExtractor {
    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// PragmaDirectiveExtractor extracts all PragmaDirective nodes from a given node.
#[derive(Default)]
pub struct PragmaDirectiveExtractor {
    pub extracted: Vec<PragmaDirective>,
}

impl PragmaDirectiveExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: PragmaDirectiveExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for PragmaDirectiveExtractor {
    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ReturnExtractor extracts all Return nodes from a given node.
#[derive(Default)]
pub struct ReturnExtractor {
    pub extracted: Vec<Return>,
}

impl ReturnExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ReturnExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ReturnExtractor {
    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// RevertStatementExtractor extracts all RevertStatement nodes from a given node.
#[derive(Default)]
pub struct RevertStatementExtractor {
    pub extracted: Vec<RevertStatement>,
}

impl RevertStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: RevertStatementExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for RevertStatementExtractor {
    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// SourceUnitExtractor is not needed

// StructDefinitionExtractor extracts all StructDefinition nodes from a given node.
#[derive(Default)]
pub struct StructDefinitionExtractor {
    pub extracted: Vec<StructDefinition>,
}

impl StructDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: StructDefinitionExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for StructDefinitionExtractor {
    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// StructuredDocumentationExtractor extracts all StructuredDocumentation nodes from a given node.
#[derive(Default)]
pub struct StructuredDocumentationExtractor {
    pub extracted: Vec<StructuredDocumentation>,
}

impl StructuredDocumentationExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: StructuredDocumentationExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for StructuredDocumentationExtractor {
    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// TryStatementExtractor extracts all TryStatement nodes from a given node.
#[derive(Default)]
pub struct TryStatementExtractor {
    pub extracted: Vec<TryStatement>,
}

impl TryStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: TryStatementExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for TryStatementExtractor {
    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// TryCatchClauseExtractor extracts all TryCatchClause nodes from a given node.
#[derive(Default)]
pub struct TryCatchClauseExtractor {
    pub extracted: Vec<TryCatchClause>,
}

impl TryCatchClauseExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: TryCatchClauseExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for TryCatchClauseExtractor {
    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// TupleExpressionExtractor extracts all TupleExpression nodes from a given node.
#[derive(Default)]
pub struct TupleExpressionExtractor {
    pub extracted: Vec<TupleExpression>,
}

impl TupleExpressionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: TupleExpressionExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for TupleExpressionExtractor {
    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// UnaryOperationExtractor extracts all UnaryOperations nodes from a given node.
#[derive(Default)]
pub struct UnaryOperationExtractor {
    pub extracted: Vec<UnaryOperation>,
}

impl UnaryOperationExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: UnaryOperationExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for UnaryOperationExtractor {
    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// UserDefinedTypeNameExtractor extracts all UserDefinedTypeName nodes from a given node.
#[derive(Default)]
pub struct UserDefinedTypeNameExtractor {
    pub extracted: Vec<UserDefinedTypeName>,
}

impl UserDefinedTypeNameExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: UserDefinedTypeNameExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for UserDefinedTypeNameExtractor {
    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// UserDefinedValueTypeDefinitionExtractor extracts all UserDefinedValueTypes nodes from a given node.
#[derive(Default)]
pub struct UserDefinedValueTypeDefinitionExtractor {
    pub extracted: Vec<UserDefinedValueTypeDefinition>,
}

impl UserDefinedValueTypeDefinitionExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: UserDefinedValueTypeDefinitionExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for UserDefinedValueTypeDefinitionExtractor {
    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// UsingForDirectiveExtractor extracts all UsingForDirective nodes from a given node.
#[derive(Default)]
pub struct UsingForDirectiveExtractor {
    pub extracted: Vec<UsingForDirective>,
}

impl UsingForDirectiveExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: UsingForDirectiveExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for UsingForDirectiveExtractor {
    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// VariableDeclarationExtractor extracts all VariableDeclaration nodes from a given node.
#[derive(Default)]
pub struct VariableDeclarationExtractor {
    pub extracted: Vec<VariableDeclaration>,
}

impl VariableDeclarationExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: VariableDeclarationExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for VariableDeclarationExtractor {
    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// VariableDeclarationStatementExtractor extracts all VariableDeclarationStatement nodes from a given node.
#[derive(Default)]
pub struct VariableDeclarationStatementExtractor {
    pub extracted: Vec<VariableDeclarationStatement>,
}

impl VariableDeclarationStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: VariableDeclarationStatementExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for VariableDeclarationStatementExtractor {
    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// WhileStatementExtractor extracts all WhileStatement nodes from a given node.
#[derive(Default)]
pub struct WhileStatementExtractor {
    pub extracted: Vec<WhileStatement>,
}

impl WhileStatementExtractor {
    pub fn extract_from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: WhileStatementExtractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for WhileStatementExtractor {
    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}
