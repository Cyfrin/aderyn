use crate::{
    ast::*,
    context::browser::ExtractImmediateChildrenIDs,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetImmediateChildren {
    /// Get the immediate children of an ASTNode
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
}

impl GetImmediateChildren for ASTNode {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id()?)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}

impl GetImmediateChildren for Assignment {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for BinaryOperation {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Block {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Conditional {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ContractDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ElementaryTypeNameExpression {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for EnumDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for EnumValue {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for EventDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ErrorDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for FunctionCall {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for FunctionCallOptions {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for FunctionDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ForStatement {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Identifier {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IdentifierPath {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IfStatement {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ImportDirective {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IndexAccess {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IndexRangeAccess {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for InheritanceSpecifier {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for InlineAssembly {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Literal {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for MemberAccess {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for NewExpression {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ModifierDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ModifierInvocation {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for OverrideSpecifier {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ParameterList {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for PragmaDirective {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Return {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for SourceUnit {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for StructDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for StructuredDocumentation {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for TupleExpression {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for UnaryOperation {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for UserDefinedValueTypeDefinition {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for UsingForDirective {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for VariableDeclaration {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for VariableDeclarationStatement {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for WhileStatement {
    fn immediate_children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
