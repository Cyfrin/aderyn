use crate::{
    ast::macros::*,
    ast::*,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

generate_ast_methods!(
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Conditional,
    ContractDefinition,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    EmitStatement,
    EnumDefinition,
    EnumValue,
    EventDefinition,
    ErrorDefinition,
    ExpressionStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    ForStatement,
    Identifier,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    MemberAccess,
    NewExpression,
    Mapping,
    ModifierDefinition,
    ModifierInvocation,
    OverrideSpecifier,
    ParameterList,
    PragmaDirective,
    Return,
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    TryStatement,
    TryCatchClause,
    TupleExpression,
    UnaryOperation,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    DoWhileStatement,
    Break,
    Continue,
    PlaceholderStatement,
);

impl From<&YulFunctionCall> for ASTNode {
    fn from(value: &YulFunctionCall) -> Self {
        ASTNode::YulFunctionCall(value.clone())
    }
}

impl From<&YulIdentifier> for ASTNode {
    fn from(value: &YulIdentifier) -> Self {
        ASTNode::YulIdentifier(value.clone())
    }
}

impl From<&YulLiteral> for ASTNode {
    fn from(value: &YulLiteral) -> Self {
        ASTNode::YulLiteral(value.clone())
    }
}
