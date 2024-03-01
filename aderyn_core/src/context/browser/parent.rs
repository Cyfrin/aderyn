use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetImmediateParent {
    /// Get the parent Chain of an ASTNode
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode>;
}

impl GetImmediateParent for Assignment {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for BinaryOperation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for Block {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for Conditional {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ContractDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ElementaryTypeNameExpression {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for EnumDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for EnumValue {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for EventDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ErrorDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for FunctionCall {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for FunctionCallOptions {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for FunctionDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ForStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for Identifier {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for IdentifierPath {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for IfStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ImportDirective {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for IndexAccess {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for IndexRangeAccess {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for InheritanceSpecifier {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for InlineAssembly {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for Literal {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for MemberAccess {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for NewExpression {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ModifierDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ModifierInvocation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for OverrideSpecifier {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for ParameterList {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for PragmaDirective {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for Return {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for SourceUnit {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for StructDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for StructuredDocumentation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for TupleExpression {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for UnaryOperation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for UserDefinedValueTypeDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for UsingForDirective {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for VariableDeclaration {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for VariableDeclarationStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
impl GetImmediateParent for WhileStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        context.get_parent(self.id).cloned()
    }
}
