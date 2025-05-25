use crate::context::{browser::Peek, workspace::WorkspaceContext};

use super::{primitives::*, voids::*, CfgBreakStatement, CfgNodeDescriptor};

impl CfgNodeDescriptor {
    pub fn display(&self, context: &WorkspaceContext) -> String {
        match self {
            // Voids
            CfgNodeDescriptor::Start(n) => n.peek(),
            CfgNodeDescriptor::End(n) => n.peek(),

            // Primitives
            CfgNodeDescriptor::VariableDeclarationStatement(n) => n.peek(context),
            CfgNodeDescriptor::ExpressionStatement(n) => n.peek(context),
            CfgNodeDescriptor::PlaceholderStatement(n) => n.peek(),
            CfgNodeDescriptor::Break(n) => n.peek(),
            CfgNodeDescriptor::Continue(n) => n.peek(),
            CfgNodeDescriptor::Return(n) => n.peek(context),
            CfgNodeDescriptor::EmitStatement(n) => n.peek(context),
            CfgNodeDescriptor::RevertStatement(n) => n.peek(context),
            CfgNodeDescriptor::InlineAssembly(n) => n.peek(context),
            CfgNodeDescriptor::TryStatement(n) => n.peek(context),
            CfgNodeDescriptor::IfStatementCondition(n) => n.peek(context),
            CfgNodeDescriptor::WhileStatementCondition(n) => n.peek(context),
            CfgNodeDescriptor::ForStatementCondition(n) => n.peek(context),
            CfgNodeDescriptor::DoWhileStatementCondition(n) => n.peek(context),

            // Reducibles
            CfgNodeDescriptor::IfStatement(_) => String::from("REDUCIBLE IF-STATEMENT"),
            CfgNodeDescriptor::WhileStatement(_) => String::from("REDUCIBLE WHILE-STATEMENT"),
            CfgNodeDescriptor::DoWhileStatement(_) => String::from("REDUCIBLE DO-WHILE-STATEMENT"),
            CfgNodeDescriptor::ForStatement(_) => String::from("REDUCIBLE FOR-STATEMENT"),
            CfgNodeDescriptor::Block(_) => String::from("REDUCIBLE BLOCK"),
            CfgNodeDescriptor::UncheckedBlock(_) => String::from("REDUCIBLE UNCHECKED BLOCK"),
        }
    }
}

impl CfgStartNode {
    pub fn peek(&self) -> String {
        match self {
            CfgStartNode::Start => String::from("START"),
            CfgStartNode::StartBlock(ast_id) => format!("START BLOCK ({})", ast_id),
            CfgStartNode::StartUncheckedBlock(ast_id) => {
                format!("START UNCHECKED BLOCK ({})", ast_id)
            }
            CfgStartNode::StartModifierBody(ast_id) => {
                format!("START MODIFIER BODY {}", ast_id)
            }
            CfgStartNode::StartFunctionBody(ast_id) => {
                format!("START FUNCTION BODY {}", ast_id)
            }
            CfgStartNode::StartIf(ast_id) => format!("START IF ({})", ast_id),
            CfgStartNode::StartIfCond => String::from("START IF COND"),
            CfgStartNode::StartIfTrue => String::from("START IF TRUE BRANCH"),
            CfgStartNode::StartIfFalse => String::from("START IF FALSE BRANCH"),
            CfgStartNode::StartWhile(ast_id) => format!("START WHILE ({})", ast_id),
            CfgStartNode::StartWhileCond => String::from("START WHILE COND"),
            CfgStartNode::StartWhileBody => String::from("START WHILE BODY"),
            CfgStartNode::StartFor(ast_id) => format!("START FOR ({})", ast_id),
            CfgStartNode::StartForInitExp => String::from("START FOR INIT_EXP"),
            CfgStartNode::StartForCond => String::from("START FOR COND"),
            CfgStartNode::StartForLoopExp => String::from("START FOR LOOP_EXP"),
            CfgStartNode::StartForBody => String::from("START FOR BODY"),
            CfgStartNode::StartDoWhile(ast_id) => format!("START DO WHILE ({})", ast_id),
            CfgStartNode::StartDoWhileCond => String::from("START DO WHILE COND"),
            CfgStartNode::StartDoWhileBody => String::from("START DO WHILE BODY"),
        }
    }
}

impl CfgEndNode {
    pub fn peek(&self) -> String {
        match self {
            CfgEndNode::End => String::from("END"),
            CfgEndNode::EndBlock(ast_id) => format!("END BLOCK ({})", ast_id),
            CfgEndNode::EndUncheckedBlock(ast_id) => format!("END UNCHECKED BLOCK ({})", ast_id),
            CfgEndNode::EndModifierBody(ast_id) => format!("END MODIFIER BODY ({})", ast_id),
            CfgEndNode::EndFunctionBody(ast_id) => format!("END FUNCTION BODY ({})", ast_id),
            CfgEndNode::EndIf(ast_id) => format!("END IF ({})", ast_id),
            CfgEndNode::EndIfCond => String::from("END IF COND"),
            CfgEndNode::EndIfTrue => String::from("END IF TRUE BRANCH"),
            CfgEndNode::EndIfFalse => String::from("END IF FALSE BRANCH"),
            CfgEndNode::EndWhile(ast_id) => format!("END WHILE ({})", ast_id),
            CfgEndNode::EndWhileCond => String::from("END WHILE COND"),
            CfgEndNode::EndWhileBody => String::from("END WHILE BODY"),
            CfgEndNode::EndFor(ast_id) => format!("END FOR ({})", ast_id),
            CfgEndNode::EndForInitExp => String::from("END FOR INIT_EXP"),
            CfgEndNode::EndForCond => String::from("END FOR COND"),
            CfgEndNode::EndForLoopExp => String::from("END FOR LOOP_EXP"),
            CfgEndNode::EndForBody => String::from("END FOR BODY"),
            CfgEndNode::EndDoWhile(ast_id) => format!("END DO WHILE ({})", ast_id),
            CfgEndNode::EndDoWhileCond => String::from("END DO WHILE COND"),
            CfgEndNode::EndDoWhileBody => String::from("END DO WHILE BODY"),
        }
    }
}

impl CfgVariableDeclarationStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Variable Decl. Stmt ({})", self.variable_declaration_statement);
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

impl CfgBreakStatement {
    pub fn peek(&self) -> String {
        format!("BREAK ({})", &self.break_statement)
    }
}

impl CfgContinueStatement {
    pub fn peek(&self) -> String {
        format!("CONTINUE ({})", &self.continue_statement)
    }
}

impl CfgReturnStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Return Stmt ({})", self.return_statement);
        if let Some(node) = context.nodes.get(&self.return_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgEmitStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Emit Stmt ({})", self.emit_statement);
        if let Some(node) = context.nodes.get(&self.emit_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgRevertStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Revert Stmt ({})", self.revert_statement);
        if let Some(node) = context.nodes.get(&self.revert_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgInlineAssemblyStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Inline Assembly Stmt ({})", self.inline_assembly_statement);
        if let Some(node) = context.nodes.get(&self.inline_assembly_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgPlaceholderStatement {
    pub fn peek(&self) -> String {
        let mut content = format!("Placeholder statement ({})", self.placeholder_statement);
        content.push_str(": \n_");
        content
    }
}

impl CfgIfStatementCondition {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let Some(if_cond) = self.if_stmt_condition else {
            return String::from("If Cond");
        };
        let mut content = format!("If Cond ({})", if_cond);
        if let Some(node) = context.nodes.get(&if_cond) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgWhileStatementCondition {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let Some(while_cond) = self.while_stmt_condition else {
            return String::from("While Cond");
        };
        let mut content = format!("While Cond ({})", while_cond);
        if let Some(node) = context.nodes.get(&while_cond) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgForStatementCondition {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let Some(for_stmt) = self.for_stmt_condition else {
            return String::from("For Cond");
        };
        let mut content = format!("For Cond ({})", for_stmt);
        if let Some(node) = context.nodes.get(&for_stmt) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgDoWhileStatementCondition {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let Some(for_stmt) = self.do_while_stmt_condition else {
            return String::from("Do While Cond");
        };
        let mut content = format!("Do While Cond ({})", for_stmt);
        if let Some(node) = context.nodes.get(&for_stmt) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgTryStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Try Stmt ({})", self.try_statement);
        if let Some(node) = context.nodes.get(&self.try_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}
