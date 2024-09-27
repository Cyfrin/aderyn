use super::{AstNodeId, Cfg, CfgNodeDescriptor, CfgNodeId};
use crate::ast::*;

// Control flow graph definitions nodes
#[derive(Debug, Clone)]
pub struct CfgVariableDeclarationStatement {
    pub variable_declaration_statement: AstNodeId,
}

impl CfgVariableDeclarationStatement {
    pub fn from(stmt: &VariableDeclarationStatement) -> Self {
        Self {
            variable_declaration_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_variable_declaration_statement(
        &mut self,
        stmt: &VariableDeclarationStatement,
    ) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::VariableDeclarationStatement(Box::new(
            CfgVariableDeclarationStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgExpressionStatement {
    pub expression_statement: AstNodeId,
}

impl CfgExpressionStatement {
    pub fn from(stmt: &ExpressionStatement) -> Self {
        Self {
            expression_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_expression_statement(&mut self, stmt: &ExpressionStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::ExpressionStatement(Box::new(
            CfgExpressionStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

/// Helper functions
impl Cfg {
    pub fn add_start_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Start)
    }
    pub fn add_end_node(&mut self) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::End)
    }
}
