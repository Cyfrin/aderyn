use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetImmediateParent {
    /// Get the parent Chain of an ASTNode
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode>;
}

impl GetImmediateParent for ASTNode {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id()?)
    }
}

impl GetImmediateParent for Assignment {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for BinaryOperation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for Block {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for Conditional {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ContractDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ElementaryTypeNameExpression {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for EnumDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for EnumValue {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for EventDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ErrorDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for FunctionCall {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for FunctionCallOptions {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for FunctionDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ForStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for Identifier {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for IdentifierPath {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for IfStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ImportDirective {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for IndexAccess {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for IndexRangeAccess {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for InheritanceSpecifier {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for InlineAssembly {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for Literal {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for MemberAccess {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for NewExpression {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ModifierDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ModifierInvocation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for OverrideSpecifier {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for ParameterList {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for PragmaDirective {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for Return {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for SourceUnit {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for StructDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for StructuredDocumentation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for TupleExpression {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for UnaryOperation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for UserDefinedValueTypeDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for UsingForDirective {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for VariableDeclaration {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for VariableDeclarationStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
impl GetImmediateParent for WhileStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        context.get_parent(self.id)
    }
}
