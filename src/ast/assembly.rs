use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::visitor::ast_visitor::{ASTConstVisitor, Node};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub src: String,
    pub statements: Vec<YulStatement>,
}

impl Node for YulBlock {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_block(self)? {
            for statement in &self.statements {
                statement.accept(visitor)?;
            }
        }
        visitor.end_visit_yul_block(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum YulStatement {
    YulAssignment(YulAssignment),
    YulBlock(YulBlock),
    YulBreak(YulBreak),
    YulContinue(YulContinue),
    YulExpressionStatement(YulExpressionStatement),
    YulLeave(YulLeave),
    YulForLoop(YulForLoop),
    YulFunctionDefinition(YulFunctionDefinition),
    YulIf(YulIf),
    YulSwitch(YulSwitch),
    YulVariableDeclaration(YulVariableDeclaration),
}

impl Node for YulStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            YulStatement::YulAssignment(yul_assignment) => yul_assignment.accept(visitor),
            YulStatement::YulBlock(yul_block) => yul_block.accept(visitor),
            YulStatement::YulBreak(_yul_break) => Ok(()),
            YulStatement::YulContinue(_yul_continue) => Ok(()),
            YulStatement::YulExpressionStatement(yul_expression_statement) => {
                yul_expression_statement.accept(visitor)
            }
            YulStatement::YulLeave(_yul_leave) => Ok(()),
            YulStatement::YulForLoop(yul_for_loop) => yul_for_loop.accept(visitor),
            YulStatement::YulFunctionDefinition(yul_function_definition) => {
                yul_function_definition.accept(visitor)
            }
            YulStatement::YulIf(yul_if) => yul_if.accept(visitor),
            YulStatement::YulSwitch(yul_switch) => yul_switch.accept(visitor),
            YulStatement::YulVariableDeclaration(yul_variable_declaration) => {
                yul_variable_declaration.accept(visitor)
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub src: String,
    pub name: String,
    pub value: YulExpression,
}

impl Node for YulAssignment {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_assignment(self)? {
            self.value.accept(visitor)?;
        }
        visitor.end_visit_yul_assignment(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulBreak {
    pub src: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulContinue {
    pub src: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub src: String,
    pub expression: YulExpression,
}

impl Node for YulExpressionStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_expression_statement(self)? {
            self.expression.accept(visitor)?;
        }
        visitor.end_visit_yul_expression_statement(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulLeave {
    pub src: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulForLoop {
    pub src: String,
    pub pre: YulBlock,
    pub condition: YulExpression,
    pub post: YulBlock,
    pub body: YulBlock,
}

impl Node for YulForLoop {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_for_loop(self)? {
            self.pre.accept(visitor)?;
            self.condition.accept(visitor)?;
            self.post.accept(visitor)?;
            self.body.accept(visitor)?;
        }
        visitor.end_visit_yul_for_loop(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionDefinition {
    pub src: String,
    pub name: String,
    pub parameters: Vec<YulTypedName>,
    pub return_variables: Vec<YulTypedName>,
    pub body: YulBlock,
}

impl Node for YulFunctionDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_function_definition(self)? {
            self.body.accept(visitor)?;
        }
        visitor.end_visit_yul_function_definition(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum YulExpression {
    YulFunctionCall(YulFunctionCall),
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
}

impl Node for YulExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            YulExpression::YulFunctionCall(yul_function_call) => yul_function_call.accept(visitor),
            YulExpression::YulIdentifier(_yul_identifier) => Ok(()),
            YulExpression::YulLiteral(_yul_literal) => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub src: String,
    pub function_name: String,
    pub arguments: Vec<YulExpression>,
}

impl Node for YulFunctionCall {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_function_call(self)? {
            for argument in &self.arguments {
                argument.accept(visitor)?;
            }
        }
        visitor.end_visit_yul_function_call(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulIf {
    pub src: String,
    pub condition: YulExpression,
    pub body: YulBlock,
}

impl Node for YulIf {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_if(self)? {
            self.condition.accept(visitor)?;
            self.body.accept(visitor)?;
        }
        visitor.end_visit_yul_if(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub src: String,
    pub value: Option<YulExpression>,
    pub variables: Vec<YulTypedName>,
}

impl Node for YulVariableDeclaration {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_variable_declaration(self)? {
            if let Some(value) = &self.value {
                value.accept(visitor)?;
            }
        }
        visitor.end_visit_yul_variable_declaration(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub src: String,
    pub expression: YulExpression,
    pub cases: Vec<YulCase>,
    pub default: Option<YulBlock>,
}

impl Node for YulSwitch {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_switch(self)? {
            self.expression.accept(visitor)?;
            for case in &self.cases {
                case.accept(visitor)?;
            }
            if let Some(default) = &self.default {
                default.accept(visitor)?;
            }
        }
        visitor.end_visit_yul_switch(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub src: String,
    pub value: YulLiteral,
    pub body: YulBlock,
}

impl Node for YulCase {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_case(self)? {
            self.body.accept(visitor)?;
        }
        visitor.end_visit_yul_case(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum YulLiteral {
    YulLiteralValue(YulLiteralValue),
    YulLiteralHexValue(YulLiteralHexValue),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteralValue {
    pub kind: String,
    pub src: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteralHexValue {
    pub hex_value: String,
    pub kind: String,
    pub src: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub src: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub src: String,
    pub name: String,
    pub typ: String,
}
