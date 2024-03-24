use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetParentChain {
    /// Get the parent Chain of an ASTNode
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
}

impl GetParentChain for ASTNode {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id()?))
    }
}

impl GetParentChain for Assignment {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for BinaryOperation {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Block {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Conditional {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ContractDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ElementaryTypeNameExpression {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for EnumDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for EnumValue {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for EventDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ErrorDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for FunctionCall {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for FunctionCallOptions {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for FunctionDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ForStatement {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Identifier {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IdentifierPath {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IfStatement {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ImportDirective {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IndexAccess {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for IndexRangeAccess {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for InheritanceSpecifier {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for InlineAssembly {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Literal {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for MemberAccess {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for NewExpression {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ModifierDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ModifierInvocation {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for OverrideSpecifier {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for ParameterList {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for PragmaDirective {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for Return {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for SourceUnit {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for StructDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for StructuredDocumentation {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for TupleExpression {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for UnaryOperation {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for UserDefinedValueTypeDefinition {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for UsingForDirective {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for VariableDeclaration {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for VariableDeclarationStatement {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
impl GetParentChain for WhileStatement {
    fn parent_chain<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        Some(context.get_parent_chain(self.id))
    }
}
