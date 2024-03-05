use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetParentChain {
    /// Get the parent Chain of an ASTNode
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
}

impl GetParentChain for Assignment {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for BinaryOperation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Block {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Conditional {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ContractDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ElementaryTypeNameExpression {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for EnumDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for EnumValue {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for EventDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ErrorDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for FunctionCall {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for FunctionCallOptions {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for FunctionDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ForStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Identifier {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IdentifierPath {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IfStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ImportDirective {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IndexAccess {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IndexRangeAccess {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for InheritanceSpecifier {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for InlineAssembly {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Literal {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for MemberAccess {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for NewExpression {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ModifierDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ModifierInvocation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for OverrideSpecifier {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ParameterList {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for PragmaDirective {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Return {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for SourceUnit {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for StructDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for StructuredDocumentation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for TupleExpression {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for UnaryOperation {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for UserDefinedValueTypeDefinition {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for UsingForDirective {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for VariableDeclaration {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for VariableDeclarationStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for WhileStatement {
    fn parent<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
