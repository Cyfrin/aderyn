use std::collections::BTreeMap;

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
