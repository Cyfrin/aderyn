use crate::context::{browser::Peek, workspace_context::WorkspaceContext};

use super::{
    primitives::{CfgExpressionStatement, CfgVariableDeclarationStatement},
    CfgNodeDescriptor,
};

impl CfgNodeDescriptor {
    pub fn display(&self, context: &WorkspaceContext) -> String {
        match self {
            CfgNodeDescriptor::Start => "START".to_string(),
            CfgNodeDescriptor::End => "END".to_string(),
            CfgNodeDescriptor::VariableDeclarationStatement(n) => n.peek(context),
            CfgNodeDescriptor::ExpressionStatement(n) => n.peek(context),
            CfgNodeDescriptor::Block(_) => todo!(),
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
