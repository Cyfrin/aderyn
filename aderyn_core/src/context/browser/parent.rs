use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

use super::get_parent_chain_of_child;

/// GetImmediateParent allows us to grab the ancestral hirearchy of a given node in the AST
/// all the way upto the ContractDefinition
pub trait GetImmediateParent {
    /// Get the parent Chain of an ASTNode
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode>;
}

impl GetImmediateParent for Assignment {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for BinaryOperation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for Block {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for Conditional {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ContractDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ElementaryTypeNameExpression {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for EnumDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for EnumValue {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for EventDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ErrorDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for FunctionCall {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for FunctionCallOptions {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for FunctionDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ForStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for Identifier {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for IdentifierPath {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for IfStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ImportDirective {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for IndexAccess {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for IndexRangeAccess {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for InheritanceSpecifier {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for InlineAssembly {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for Literal {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for MemberAccess {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for NewExpression {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ModifierDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ModifierInvocation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for OverrideSpecifier {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for ParameterList {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for PragmaDirective {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for Return {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for SourceUnit {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for StructDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for StructuredDocumentation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for TupleExpression {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for UnaryOperation {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for UserDefinedValueTypeDefinition {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for UsingForDirective {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for VariableDeclaration {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for VariableDeclarationStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
impl GetImmediateParent for WhileStatement {
    fn parent(&self, context: &WorkspaceContext) -> Option<ASTNode> {
        let chain = get_parent_chain_of_child(self.id, context);
        if chain.len() > 1 {
            return Some(chain[1].clone());
        }
        None
    }
}
