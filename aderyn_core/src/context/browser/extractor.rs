use crate::{
    ast::*,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

////////// PUBLICLY AVAILABLE EXTRACTION LIBRARY /////////////////////////

// ExtractArrayTypeNames is an extractor that extracts all ArrayTypeName nodes from a node.
#[derive(Default)]
pub struct ExtractArrayTypeNames {
    pub extracted: Vec<ArrayTypeName>,
}

impl ExtractArrayTypeNames {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractArrayTypeNames = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractArrayTypeNames {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractAssignments is an extractor that extracts all Assignments nodes from a node.
#[derive(Default)]
pub struct ExtractAssignments {
    pub extracted: Vec<Assignment>,
}

impl ExtractAssignments {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractAssignments = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractAssignments {
    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractBinaryOperations is an extractor that extracts all BinaryOperations nodes from a node.
#[derive(Default)]
pub struct ExtractBinaryOperations {
    pub extracted: Vec<BinaryOperation>,
}

impl ExtractBinaryOperations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractBinaryOperations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractBinaryOperations {
    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractBlocks extracts all Block nodes from a given node.
#[derive(Default)]
pub struct ExtractBlocks {
    pub extracted: Vec<Block>,
}

impl ExtractBlocks {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractBlocks {
    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractConditionals extracts all Conditional nodes from a given node.
#[derive(Default)]
pub struct ExtractConditionals {
    pub extracted: Vec<Conditional>,
}

impl ExtractConditionals {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractConditionals {
    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractContractDefinitions extracts all ContractDefinition nodes from a given node.
#[derive(Default)]
pub struct ExtractContractDefinitions {
    pub extracted: Vec<ContractDefinition>,
}

impl ExtractContractDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractContractDefinitions {
    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractElementaryTypeNames extracts all ElementaryTypeName nodes from a given node.
#[derive(Default)]
pub struct ExtractElementaryTypeNames {
    pub extracted: Vec<ElementaryTypeName>,
}

impl ExtractElementaryTypeNames {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractElementaryTypeNames {
    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractElementaryTypeNameExpressions extracts all ElementaryTypeNameExpression nodes from a given node.
#[derive(Default)]
pub struct ExtractElementaryTypeNameExpressions {
    pub extracted: Vec<ElementaryTypeNameExpression>,
}

impl ExtractElementaryTypeNameExpressions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractElementaryTypeNameExpressions {
    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractEmitStatements extracts all EmitStatement nodes from a given node.
#[derive(Default)]
pub struct ExtractEmitStatements {
    pub extracted: Vec<EmitStatement>,
}

impl ExtractEmitStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractEmitStatements {
    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractEnumDefinitions extracts all EnumDefinition nodes from a given node.
#[derive(Default)]
pub struct ExtractEnumDefinitions {
    pub extracted: Vec<EnumDefinition>,
}

impl ExtractEnumDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractEnumDefinitions {
    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractEnumValues extracts all EnumValue nodes from a given node.
#[derive(Default)]
pub struct ExtractEnumValues {
    pub extracted: Vec<EnumValue>,
}

impl ExtractEnumValues {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractEnumValues {
    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractEventDefinitions extracts all EventDefinition nodes from a given node.
#[derive(Default)]
pub struct ExtractEventDefinitions {
    pub extracted: Vec<EventDefinition>,
}

impl ExtractEventDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractEventDefinitions {
    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractErrorDefinitions extracts all ErrorDefinition nodes from a given node.
#[derive(Default)]
pub struct ExtractErrorDefinitions {
    pub extracted: Vec<ErrorDefinition>,
}

impl ExtractErrorDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractErrorDefinitions {
    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractExpressionStatements extracts all ExpressionStatement nodes from a given node.
#[derive(Default)]
pub struct ExtractExpressionStatements {
    pub extracted: Vec<ExpressionStatement>,
}

impl ExtractExpressionStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractExpressionStatements {
    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractFunctionCalls extracts all FunctionCall nodes from a given node.
#[derive(Default)]
pub struct ExtractFunctionCalls {
    pub extracted: Vec<FunctionCall>,
}

impl ExtractFunctionCalls {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractFunctionCalls {
    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractFunctionCallsOps extracts all FunctionCallOps nodes from a given node.
#[derive(Default)]
pub struct ExtractFunctionCallOptions {
    pub extracted: Vec<FunctionCallOptions>,
}

impl ExtractFunctionCallOptions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractFunctionCallOptions {
    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractFunctionDefinitions extracts all FunctionDefinition nodes from a given node.
#[derive(Default)]

pub struct ExtractFunctionDefinitions {
    pub extracted: Vec<FunctionDefinition>,
}

impl ExtractFunctionDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractFunctionDefinitions {
    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractFunctionTypeNames extracts all FunctionTypeName nodes from a given node.
#[derive(Default)]

pub struct ExtractFunctionTypeNames {
    pub extracted: Vec<FunctionTypeName>,
}

impl ExtractFunctionTypeNames {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractFunctionTypeNames {
    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractForStatements extracts all ForStatement nodes from a given node.
#[derive(Default)]

pub struct ExtractForStatements {
    pub extracted: Vec<ForStatement>,
}

impl ExtractForStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractForStatements {
    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractIdentifiers extracts all Identifier nodes from a given node.
#[derive(Default)]

pub struct ExtractIdentifiers {
    pub extracted: Vec<Identifier>,
}

impl ExtractIdentifiers {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractIdentifiers {
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractIdentifierPaths extracts all IdentifierPath nodes from a given node.
#[derive(Default)]

pub struct ExtractIdentifierPaths {
    pub extracted: Vec<IdentifierPath>,
}

impl ExtractIdentifierPaths {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractIdentifierPaths {
    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractIfStatements extracts all IfStatement nodes from a given node.
#[derive(Default)]
pub struct ExtractIfStatements {
    pub extracted: Vec<IfStatement>,
}

impl ExtractIfStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractIfStatements = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractIfStatements {
    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractImportDirectives extracts all ImportDirective nodes from a given node.
#[derive(Default)]
pub struct ExtractImportDirectives {
    pub extracted: Vec<ImportDirective>,
}

impl ExtractImportDirectives {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractImportDirectives = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractImportDirectives {
    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractIndexAccesses extracts all IndexAccess nodes from a given node.
#[derive(Default)]
pub struct ExtractIndexAccesses {
    pub extracted: Vec<IndexAccess>,
}

impl ExtractIndexAccesses {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractIndexAccesses = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractIndexAccesses {
    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractIndexRangeAccesses extracts all IndexRangeAccess nodes from a given node.
#[derive(Default)]
pub struct ExtractIndexRangeAccesses {
    pub extracted: Vec<IndexRangeAccess>,
}

impl ExtractIndexRangeAccesses {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractIndexRangeAccesses = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractIndexRangeAccesses {
    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractInheritanceSpecifiers extracts all InheritanceSpecifier nodes from a given node.
#[derive(Default)]
pub struct ExtractInheritanceSpecifiers {
    pub extracted: Vec<InheritanceSpecifier>,
}

impl ExtractInheritanceSpecifiers {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractInheritanceSpecifiers = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractInheritanceSpecifiers {
    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractInlineAssemblys extracts all InlineAssembly nodes from a given node.
#[derive(Default)]
pub struct ExtractInlineAssemblys {
    pub extracted: Vec<InlineAssembly>,
}

impl ExtractInlineAssemblys {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractInlineAssemblys = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractInlineAssemblys {
    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractLiterals extracts all Literal nodes from a given node.
#[derive(Default)]
pub struct ExtractLiterals {
    pub extracted: Vec<Literal>,
}

impl ExtractLiterals {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractLiterals = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractLiterals {
    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractMemberAccesses is an extractor that extracts all MemberAccess nodes from a node.
#[derive(Default)]
pub struct ExtractMemberAccesses {
    pub extracted: Vec<MemberAccess>,
}

impl ExtractMemberAccesses {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractMemberAccesses = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractMemberAccesses {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractNewExpressions extracts all NewExpression nodes from a given node.
#[derive(Default)]
pub struct ExtractNewExpressions {
    pub extracted: Vec<NewExpression>,
}

impl ExtractNewExpressions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractNewExpressions = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractNewExpressions {
    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractMappings extracts all Mapping nodes from a given node.
#[derive(Default)]
pub struct ExtractMappings {
    pub extracted: Vec<Mapping>,
}

impl ExtractMappings {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractMappings = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractMappings {
    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractModifierDefinitions extracts all ModifierDefinition nodes from a given node.
#[derive(Default)]
pub struct ExtractModifierDefinitions {
    pub extracted: Vec<ModifierDefinition>,
}

impl ExtractModifierDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractModifierDefinitions = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractModifierDefinitions {
    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractModifierInvocations extracts all ModifierInvocation nodes from a given node.
#[derive(Default)]
pub struct ExtractModifierInvocations {
    pub extracted: Vec<ModifierInvocation>,
}

impl ExtractModifierInvocations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractModifierInvocations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractModifierInvocations {
    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractOverrideSpecifiers extracts all OverrideSpecifier nodes from a given node.
#[derive(Default)]
pub struct ExtractOverrideSpecifiers {
    pub extracted: Vec<OverrideSpecifier>,
}

impl ExtractOverrideSpecifiers {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractOverrideSpecifiers = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractOverrideSpecifiers {
    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractParameterLists extracts all ParameterList nodes from a given node.
#[derive(Default)]
pub struct ExtractParameterLists {
    pub extracted: Vec<ParameterList>,
}

impl ExtractParameterLists {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractParameterLists = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractParameterLists {
    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractPragmaDirectives extracts all PragmaDirective nodes from a given node.
#[derive(Default)]
pub struct ExtractPragmaDirectives {
    pub extracted: Vec<PragmaDirective>,
}

impl ExtractPragmaDirectives {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractPragmaDirectives = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractPragmaDirectives {
    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractReturns extracts all Return nodes from a given node.
#[derive(Default)]
pub struct ExtractReturns {
    pub extracted: Vec<Return>,
}

impl ExtractReturns {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractReturns = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractReturns {
    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractRevertStatements extracts all RevertStatement nodes from a given node.
#[derive(Default)]
pub struct ExtractRevertStatements {
    pub extracted: Vec<RevertStatement>,
}

impl ExtractRevertStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractRevertStatements = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractRevertStatements {
    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractSourceUnits is not needed

// ExtractStructDefinitions extracts all StructDefinition nodes from a given node.
#[derive(Default)]
pub struct ExtractStructDefinitions {
    pub extracted: Vec<StructDefinition>,
}

impl ExtractStructDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractStructDefinitions = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractStructDefinitions {
    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractStructuredDocumentations extracts all StructuredDocumentation nodes from a given node.
#[derive(Default)]
pub struct ExtractStructuredDocumentations {
    pub extracted: Vec<StructuredDocumentation>,
}

impl ExtractStructuredDocumentations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractStructuredDocumentations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractStructuredDocumentations {
    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractTryStatements extracts all TryStatement nodes from a given node.
#[derive(Default)]
pub struct ExtractTryStatements {
    pub extracted: Vec<TryStatement>,
}

impl ExtractTryStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractTryStatements = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractTryStatements {
    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractTryCatchClauses extracts all TryCatchClause nodes from a given node.
#[derive(Default)]
pub struct ExtractTryCatchClauses {
    pub extracted: Vec<TryCatchClause>,
}

impl ExtractTryCatchClauses {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractTryCatchClauses = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractTryCatchClauses {
    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractTupleExpressions extracts all TupleExpression nodes from a given node.
#[derive(Default)]
pub struct ExtractTupleExpressions {
    pub extracted: Vec<TupleExpression>,
}

impl ExtractTupleExpressions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractTupleExpressions = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractTupleExpressions {
    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractUnaryOperations extracts all UnaryOperations nodes from a given node.
#[derive(Default)]
pub struct ExtractUnaryOperations {
    pub extracted: Vec<UnaryOperation>,
}

impl ExtractUnaryOperations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractUnaryOperations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractUnaryOperations {
    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractUserDefinedTypeNames extracts all UserDefinedTypeName nodes from a given node.
#[derive(Default)]
pub struct ExtractUserDefinedTypeNames {
    pub extracted: Vec<UserDefinedTypeName>,
}

impl ExtractUserDefinedTypeNames {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractUserDefinedTypeNames = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractUserDefinedTypeNames {
    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractUserDefinedValueTypeDefinitions extracts all UserDefinedValueTypes nodes from a given node.
#[derive(Default)]
pub struct ExtractUserDefinedValueTypeDefinitions {
    pub extracted: Vec<UserDefinedValueTypeDefinition>,
}

impl ExtractUserDefinedValueTypeDefinitions {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractUserDefinedValueTypeDefinitions = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractUserDefinedValueTypeDefinitions {
    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractUsingForDirectives extracts all UsingForDirective nodes from a given node.
#[derive(Default)]
pub struct ExtractUsingForDirectives {
    pub extracted: Vec<UsingForDirective>,
}

impl ExtractUsingForDirectives {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractUsingForDirectives = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractUsingForDirectives {
    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractVariableDeclarations extracts all VariableDeclaration nodes from a given node.
#[derive(Default)]
pub struct ExtractVariableDeclarations {
    pub extracted: Vec<VariableDeclaration>,
}

impl ExtractVariableDeclarations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractVariableDeclarations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractVariableDeclarations {
    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractVariableDeclarationStatements extracts all VariableDeclarationStatement nodes from a given node.
#[derive(Default)]
pub struct ExtractVariableDeclarationStatements {
    pub extracted: Vec<VariableDeclarationStatement>,
}

impl ExtractVariableDeclarationStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractVariableDeclarationStatements = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractVariableDeclarationStatements {
    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

// ExtractWhileStatements extracts all WhileStatement nodes from a given node.
#[derive(Default)]
pub struct ExtractWhileStatements {
    pub extracted: Vec<WhileStatement>,
}

impl ExtractWhileStatements {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractWhileStatements = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractWhileStatements {
    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        self.extracted.push(node.clone());
        Ok(true)
    }
}

/////////// EXTRACTION UTILS FOR CRATE - LEVEL ACCESS //////////////////

// ExtractImmediateChildren is an extractor that extracts immediate children from a node
#[derive(Default)]
pub(crate) struct ExtractImmediateChildrenIDs {
    pub extracted: Vec<NodeID>,
}

impl ExtractImmediateChildrenIDs {
    pub(crate) fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractImmediateChildrenIDs = Self::default();
        node.accept_metadata(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractImmediateChildrenIDs {
    fn visit_immediate_children(
        &mut self,
        _node_id: NodeID,
        node_children_ids: Vec<NodeID>,
    ) -> Result<()> {
        self.extracted.extend(node_children_ids);
        Ok(())
    }
}
