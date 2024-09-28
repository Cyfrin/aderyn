use crate::context::{browser::Peek, workspace_context::WorkspaceContext};

use super::{
    primitives::{CfgExpressionStatement, CfgVariableDeclarationStatement},
    voids::{CfgEndNode, CfgStartNode},
    CfgNodeDescriptor,
};

impl CfgNodeDescriptor {
    pub fn display(&self, context: &WorkspaceContext) -> String {
        match self {
            CfgNodeDescriptor::Start(n) => n.peek(),
            CfgNodeDescriptor::End(n) => n.peek(),
            CfgNodeDescriptor::VariableDeclarationStatement(n) => n.peek(context),
            CfgNodeDescriptor::ExpressionStatement(n) => n.peek(context),
            CfgNodeDescriptor::Block(_) => todo!(),
        }
    }
}

impl CfgStartNode {
    pub fn peek(&self) -> String {
        match self {
            CfgStartNode::Start => String::from("START"),
            CfgStartNode::StartBlock(ast_id) => format!("START BLOCK ({})", ast_id),
        }
    }
}

impl CfgEndNode {
    pub fn peek(&self) -> String {
        match self {
            CfgEndNode::End => String::from("END"),
            CfgEndNode::EndBlock(ast_id) => format!("END BLOCK ({})", ast_id),
        }
    }
}

impl CfgVariableDeclarationStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!(
            "Variable Decl. Stmt ({})",
            self.variable_declaration_statement
        );
        if let Some(node) = context.nodes.get(&self.variable_declaration_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgExpressionStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Expression Stmt ({})", self.expression_statement);
        if let Some(node) = context.nodes.get(&self.expression_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}
