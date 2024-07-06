use std::collections::{BTreeMap, HashMap};

use super::macros::ast_node;
use super::*;

use serde::{Deserialize, Serialize};

ast_node!(
    struct Block {
        statements: Vec<Statement>,
    }
);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum ContractDefinitionNode {
    UsingForDirective(UsingForDirective),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub base_name: UserDefinedTypeNameOrIdentifierPath,
    pub arguments: Option<Vec<Expression>>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub documentation: Option<Documentation>,
    #[serde(rename = "contractKind")]
    pub kind: ContractKind,
    #[serde(rename = "abstract")]
    pub is_abstract: Option<bool>,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub canonical_name: Option<String>,
    pub contract_dependencies: Vec<NodeID>,
    pub used_errors: Option<Vec<NodeID>>,
    pub used_events: Option<Vec<usize>>,
    #[serde(default, rename = "internalFunctionIDs")]
    pub internal_function_ids: BTreeMap<String, usize>,
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: NodeID,
    pub fully_implemented: Option<bool>,
    pub linearized_base_contracts: Option<Vec<NodeID>>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum Documentation {
    String(Option<String>),
    Structured(Option<StructuredDocumentation>),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub text: String,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub name_location: Option<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub members: Vec<EnumValue>,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub documentation: Option<Documentation>,
    pub error_selector: Option<String>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub anonymous: bool,
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub event_selector: Option<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    MemberAccess(MemberAccess),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    TupleExpression(TupleExpression),
    NewExpression(NewExpression),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub prefix: bool,
    pub sub_expression: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub common_type: TypeDescriptions,
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub true_expression: Box<Expression>,
    pub false_expression: Box<Expression>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub left_hand_side: Box<Expression>,
    pub right_hand_side: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub kind: FunctionCallKind,
    pub try_call: Option<bool>,
    pub names: Vec<String>,
    pub arguments: Vec<Expression>,
    pub expression: Box<Expression>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FunctionCallKind {
    FunctionCall,
    TypeConversion,
    StructConstructorCall,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {
    pub names: Vec<String>,
    pub options: Vec<Expression>,
    pub arguments: Option<Vec<Expression>>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub type_descriptions: TypeDescriptions,
    pub type_name: TypeName,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub base_expression: Box<Expression>,
    pub index_expression: Option<Box<Expression>>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub base_expression: Box<Expression>,
    pub start_expression: Option<Box<Expression>>,
    pub end_expression: Option<Box<Expression>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {
    pub member_name: String,
    pub expression: Box<Expression>,
    pub referenced_declaration: Option<NodeID>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub type_name: TypeName,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpression {
    pub components: Vec<Option<Expression>>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_inline_array: bool,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Constructor,
    Function,
    Receive,
    Fallback,
    FreeFunction,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ParameterList {
    pub parameters: Vec<VariableDeclaration>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct OverrideSpecifier {
    pub overrides: Vec<UserDefinedTypeNameOrIdentifierPath>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDefinition {
    pub base_functions: Option<Vec<NodeID>>,
    pub body: Option<Block>,
    pub documentation: Option<Documentation>,
    pub function_selector: Option<String>,
    pub implemented: bool,
    /// The kind of function this node defines. Only valid for Solidity versions 0.5.x and
    /// above.
    ///
    /// For cross-version compatibility use [`FunctionDefinition::kind()`].
    pub kind: Option<FunctionKind>,
    #[serde(default)]
    /// For cross-version compatibility use [`FunctionDefinition::state_mutability()`].
    pub state_mutability: Option<StateMutability>,
    #[serde(default, rename = "virtual")]
    pub is_virtual: bool,
    /// Whether or not this function is the constructor. Only valid for Solidity versions below
    /// 0.5.x.
    ///
    /// After 0.5.x you must use `kind`. For cross-version compatibility use
    /// [`FunctionDefinition::kind()`].
    #[serde(default)]
    pub is_constructor: bool,
    /// Whether or not this function is constant (view or pure). Only valid for Solidity
    /// versions below 0.5.x.
    ///
    /// After 0.5.x you must use `state_mutability`. For cross-version compatibility use
    /// [`FunctionDefinition::state_mutability()`].
    #[serde(default)]
    pub is_declared_const: bool,
    /// Whether or not this function is payable. Only valid for Solidity versions below
    /// 0.5.x.
    ///
    /// After 0.5.x you must use `state_mutability`. For cross-version compatibility use
    /// [`FunctionDefinition::state_mutability()`].
    #[serde(default)]
    pub is_payable: bool,
    pub modifiers: Vec<ModifierInvocation>,
    pub name: String,
    pub name_location: Option<String>,
    pub overrides: Option<OverrideSpecifier>,
    pub parameters: ParameterList,
    pub return_parameters: ParameterList,
    pub scope: NodeID,
    pub super_function: Option<NodeID>,
    pub r#virtual: Option<bool>,
    pub visibility: Visibility,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub name: String,
    pub overloaded_declarations: Vec<NodeID>,
    pub referenced_declaration: Option<NodeID>,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub name: String,
    pub referenced_declaration: Option<NodeID>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Identifier,
    pub local: Option<String>,
    pub name_location: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ImportDirective {
    pub file: String,
    pub source_unit: NodeID,
    pub scope: NodeID,
    pub absolute_path: Option<String>,
    pub unit_alias: String,
    pub name_location: Option<String>,
    pub symbol_aliases: Vec<SymbolAlias>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum LiteralKind {
    Bool,
    Number,
    String,
    HexString,
    Address,
    UnicodeString,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub hex_value: Option<String>,
    pub value: Option<String>,
    pub subdenomination: Option<String>,
    pub kind: LiteralKind,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub body: Block,
    pub base_modifiers: Option<Vec<usize>>,
    pub overrides: Option<OverrideSpecifier>,
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub r#virtual: Option<bool>,
    pub visibility: Visibility,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    ModifierInvocation,
    BaseConstructorSpecifier,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub arguments: Option<Vec<Expression>>,
    pub modifier_name: IdentifierOrIdentifierPath,
    pub src: String,
    pub id: NodeID,
    pub kind: Option<ModifierInvocationKind>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum IdentifierOrIdentifierPath {
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
}

#[derive(Default, Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PragmaDirective {
    pub literals: Vec<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Eq, Deserialize, Serialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum SourceUnitNode {
    FunctionDefinition(FunctionDefinition),
    StructDefinition(StructDefinition),
    ErrorDefinition(ErrorDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    ImportDirective(ImportDirective),
    PragmaDirective(PragmaDirective),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    ContractDefinition(ContractDefinition),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub license: Option<String>,
    pub nodes: Vec<SourceUnitNode>,
    pub exported_symbols: Option<HashMap<String, Vec<NodeID>>>,
    pub absolute_path: Option<String>,
    pub id: NodeID,

    #[serde(skip_serializing)]
    pub source: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Block),
    Break(Break),
    Continue(Continue),
    DoWhileStatement(DoWhileStatement),
    PlaceholderStatement(PlaceholderStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    UncheckedBlock(Block),
    Return(Return),
    RevertStatement(RevertStatement),
    ExpressionStatement(ExpressionStatement),
    InlineAssembly(InlineAssembly),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub assignments: Vec<Option<NodeID>>,
    pub declarations: Vec<Option<VariableDeclaration>>,
    pub initial_value: Option<Expression>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum BlockOrStatement {
    Block(Box<Block>),
    Statement(Box<Statement>),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub condition: Expression,
    pub true_body: BlockOrStatement,
    pub false_body: Option<BlockOrStatement>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub initialization_expression: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub loop_expression: Option<Box<ExpressionStatement>>,
    pub body: BlockOrStatement,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
    pub body: Block,
    pub condition: Expression,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub event_call: Expression,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub clauses: Vec<TryCatchClause>,
    pub external_call: FunctionCall,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub error_call: FunctionCall,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub block: Block,
    pub error_name: Option<String>,
    pub parameters: Option<ParameterList>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub function_return_parameters: NodeID,
    pub expression: Option<Expression>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    #[serde(rename = "AST")]
    pub ast: Option<YulBlock>,
    pub evm_version: Option<String>,
    pub external_references: Vec<ExternalReference>,
    pub operations: Option<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Break {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Continue {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderStatement {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockOrStatement,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub visibility: Visibility,
    pub members: Vec<VariableDeclaration>,
    pub scope: NodeID,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum TypeName {
    FunctionTypeName(FunctionTypeName),
    ArrayTypeName(ArrayTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
    ElementaryTypeName(ElementaryTypeName),
    /// A string representing the type name.
    ///
    /// This variant applies to older compiler versions.
    Raw(String),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub state_mutability: Option<StateMutability>,
    pub name: String,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedTypeName {
    pub path_node: Option<IdentifierPath>,
    pub referenced_declaration: NodeID,
    pub name: Option<String>,
    pub type_descriptions: TypeDescriptions,
    pub contract_scope: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTypeName {
    pub visibility: Visibility,
    pub state_mutability: StateMutability,
    pub parameter_types: ParameterList,
    pub return_parameter_types: ParameterList,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeName {
    pub base_type: Box<TypeName>,
    pub length: Box<Option<Expression>>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub key_type: Box<TypeName>,
    pub value_type: Box<TypeName>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub underlying_type: TypeName,
    pub name: String,
    pub name_location: Option<String>,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub function_list: Option<Vec<UsingForFunctionItem>>,
    #[serde(default)]
    pub global: bool,
    pub library_name: Option<UserDefinedTypeNameOrIdentifierPath>,
    pub type_name: Option<TypeName>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(tag = "nodeType")]
pub enum UserDefinedTypeNameOrIdentifierPath {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum UsingForFunctionItem {
    Function(FunctionIdentifierPath),
    OverloadedOperator(OverloadedOperator),
}

/// A wrapper around [IdentifierPath] for the [UsingForDirective].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct FunctionIdentifierPath {
    pub function: IdentifierPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct OverloadedOperator {
    pub definition: IdentifierPath,
    pub operator: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Immutable,
    Mutable,
    Constant,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    NonPayable,
    Payable,
    View,
    Pure,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Internal,
    External,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Default,
    Memory,
    Calldata,
    Storage,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclaration {
    pub base_functions: Option<Vec<NodeID>>,
    /// Marks whether or not the variable is a constant before Solidity 0.7.x.
    ///
    /// After 0.7.x you must use `mutability`. For cross-version compatibility use
    /// [`VariableDeclaration::mutability()`].
    #[serde(default)]
    pub constant: bool,
    pub documentation: Option<Documentation>,
    pub function_selector: Option<String>,
    pub indexed: Option<bool>,
    /// Marks the variable's mutability from Solidity 0.7.x onwards.
    /// For cross-version compatibility use [`VariableDeclaration::mutability()`].
    #[serde(default)]
    pub mutability: Option<Mutability>,
    pub name: String,
    pub name_location: Option<String>,
    pub overrides: Option<OverrideSpecifier>,
    pub scope: NodeID,
    /// Marks whether or not the variable is a state variable before Solidity 0.7.x.
    ///
    /// After 0.7.x you must use `mutability`. For cross-version compatibility use
    /// [`VariableDeclaration::mutability()`].
    #[serde(default)]
    pub state_variable: bool,
    pub storage_location: StorageLocation,
    pub type_descriptions: TypeDescriptions,
    pub type_name: Option<TypeName>,
    pub value: Option<Expression>,
    pub visibility: Visibility,
    pub src: String,
    pub id: NodeID,
}
