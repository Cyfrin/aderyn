use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetClosestParentOfTypeX {
    /// Get the parent Chain of an ASTNode
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode>;
}

impl GetClosestParentOfTypeX for ASTNode {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id()?, node_type)
    }
}

impl GetClosestParentOfTypeX for Assignment {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for BinaryOperation {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for Block {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for Conditional {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ContractDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ElementaryTypeNameExpression {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for EnumDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for EnumValue {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for EventDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ErrorDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for FunctionCall {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for FunctionCallOptions {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for FunctionDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ForStatement {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for Identifier {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for IdentifierPath {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for IfStatement {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ImportDirective {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for IndexAccess {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for IndexRangeAccess {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for InheritanceSpecifier {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for InlineAssembly {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for Literal {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for MemberAccess {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for NewExpression {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ModifierDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ModifierInvocation {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for OverrideSpecifier {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for ParameterList {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for PragmaDirective {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for Return {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for SourceUnit {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for StructDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for StructuredDocumentation {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for TupleExpression {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for UnaryOperation {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for UserDefinedValueTypeDefinition {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for UsingForDirective {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for VariableDeclaration {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for VariableDeclarationStatement {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
impl GetClosestParentOfTypeX for WhileStatement {
    fn closest_parent_of_type<'a>(
        &self,
        context: &'a WorkspaceContext,
        node_type: NodeType,
    ) -> Option<&'a ASTNode> {
        context.get_closest_parent(self.id, node_type)
    }
}
