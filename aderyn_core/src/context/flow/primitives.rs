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

#[derive(Debug, Clone)]
pub struct CfgPlaceholderStatement {
    pub placeholder_statement: AstNodeId,
}

impl CfgPlaceholderStatement {
    pub fn from(stmt: &PlaceholderStatement) -> Self {
        Self {
            placeholder_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_placeholder_statement(&mut self, stmt: &PlaceholderStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::PlaceholderStatement(Box::new(
            CfgPlaceholderStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgBreakStatement {
    pub break_statement: AstNodeId,
}

impl CfgBreakStatement {
    pub fn from(stmt: &Break) -> Self {
        Self {
            break_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_break_statement(&mut self, stmt: &Break) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Break(Box::new(CfgBreakStatement::from(
            stmt,
        ))))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgContinueStatement {
    pub continue_statement: AstNodeId,
}

impl CfgContinueStatement {
    pub fn from(stmt: &Continue) -> Self {
        Self {
            continue_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_continue_statement(&mut self, stmt: &Continue) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Continue(Box::new(
            CfgContinueStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgReturnStatement {
    pub return_statement: AstNodeId,
}

impl CfgReturnStatement {
    pub fn from(stmt: &Return) -> Self {
        Self {
            return_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_return_statement(&mut self, stmt: &Return) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::Return(Box::new(
            CfgReturnStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgEmitStatement {
    pub emit_statement: AstNodeId,
}

impl CfgEmitStatement {
    pub fn from(stmt: &EmitStatement) -> Self {
        Self {
            emit_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_emit_statement(&mut self, stmt: &EmitStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::EmitStatement(Box::new(
            CfgEmitStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgRevertStatement {
    pub revert_statement: AstNodeId,
}

impl CfgRevertStatement {
    pub fn from(stmt: &RevertStatement) -> Self {
        Self {
            revert_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_revert_statement(&mut self, stmt: &RevertStatement) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::RevertStatement(Box::new(
            CfgRevertStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgInlineAssemblyStatement {
    pub inline_assembly_statement: AstNodeId,
}

impl CfgInlineAssemblyStatement {
    pub fn from(stmt: &InlineAssembly) -> Self {
        Self {
            inline_assembly_statement: stmt.id,
        }
    }
}

impl Cfg {
    pub fn add_inline_assembly_statement(&mut self, stmt: &InlineAssembly) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::InlineAssembly(Box::new(
            CfgInlineAssemblyStatement::from(stmt),
        )))
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct CfgIfStatementCondition {
    pub if_stmt_condition: Option<AstNodeId>,
}

impl CfgIfStatementCondition {
    pub fn from(exp: &Expression) -> Self {
        Self {
            if_stmt_condition: exp.get_node_id(),
        }
    }
}

impl Cfg {
    pub fn add_if_statement_condition(&mut self, exp: &Expression) -> CfgNodeId {
        self.add_node(CfgNodeDescriptor::IfStatementCondition(Box::new(
            CfgIfStatementCondition::from(exp),
        )))
    }
}
