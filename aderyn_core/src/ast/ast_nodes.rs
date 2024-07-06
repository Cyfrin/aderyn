use std::collections::{BTreeMap, HashMap};

use super::macros::{ast_node, ast_node_no_partial_eq, expr_node};
use super::*;

use serde::{Deserialize, Serialize};

ast_node!(
    #[derive(Hash)]
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

ast_node!(
    #[derive(Hash)]
    struct InheritanceSpecifier {
        base_name: UserDefinedTypeNameOrIdentifierPath,
        arguments: Option<Vec<Expression>>,
    }
);

ast_node!(
    #[derive(Hash)]
    struct ContractDefinition {
        name: String,
        name_location: Option<String>,
        documentation: Option<Documentation>,
        #[serde(rename = "contractKind")]
        kind: ContractKind,
        #[serde(rename = "abstract")]
        is_abstract: Option<bool>,
        base_contracts: Vec<InheritanceSpecifier>,
        canonical_name: Option<String>,
        contract_dependencies: Vec<NodeID>,
        used_errors: Option<Vec<NodeID>>,
        used_events: Option<Vec<usize>>,
        #[serde(default, rename = "internalFunctionIDs")]
        internal_function_ids: BTreeMap<String, usize>,
        nodes: Vec<ContractDefinitionNode>,
        scope: NodeID,
        fully_implemented: Option<bool>,
        linearized_base_contracts: Option<Vec<NodeID>>,
    }
);

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

ast_node!(
    #[derive(Hash)]
    struct EnumValue {
        name: String,
        name_location: Option<String>,
    }
);

ast_node!(
    #[derive(Hash)]
    struct EnumDefinition {
        name: String,
        name_location: Option<String>,
        members: Vec<EnumValue>,
        canonical_name: Option<String>,
    }
);

ast_node!(
    #[derive(Hash)]
    struct ErrorDefinition {
        documentation: Option<Documentation>,
        error_selector: Option<String>,
        name: String,
        name_location: Option<String>,
        parameters: ParameterList,
    }
);

ast_node!(
    #[derive(Hash)]
    struct EventDefinition {
        anonymous: bool,
        documentation: Option<Documentation>,
        name: String,
        name_location: Option<String>,
        parameters: ParameterList,
        event_selector: Option<String>,
    }
);

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

expr_node!(
    #[derive(Hash)]
    struct UnaryOperation {
        operator: String,
        /// Whether the unary operator is before or after the expression (e.g. `x++` vs. `++x`)
        prefix: bool,
        sub_expression: Box<Expression>,
    }
);

expr_node!(
    #[derive(Hash)]
    struct BinaryOperation {
        common_type: TypeDescriptions,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
        operator: String,
    }
);

expr_node!(
    #[derive(Hash)]
    struct Conditional {
        condition: Box<Expression>,
        true_expression: Box<Expression>,
        false_expression: Box<Expression>,
    }
);

expr_node!(
    #[derive(Hash)]
    struct Assignment {
        left_hand_side: Box<Expression>,
        right_hand_side: Box<Expression>,
        operator: String,
    }
);

expr_node!(
    #[derive(Hash)]
    struct FunctionCall {
        kind: FunctionCallKind,
        #[serde(default)]
        try_call: bool,
        names: Vec<String>,
        arguments: Vec<Expression>,
        expression: Box<Expression>,
    }
);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FunctionCallKind {
    FunctionCall,
    TypeConversion,
    StructConstructorCall,
}

expr_node!(
    #[derive(Hash)]
    struct FunctionCallOptions {
        names: Vec<String>,
        options: Vec<Expression>,
        arguments: Option<Vec<Expression>>,
        expression: Box<Expression>,
    }
);

expr_node!(
    #[derive(Hash)]
    struct NewExpression {
        type_name: TypeName,
    }
);

expr_node!(
    #[derive(Hash)]
    struct IndexAccess {
        base_expression: Box<Expression>,
        index_expression: Option<Box<Expression>>,
    }
);

expr_node!(
    #[derive(Hash)]
    struct IndexRangeAccess {
        base_expression: Box<Expression>,
        start_expression: Option<Box<Expression>>,
        end_expression: Option<Box<Expression>>,
    }
);

expr_node!(
    #[derive(Hash)]
    struct MemberAccess {
        member_name: String,
        expression: Box<Expression>,
        referenced_declaration: Option<NodeID>,
    }
);

expr_node!(
    #[derive(Hash)]
    struct ElementaryTypeNameExpression {
        type_name: TypeName,
    }
);

expr_node!(
    #[derive(Hash)]
    struct TupleExpression {
        components: Vec<Option<Expression>>,
        is_inline_array: bool,
    }
);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Constructor,
    Function,
    Receive,
    Fallback,
    FreeFunction,
}

ast_node!(
    #[derive(Hash)]
    struct ParameterList {
        parameters: Vec<VariableDeclaration>,
    }
);

ast_node!(
    #[derive(Hash)]
    struct OverrideSpecifier {
        overrides: Vec<UserDefinedTypeNameOrIdentifierPath>,
    }
);

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

ast_node_no_partial_eq!(
    struct Identifier {
        argument_types: Option<Vec<TypeDescriptions>>,
        name: String,
        overloaded_declarations: Vec<NodeID>,
        referenced_declaration: Option<NodeID>,
        type_descriptions: TypeDescriptions,
    }
);

ast_node_no_partial_eq!(
    struct IdentifierPath {
        name: String,
        referenced_declaration: isize,
    }
);

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Identifier,
    pub local: Option<String>,
    pub name_location: Option<String>,
}

ast_node!(
    #[derive(Hash)]
    struct ImportDirective {
        file: String,
        source_unit: NodeID,
        scope: NodeID,
        absolute_path: Option<String>,
        unit_alias: String,
        name_location: Option<String>,
        symbol_aliases: Vec<SymbolAlias>,
    }
);

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

expr_node!(
    #[derive(Hash)]
    struct Literal {
        hex_value: Option<String>, // TODO: remove "Option"
        value: Option<String>,
        subdenomination: Option<String>,
        kind: LiteralKind,
    }
);

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

ast_node!(
    #[derive(Hash)]
    struct Return {
        function_return_parameters: NodeID,
        expression: Option<Expression>,
    }
);

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

ast_node!(
    #[derive(Hash)]
    struct VariableDeclaration {
        base_functions: Option<Vec<NodeID>>,
        /// Marks whether or not the variable is a constant before Solidity 0.7.x.
        ///
        /// After 0.7.x you must use `mutability`. For cross-version compatibility use
        /// [`VariableDeclaration::mutability()`].
        #[serde(default)]
        constant: bool,
        documentation: Option<Documentation>,
        function_selector: Option<String>,
        indexed: Option<bool>,
        /// Marks the variable's mutability from Solidity 0.7.x onwards.
        /// For cross-version compatibility use [`VariableDeclaration::mutability()`].
        #[serde(default)]
        mutability: Option<Mutability>,
        name: String,
        name_location: Option<String>,
        overrides: Option<OverrideSpecifier>,
        scope: NodeID,
        /// Marks whether or not the variable is a state variable before Solidity 0.7.x.
        ///
        /// After 0.7.x you must use `mutability`. For cross-version compatibility use
        /// [`VariableDeclaration::mutability()`].
        #[serde(default)]
        state_variable: bool,
        storage_location: StorageLocation,
        type_descriptions: TypeDescriptions,
        type_name: Option<TypeName>,
        value: Option<Expression>,
        visibility: Visibility,
    }
);
