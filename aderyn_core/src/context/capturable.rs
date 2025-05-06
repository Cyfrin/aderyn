use crate::ast::{ContractDefinition, *};

use super::{
    macros::generate_capturable_methods,
    workspace::{ASTNode, WorkspaceContext},
};

generate_capturable_methods! {
    Assignment,
    BinaryOperation,
    Block,
    Conditional,
    ContractDefinition,
    ElementaryTypeNameExpression,
    EnumDefinition,
    EnumValue,
    EventDefinition,
    ExpressionStatement,
    ErrorDefinition,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
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
    ModifierDefinition,
    ModifierInvocation,
    OverrideSpecifier,
    ParameterList,
    PragmaDirective,
    Return,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    TupleExpression,
    UnaryOperation,
    UncheckedBlock,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    DoWhileStatement,
    Break,
    Continue,
    PlaceholderStatement,
}

impl From<&&ContractDefinition> for Capturable {
    fn from(value: &&ContractDefinition) -> Self {
        #[allow(suspicious_double_ref_op)]
        Self::ContractDefinition(value.clone().clone())
    }
}

impl From<&&ModifierInvocation> for Capturable {
    fn from(value: &&ModifierInvocation) -> Self {
        #[allow(suspicious_double_ref_op)]
        Self::ModifierInvocation(value.clone().clone())
    }
}

impl From<YulFunctionCall> for Capturable {
    fn from(value: YulFunctionCall) -> Self {
        Self::YulFunctionCall(value)
    }
}

impl From<&YulIdentifier> for Capturable {
    fn from(value: &YulIdentifier) -> Self {
        Self::YulIdentifier(value.clone())
    }
}

impl From<YulLiteral> for Capturable {
    fn from(value: YulLiteral) -> Self {
        Self::YulLiteral(value)
    }
}
