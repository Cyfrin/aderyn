use crate::{
    ast::*,
    context::browser::ExtractImmediateChildrenIDs,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

pub trait GetImmediateChildren {
    /// Get the immediate children of an ASTNode
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
}

impl GetImmediateChildren for ASTNode {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id()?)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}

impl GetImmediateChildren for Assignment {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for BinaryOperation {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Block {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Conditional {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ContractDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ElementaryTypeNameExpression {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for EnumDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for EnumValue {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for EventDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ErrorDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for FunctionCall {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for FunctionCallOptions {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for FunctionDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ForStatement {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Identifier {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IdentifierPath {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IfStatement {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ImportDirective {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IndexAccess {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for IndexRangeAccess {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for InheritanceSpecifier {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for InlineAssembly {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Literal {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for MemberAccess {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for NewExpression {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ModifierDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ModifierInvocation {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for OverrideSpecifier {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for ParameterList {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for PragmaDirective {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for Return {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for SourceUnit {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for StructDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for StructuredDocumentation {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for TupleExpression {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for UnaryOperation {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for UserDefinedValueTypeDefinition {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for UsingForDirective {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for VariableDeclaration {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for VariableDeclarationStatement {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
impl GetImmediateChildren for WhileStatement {
    fn children<'a>(&self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        let children = ExtractImmediateChildrenIDs::from(context.nodes.get(&self.id)?)
            .extracted
            .into_iter()
            .filter_map(|x| context.nodes.get(&x))
            .collect::<Vec<_>>();
        Some(children)
    }
}
