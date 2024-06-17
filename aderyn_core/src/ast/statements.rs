use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Block),
    Break(Break),
    Continue(Continue),
    DoWhileStatement(DoWhileStatement),
    PlaceholderStatement(PlaceholderStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    UncheckedBlock(Block),
    Return(Return),
    RevertStatement(RevertStatement),
    ExpressionStatement(ExpressionStatement),
    InlineAssembly(InlineAssembly),
}

impl Statement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                Some(variable_declaration_statement.id)
            }
            Statement::IfStatement(if_statement) => Some(if_statement.id),
            Statement::ForStatement(for_statement) => Some(for_statement.id),
            Statement::WhileStatement(while_statement) => Some(while_statement.id),
            Statement::EmitStatement(emit_statement) => emit_statement.event_call.get_node_id(),
            Statement::UncheckedBlock(unchecked_statement) => Some(unchecked_statement.id),
            Statement::Return(return_statement) => Some(return_statement.id),
            Statement::RevertStatement(revert_statement) => Some(revert_statement.error_call.id),
            Statement::ExpressionStatement(expression_statement) => {
                expression_statement.expression.get_node_id()
            }
            Statement::InlineAssembly(inline_assembly) => Some(inline_assembly.id),
            Statement::TryStatement(_) => None,
            Statement::Block(block) => Some(block.id),
            Statement::Break(break_statement) => Some(break_statement.id),
            Statement::Continue(continue_statement) => Some(continue_statement.id),
            Statement::DoWhileStatement(do_while_statement) => Some(do_while_statement.id),
            Statement::PlaceholderStatement(placeholder) => Some(placeholder.id),
        }
    }
}

impl Node for Statement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                variable_declaration_statement.accept(visitor)
            }
            Statement::IfStatement(if_statement) => if_statement.accept(visitor),
            Statement::ForStatement(for_statement) => for_statement.accept(visitor),
            Statement::WhileStatement(while_statement) => while_statement.accept(visitor),
            Statement::EmitStatement(emit_statement) => emit_statement.accept(visitor),
            Statement::TryStatement(try_statement) => try_statement.accept(visitor),
            Statement::UncheckedBlock(unchecked_statement) => unchecked_statement.accept(visitor),
            Statement::Return(return_statement) => return_statement.accept(visitor),
            Statement::RevertStatement(revert_statement) => revert_statement.accept(visitor),
            Statement::ExpressionStatement(expression_statement) => {
                expression_statement.accept(visitor)
            }
            Statement::InlineAssembly(inline_assembly) => inline_assembly.accept(visitor),
            Statement::Block(block) => block.accept(visitor),
            Statement::Break(break_statement) => break_statement.accept(visitor),
            Statement::Continue(continue_statement) => continue_statement.accept(visitor),
            Statement::DoWhileStatement(do_while_statement) => do_while_statement.accept(visitor),
            Statement::PlaceholderStatement(placeholder_statement) => {
                placeholder_statement.accept(visitor)
            }
        }
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl Statement {
    pub fn is_return_statement(&self) -> bool {
        matches!(self, Statement::Return(_))
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VariableDeclarationStatement(stmt) => stmt.fmt(f),
            Statement::IfStatement(stmt) => stmt.fmt(f),
            Statement::ForStatement(stmt) => stmt.fmt(f),
            Statement::WhileStatement(stmt) => stmt.fmt(f),
            Statement::EmitStatement(stmt) => stmt.fmt(f),
            Statement::TryStatement(stmt) => stmt.fmt(f),
            Statement::RevertStatement(stmt) => stmt.fmt(f),
            Statement::UncheckedBlock(stmt) => stmt.fmt(f),
            Statement::Return(stmt) => stmt.fmt(f),
            Statement::ExpressionStatement(stmt) => stmt.fmt(f),
            Statement::InlineAssembly(..) => {
                f.write_str("assembly { /* WARNING: not implemented */ }")
            }
            _ => f.write_str("unrecognized!"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_expression_statement(self)? {
            self.expression.accept(visitor)?;
        }
        visitor.end_visit_expression_statement(self)
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.expression))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub assignments: Vec<Option<NodeID>>,
    pub declarations: Vec<Option<VariableDeclaration>>,
    pub initial_value: Option<Expression>,
    pub src: String,
    pub id: NodeID,
}

impl Node for VariableDeclarationStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_variable_declaration_statement(self)? {
            for declaration in &self.declarations {
                if declaration.is_some() {
                    declaration.as_ref().unwrap().accept(visitor)?;
                }
            }
            if self.initial_value.is_some() {
                self.initial_value.as_ref().unwrap().accept(visitor)?;
            }
            self.accept_metadata(visitor)?;
        }
        visitor.end_visit_variable_declaration_statement(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let declaration_ids = self
            .declarations
            .iter()
            .flatten()
            .map(|x| x.id)
            .collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, declaration_ids)?;
        if let Some(initial_value) = &self.initial_value {
            if let Some(id) = initial_value.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for VariableDeclarationStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.declarations.len() == 1 {
            if let Some(declaration) = self.declarations[0].as_ref() {
                f.write_fmt(format_args!("{declaration}"))?;
            } else {
                f.write_str("()")?;
            }
        } else {
            f.write_str("(")?;

            for (i, declaration) in self.declarations.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                if let Some(declaration) = declaration {
                    f.write_fmt(format_args!("{declaration}"))?;
                }
            }

            f.write_str(")")?;
        }

        if let Some(initial_value) = self.initial_value.as_ref() {
            f.write_fmt(format_args!(" = {initial_value}"))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum BlockOrStatement {
    Block(Box<Block>),
    Statement(Box<Statement>),
}

impl BlockOrStatement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            BlockOrStatement::Block(block) => Some(block.id),
            BlockOrStatement::Statement(statement) => statement.get_node_id(),
        }
    }
}

impl Node for BlockOrStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            BlockOrStatement::Block(block) => block.accept(visitor),
            BlockOrStatement::Statement(statement) => statement.accept(visitor),
        }
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl BlockOrStatement {
    pub fn contains_returns(&self) -> bool {
        match self {
            BlockOrStatement::Block(block) => block
                .statements
                .last()
                .map(|s| BlockOrStatement::Statement(Box::new(s.clone())).contains_returns())
                .unwrap_or(false),

            BlockOrStatement::Statement(statement) => match statement.as_ref() {
                Statement::Return(Return { .. }) => true,

                Statement::IfStatement(IfStatement {
                    true_body,
                    false_body,
                    ..
                }) => {
                    if !true_body.contains_returns() {
                        return false;
                    }

                    match false_body {
                        Some(false_body) => false_body.contains_returns(),
                        None => true,
                    }
                }

                _ => false,
            },
        }
    }
}

impl Display for BlockOrStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockOrStatement::Block(block) => block.fmt(f),
            BlockOrStatement::Statement(statement) => statement.fmt(f),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub condition: Expression,
    pub true_body: BlockOrStatement,
    pub false_body: Option<BlockOrStatement>,
    pub src: String,
    pub id: NodeID,
}

impl Node for IfStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_if_statement(self)? {
            self.condition.accept(visitor)?;
            self.true_body.accept(visitor)?;
            if self.false_body.is_some() {
                self.false_body.as_ref().unwrap().accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_if_statement(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(cond_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![cond_id])?;
        }
        if let Some(true_body_id) = self.true_body.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![true_body_id])?;
        }
        if let Some(false_body) = &self.false_body {
            if let Some(false_body_id) = false_body.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![false_body_id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("if ({}) {}", self.condition, self.true_body))?;

        if let Some(false_body) = self.false_body.as_ref() {
            f.write_fmt(format_args!("\nelse {false_body}"))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub initialization_expression: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub loop_expression: Option<Box<Statement>>,
    pub body: BlockOrStatement,
    pub src: String,
    pub id: NodeID,
}

impl Node for ForStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_for_statement(self)? {
            if self.initialization_expression.is_some() {
                self.initialization_expression
                    .as_ref()
                    .unwrap()
                    .accept(visitor)?;
            }
            if self.condition.is_some() {
                self.condition.as_ref().unwrap().accept(visitor)?;
            }
            if self.loop_expression.is_some() {
                self.loop_expression.as_ref().unwrap().accept(visitor)?;
            }
            self.body.accept(visitor)?;
            self.accept_metadata(visitor)?;
        }
        visitor.end_visit_for_statement(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(initialization_expr) = &self.initialization_expression {
            if let Some(expr_id) = initialization_expr.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![expr_id])?;
            }
        }
        if let Some(condition) = &self.condition {
            if let Some(cond_id) = condition.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![cond_id])?;
            }
        }
        if let Some(loop_expr) = &self.loop_expression {
            if let Some(loop_id) = loop_expr.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![loop_id])?;
            }
        }
        if let Some(body_id) = self.body.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![body_id])?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("for (")?;

        if let Some(initialization_expression) = self.initialization_expression.as_ref() {
            f.write_fmt(format_args!("{initialization_expression}"))?;
        }

        f.write_str("; ")?;

        if let Some(condition) = self.condition.as_ref() {
            f.write_fmt(format_args!("{condition}"))?;
        }

        f.write_str("; ")?;

        if let Some(loop_expression) = self.loop_expression.as_ref() {
            f.write_fmt(format_args!("{loop_expression}"))?;
        }

        f.write_fmt(format_args!(") {}", self.body))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockOrStatement,
    pub src: String,
    pub id: NodeID,
}

impl Node for WhileStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_while_statement(self)? {
            self.condition.accept(visitor)?;
            self.body.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_while_statement(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(cond_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![cond_id])?;
        }

        if let Some(body_id) = self.body.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![body_id])?;
        }
        Ok(())
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("while ({}) {}", self.condition, self.body))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
    pub body: Block,
    pub condition: Expression,
}

impl Node for DoWhileStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_do_while_statement(self)? {
            self.condition.accept(visitor)?;
            self.body.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_do_visit_while_statement(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(cond_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![cond_id])?;
        }
        visitor.visit_immediate_children(self.id, vec![self.body.id])?;
        Ok(())
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for DoWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("do {} while({});", self.body, self.condition))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub event_call: Expression,
}

impl Node for EmitStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_emit_statement(self)? {
            self.event_call.accept(visitor)?;
        }
        visitor.end_visit_emit_statement(self)
    }
}

impl Display for EmitStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("emit {}", self.event_call))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub clauses: Vec<TryCatchClause>,
    pub external_call: FunctionCall,
}

impl Node for TryStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_try_statement(self)? {
            self.external_call.accept(visitor)?;
            list_accept(&self.clauses, visitor)?;
        }
        visitor.end_visit_try_statement(self)
    }
}

impl Display for TryStatement {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub error_call: FunctionCall,
}

impl Node for RevertStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_revert_statement(self)? {
            self.error_call.accept(visitor)?;
        }
        visitor.end_visit_revert_statement(self)
    }
}

impl Display for RevertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("revert {}", self.error_call))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub block: Block,
    pub error_name: Option<String>,
    pub parameters: Option<ParameterList>,
}

impl Node for TryCatchClause {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_try_catch_clause(self)? {
            if self.parameters.is_some() {
                self.parameters.as_ref().unwrap().accept(visitor)?;
            }
            self.block.accept(visitor)?;
        }
        visitor.end_visit_try_catch_clause(self)
    }
}

impl Display for TryCatchClause {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub function_return_parameters: NodeID,
    pub expression: Option<Expression>,
    pub src: String,
    pub id: NodeID,
}

impl Node for Return {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_return(self)? && self.expression.is_some() {
            self.expression.as_ref().unwrap().accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_return(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr) = &self.expression {
            if let Some(expr_id) = expr.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![expr_id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("return")?;

        if let Some(expression) = self.expression.as_ref() {
            f.write_fmt(format_args!(" {expression}"))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    #[serde(rename = "AST")]
    pub ast: Option<YulBlock>,
    pub evm_version: Option<String>,
    pub external_references: Vec<ExternalReference>,
    pub operations: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Node for InlineAssembly {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_inline_assembly(self)? && self.ast.is_some() {
            self.ast.as_ref().unwrap().accept(visitor)?;
        }
        visitor.end_visit_inline_assembly(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Break {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
}

impl Node for Break {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_break_statement(self)?;
        visitor.end_visit_break_statement(self)
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("break;")
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Continue {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
}

impl Node for Continue {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_continue_statement(self)?;
        visitor.end_visit_continue_statement(self)
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for Continue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("continue;")
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderStatement {
    pub id: NodeID,
    pub src: String,
    pub documentation: Option<String>,
}

impl Node for PlaceholderStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_placeholder_statement(self)?;
        visitor.end_visit_placeholder_statement(self)
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
