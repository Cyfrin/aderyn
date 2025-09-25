use super::{
    macros::{ast_node, ast_node_no_partial_eq, expr_node, node_group, stmt_node},
    *,
};
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

node_group! {
    SourceUnitNode;

    ContractDefinition,
    EnumDefinition,
    ErrorDefinition,
    EventDefinition,
    FunctionDefinition,
    ImportDirective,
    PragmaDirective,
    StructDefinition,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
}

node_group! {
    Expression;

    Assignment,
    BinaryOperation,
    Conditional,
    ElementaryTypeNameExpression,
    FunctionCall,
    FunctionCallOptions,
    Identifier,
    IndexAccess,
    IndexRangeAccess,
    Literal,
    MemberAccess,
    NewExpression,
    TupleExpression,
    UnaryOperation,
}

node_group! {
    Statement;

    Block,
    Break,
    Continue,
    DoWhileStatement,
    EmitStatement,
    ExpressionStatement,
    ForStatement,
    IfStatement,
    InlineAssembly,
    PlaceholderStatement,
    Return,
    RevertStatement,
    TryStatement,
    UncheckedBlock,
    VariableDeclarationStatement,
    WhileStatement,
}

node_group! {
    ContractDefinitionNode;

    EnumDefinition,
    ErrorDefinition,
    EventDefinition,
    FunctionDefinition,
    ModifierDefinition,
    StructDefinition,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
}

#[derive(Clone, Debug, Eq, Serialize, PartialEq, Hash)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    /// A string representing the type name.
    ///
    /// This variant applies to older compiler versions.
    Raw(String),
    UserDefinedTypeName(UserDefinedTypeName),
}

impl<'de> Deserialize<'de> for TypeName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType");

        if node_type.is_none() {
            return Ok(TypeName::Raw(json.to_string()));
        }

        let type_name = node_type.unwrap().as_str().unwrap();

        match type_name {
            "FunctionTypeName" => {
                Ok(TypeName::FunctionTypeName(serde_json::from_value(json).unwrap()))
            }
            "ArrayTypeName" => Ok(TypeName::ArrayTypeName(serde_json::from_value(json).unwrap())),
            "Mapping" => Ok(TypeName::Mapping(serde_json::from_value(json).unwrap())),
            "UserDefinedTypeName" => {
                Ok(TypeName::UserDefinedTypeName(serde_json::from_value(json).unwrap()))
            }
            "ElementaryTypeName" => {
                Ok(TypeName::ElementaryTypeName(serde_json::from_value(json).unwrap()))
            }
            _ => panic!("Unrecognized type name {type_name}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(tag = "nodeType")]
pub enum UserDefinedTypeNameOrIdentifierPath {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(tag = "nodeType")]
pub enum ExpressionOrVariableDeclarationStatement {
    ExpressionStatement(ExpressionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
}

impl ExpressionOrVariableDeclarationStatement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            ExpressionOrVariableDeclarationStatement::ExpressionStatement(ref expression) => {
                Some(expression.id)
            }
            ExpressionOrVariableDeclarationStatement::VariableDeclarationStatement(
                ref vd_stmnt,
            ) => Some(vd_stmnt.id),
        }
    }
}

ast_node!(
    #[derive(Hash)]
    struct Block {
        statements: Vec<Statement>,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct UncheckedBlock {
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
        #[serde(default, rename = "abstract")]
        is_abstract: bool,
        base_contracts: Vec<InheritanceSpecifier>,
        canonical_name: Option<String>,
        contract_dependencies: Vec<NodeID>,
        used_errors: Option<Vec<NodeID>>,
        used_events: Option<Vec<usize>>,
        #[serde(default, rename = "internalFunctionIDs")]
        internal_function_ids: BTreeMap<String, usize>,
        nodes: Vec<ContractDefinitionNode>,
        scope: NodeID,
        fully_implemented: bool,
        linearized_base_contracts: Vec<NodeID>,
    }
);

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum Documentation {
    String(Option<String>),
    Structured(Option<StructuredDocumentation>),
}

ast_node!(
    #[derive(Hash)]
    struct StructuredDocumentation {
        text: String,
    }
);

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

ast_node!(
    #[derive(Hash)]
    struct FunctionDefinition {
        base_functions: Option<Vec<NodeID>>,
        body: Option<Block>,
        documentation: Option<Documentation>,
        function_selector: Option<String>,
        implemented: bool,
        /// The kind of function this node defines. Only valid for Solidity versions 0.5.x and
        /// above.
        ///
        /// For cross-version compatibility use [`FunctionDefinition::kind()`].
        kind: Option<FunctionKind>,
        #[serde(default)]
        /// For cross-version compatibility use [`FunctionDefinition::state_mutability()`].
        state_mutability: Option<StateMutability>,
        #[serde(default, rename = "virtual")]
        is_virtual: bool,
        /// Whether or not this function is the constructor. Only valid for Solidity versions below
        /// 0.5.x.
        ///
        /// After 0.5.x you must use `kind`. For cross-version compatibility use
        /// [`FunctionDefinition::kind()`].
        #[serde(default)]
        is_constructor: bool,
        /// Whether or not this function is constant (view or pure). Only valid for Solidity
        /// versions below 0.5.x.
        ///
        /// After 0.5.x you must use `state_mutability`. For cross-version compatibility use
        /// [`FunctionDefinition::state_mutability()`].
        #[serde(default)]
        is_declared_const: bool,
        /// Whether or not this function is payable. Only valid for Solidity versions below
        /// 0.5.x.
        ///
        /// After 0.5.x you must use `state_mutability`. For cross-version compatibility use
        /// [`FunctionDefinition::state_mutability()`].
        #[serde(default)]
        is_payable: bool,
        modifiers: Vec<ModifierInvocation>,
        name: String,
        name_location: Option<String>,
        overrides: Option<OverrideSpecifier>,
        parameters: ParameterList,
        return_parameters: ParameterList,
        scope: NodeID,
        super_function: Option<NodeID>,
        visibility: Visibility,
    }
);

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
        referenced_declaration: i64,
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
        hex_value: String,
        value: Option<String>,
        subdenomination: Option<String>,
        kind: LiteralKind,
    }
);

ast_node!(
    #[derive(Hash)]
    struct ModifierDefinition {
        body: Option<Block>,
        base_modifiers: Option<Vec<usize>>,
        overrides: Option<OverrideSpecifier>,
        documentation: Option<Documentation>,
        name: String,
        name_location: Option<String>,
        parameters: ParameterList,
        #[serde(default, rename = "virtual")]
        is_virtual: bool,
        visibility: Visibility,
    }
);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    ModifierInvocation,
    BaseConstructorSpecifier,
}

ast_node!(
    #[derive(Hash)]
    struct ModifierInvocation {
        arguments: Option<Vec<Expression>>,
        modifier_name: IdentifierOrIdentifierPath,
        kind: Option<ModifierInvocationKind>,
    }
);

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum IdentifierOrIdentifierPath {
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
}

ast_node!(
    #[derive(Hash)]
    struct PragmaDirective {
        literals: Vec<String>,
    }
);

ast_node!(
    struct SourceUnit {
        license: Option<String>,
        nodes: Vec<SourceUnitNode>,
        exported_symbols: Option<HashMap<String, Vec<NodeID>>>,
        absolute_path: Option<String>,

        #[serde(skip_serializing)]
        source: Option<String>,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct ExpressionStatement {
        expression: Expression,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct VariableDeclarationStatement {
        assignments: Vec<Option<NodeID>>,
        declarations: Vec<Option<VariableDeclaration>>,
        initial_value: Option<Expression>,
    }
);

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum BlockOrStatement {
    Block(Box<Block>),
    Statement(Box<Statement>),
}

stmt_node!(
    #[derive(Hash)]
    struct IfStatement {
        condition: Expression,
        true_body: BlockOrStatement,
        false_body: Option<BlockOrStatement>,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct ForStatement {
        initialization_expression: Option<Box<ExpressionOrVariableDeclarationStatement>>,
        condition: Option<Expression>,
        loop_expression: Option<Box<ExpressionStatement>>,
        body: BlockOrStatement,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct DoWhileStatement {
        body: Block,
        condition: Expression,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct EmitStatement {
        event_call: FunctionCall,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct TryStatement {
        clauses: Vec<TryCatchClause>,
        external_call: FunctionCall,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct RevertStatement {
        error_call: FunctionCall,
    }
);

ast_node!(
    #[derive(Hash)]
    struct TryCatchClause {
        block: Block,
        error_name: String,
        parameters: Option<ParameterList>,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct Return {
        function_return_parameters: Option<NodeID>, /* When returning in a modifier, this can be
                                                     * none */
        expression: Option<Expression>,
    }
);

ast_node!(
    #[derive(Hash)]
    struct InlineAssembly {
        #[serde(rename = "AST")]
        ast: Option<YulBlock>,
        evm_version: Option<String>,
        external_references: Vec<ExternalReference>,
        operations: Option<String>,
    }
);

stmt_node!(
    #[derive(Hash)]
    struct Break {}
);

stmt_node!(
    #[derive(Hash)]
    struct Continue {}
);

stmt_node!(
    #[derive(Hash)]
    struct PlaceholderStatement {}
);

stmt_node!(
    #[derive(Hash)]
    struct WhileStatement {
        condition: Expression,
        body: BlockOrStatement,
    }
);

ast_node!(
    #[derive(Hash)]
    struct StructDefinition {
        name: String,
        name_location: Option<String>,
        visibility: Visibility,
        members: Vec<VariableDeclaration>,
        scope: NodeID,
        canonical_name: Option<String>,
    }
);

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
}

ast_node_no_partial_eq!(
    struct ElementaryTypeName {
        state_mutability: Option<StateMutability>,
        name: String,
        type_descriptions: TypeDescriptions,
    }
);

ast_node_no_partial_eq!(
    struct UserDefinedTypeName {
        path_node: Option<IdentifierPath>,
        referenced_declaration: NodeID,
        name: Option<String>,
        type_descriptions: TypeDescriptions,
        contract_scope: Option<String>,
    }
);

ast_node!(
    #[derive(Hash)]
    struct FunctionTypeName {
        visibility: Visibility,
        state_mutability: StateMutability,
        parameter_types: ParameterList,
        return_parameter_types: ParameterList,
        type_descriptions: TypeDescriptions,
    }
);

ast_node!(
    #[derive(Hash)]
    struct ArrayTypeName {
        base_type: Box<TypeName>,
        length: Box<Option<Expression>>,
        type_descriptions: TypeDescriptions,
    }
);

ast_node!(
    #[derive(Hash)]
    struct Mapping {
        key_type: Box<TypeName>,
        value_type: Box<TypeName>,
        type_descriptions: TypeDescriptions,
    }
);

ast_node!(
    #[derive(Hash)]
    struct UserDefinedValueTypeDefinition {
        underlying_type: TypeName,
        name: String,
        name_location: Option<String>,
        canonical_name: Option<String>,
    }
);

ast_node!(
    #[derive(Hash)]
    struct UsingForDirective {
        function_list: Option<Vec<UsingForFunctionItem>>,
        #[serde(default)]
        global: bool,
        library_name: Option<UserDefinedTypeNameOrIdentifierPath>,
        type_name: Option<TypeName>,
    }
);

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
    Transient,
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
