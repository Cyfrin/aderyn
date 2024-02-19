use crate::{
    ast::*,
    context::{
        browser::{
            get_all_children, get_node_ids_of_ast_nodes_that_have_ids, get_parent_chain_of_child,
        },
        workspace_context::{ASTNode, WorkspaceContext},
    },
};

/// GetImmediateChildren allows us to grab the ancestral hirearchy of a given node in the AST
/// all the way upto the ContractDefinition
pub trait GetImmediateChildren {
    /// Get the parent Chain of an ASTNode
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>>;
}

impl GetImmediateChildren for Assignment {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::Assignment(self.clone()), context)
    }
}
impl GetImmediateChildren for BinaryOperation {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::BinaryOperation(self.clone()), context)
    }
}
impl GetImmediateChildren for Block {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::Block(self.clone()), context)
    }
}
impl GetImmediateChildren for Conditional {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::Conditional(self.clone()), context)
    }
}
impl GetImmediateChildren for ContractDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::ContractDefinition(self.clone()), context)
    }
}
impl GetImmediateChildren for ElementaryTypeNameExpression {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(
            &ASTNode::ElementaryTypeNameExpression(self.clone()),
            context,
        )
    }
}
impl GetImmediateChildren for EnumDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::EnumDefinition(self.clone()), context)
    }
}
impl GetImmediateChildren for EnumValue {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::EnumValue(self.clone()), context)
    }
}
impl GetImmediateChildren for EventDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::EventDefinition(self.clone()), context)
    }
}
impl GetImmediateChildren for ErrorDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::ErrorDefinition(self.clone()), context)
    }
}
impl GetImmediateChildren for FunctionCall {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::FunctionCall(self.clone()), context)
    }
}
impl GetImmediateChildren for FunctionCallOptions {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::FunctionCallOptions(self.clone()), context)
    }
}
impl GetImmediateChildren for FunctionDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::FunctionDefinition(self.clone()), context)
    }
}
impl GetImmediateChildren for ForStatement {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::ForStatement(self.clone()), context)
    }
}
impl GetImmediateChildren for Identifier {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::Identifier(self.clone()), context)
    }
}
impl GetImmediateChildren for IdentifierPath {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::IdentifierPath(self.clone()), context)
    }
}
impl GetImmediateChildren for IfStatement {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::IfStatement(self.clone()), context)
    }
}
impl GetImmediateChildren for ImportDirective {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::ImportDirective(self.clone()), context)
    }
}
impl GetImmediateChildren for IndexAccess {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::IndexAccess(self.clone()), context)
    }
}
impl GetImmediateChildren for IndexRangeAccess {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::IndexRangeAccess(self.clone()), context)
    }
}
impl GetImmediateChildren for InheritanceSpecifier {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::InheritanceSpecifier(self.clone()), context)
    }
}
impl GetImmediateChildren for InlineAssembly {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::InlineAssembly(self.clone()), context)
    }
}
impl GetImmediateChildren for Literal {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::Literal(self.clone()), context)
    }
}
impl GetImmediateChildren for MemberAccess {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::MemberAccess(self.clone()), context)
    }
}
impl GetImmediateChildren for NewExpression {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::NewExpression(self.clone()), context)
    }
}
impl GetImmediateChildren for ModifierDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::ModifierDefinition(self.clone()), context)
    }
}
impl GetImmediateChildren for ModifierInvocation {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::ModifierInvocation(self.clone()), context)
    }
}
impl GetImmediateChildren for OverrideSpecifier {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::OverrideSpecifier(self.clone()), context)
    }
}
impl GetImmediateChildren for ParameterList {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::ParameterList(self.clone()), context)
    }
}
impl GetImmediateChildren for PragmaDirective {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::PragmaDirective(self.clone()), context)
    }
}
impl GetImmediateChildren for Return {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::Return(self.clone()), context)
    }
}
impl GetImmediateChildren for SourceUnit {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::SourceUnit(self.clone()), context)
    }
}
impl GetImmediateChildren for StructDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::StructDefinition(self.clone()), context)
    }
}
impl GetImmediateChildren for StructuredDocumentation {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::StructuredDocumentation(self.clone()), context)
    }
}
impl GetImmediateChildren for TupleExpression {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::TupleExpression(self.clone()), context)
    }
}
impl GetImmediateChildren for UnaryOperation {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::UnaryOperation(self.clone()), context)
    }
}
impl GetImmediateChildren for UserDefinedValueTypeDefinition {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(
            &ASTNode::UserDefinedValueTypeDefinition(self.clone()),
            context,
        )
    }
}
impl GetImmediateChildren for UsingForDirective {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::UsingForDirective(self.clone()), context)
    }
}
impl GetImmediateChildren for VariableDeclaration {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::VariableDeclaration(self.clone()), context)
    }
}
impl GetImmediateChildren for VariableDeclarationStatement {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(
            &ASTNode::VariableDeclarationStatement(self.clone()),
            context,
        )
    }
}
impl GetImmediateChildren for WhileStatement {
    fn immediate_children(&self, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
        get_children_of_node(&ASTNode::WhileStatement(self.clone()), context)
    }
}

pub fn get_children_of_node(node: &ASTNode, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
    let current_node = vec![node.clone()];
    let current_nodes_id = get_node_ids_of_ast_nodes_that_have_ids(&current_node);
    if current_nodes_id.is_empty() {
        return None;
    }
    let (current_node_id, _) = current_nodes_id.iter().next().unwrap();

    let all_children = get_all_children(node);
    let all_children_ids = get_node_ids_of_ast_nodes_that_have_ids(&all_children);

    let mut immediate_children = vec![];

    for (k, v) in all_children_ids {
        let parent_chain = get_parent_chain_of_child(k, context);
        if parent_chain.len() > 1 {
            let first_parent = vec![parent_chain[1].clone()];
            let parents_id = get_node_ids_of_ast_nodes_that_have_ids(&first_parent);
            if !parents_id.is_empty() {
                let (parent_id, _) = parents_id.iter().next().unwrap();
                if parent_id == current_node_id {
                    immediate_children.push(v.clone())
                }
            }
        }
    }

    let mut hooks = vec![];

    for (idx, child) in immediate_children.iter().enumerate() {
        let trace = child.src().unwrap();
        let char_index = trace
            .split(":")
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        hooks.push((char_index, idx))
    }

    hooks.sort_by(|a, b| a.0.cmp(&b.0));

    let mut rearranged_children = vec![];

    for index in &hooks {
        rearranged_children.push(immediate_children[index.1].clone());
    }

    Some(rearranged_children)
}
