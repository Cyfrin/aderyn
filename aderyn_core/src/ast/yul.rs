// Original source: https://github.com/camden-smallwood/solidity-rs
use crate::visitor::ast_visitor::{list_accept, ASTConstVisitor, Node};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, hash::Hasher};

use super::*;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ExternalReference {
    Untagged(ExternalReferenceData),
    Tagged(HashMap<String, ExternalReferenceData>),
}

impl Hash for ExternalReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ExternalReference::Untagged(data) => {
                0.hash(state); // A unique value to denote the Untagged variant
                data.hash(state);
            }
            ExternalReference::Tagged(map) => {
                1.hash(state); // A unique value to denote the Tagged variant

                // Create a vector of references to the map's key-value pairs
                let mut pairs: Vec<_> = map.iter().collect();

                // Sort the vector by keys
                pairs.sort_by(|a, b| a.0.cmp(b.0));

                // Hash each pair in the sorted order
                for (key, value) in pairs {
                    key.hash(state);
                    value.hash(state);
                }
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExternalReferenceData {
    declaration: NodeID,
    is_offset: bool,
    is_slot: bool,
    src: String,
    value_size: NodeID,
}

#[derive(Clone, Debug, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum YulExpression {
    YulLiteral(YulLiteral),
    YulIdentifier(YulIdentifier),
    YulFunctionCall(YulFunctionCall),
}

impl Node for YulExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            YulExpression::YulLiteral(node) => node.accept(visitor),
            YulExpression::YulIdentifier(node) => node.accept(visitor),
            YulExpression::YulFunctionCall(node) => node.accept(visitor),
        }
    }
}

impl<'de> Deserialize<'de> for YulExpression {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType").unwrap().as_str().unwrap();

        match node_type {
            "YulLiteral" => Ok(YulExpression::YulLiteral(
                serde_json::from_value(json).unwrap(),
            )),
            "YulIdentifier" => Ok(YulExpression::YulIdentifier(
                serde_json::from_value(json).unwrap(),
            )),
            "YulFunctionCall" => Ok(YulExpression::YulFunctionCall(
                serde_json::from_value(json).unwrap(),
            )),
            _ => panic!("Invalid yul expression node type: {node_type}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteral {
    pub kind: YulLiteralKind,
    pub value: Option<String>,
    pub hex_value: Option<String>,
}

impl Node for YulLiteral {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_yul_literal(self)?;
        visitor.end_visit_yul_literal(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum YulLiteralKind {
    Bool,
    Number,
    String,
    HexString,
    Address,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub name: String,
}

impl Node for YulIdentifier {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_yul_identifier(self)?;
        visitor.end_visit_yul_identifier(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub function_name: YulIdentifier,
    pub arguments: Vec<YulExpression>,
}

impl Node for YulFunctionCall {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_function_call(self)? {
            self.function_name.accept(visitor)?;
            list_accept(&self.arguments, visitor)?;
        }
        visitor.end_visit_yul_function_call(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub statements: Vec<YulStatement>,
}

impl Node for YulBlock {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_block(self)? {
            list_accept(&self.statements, visitor)?;
        }
        visitor.end_visit_yul_block(self)
    }
}

#[derive(Clone, Debug, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum YulStatement {
    YulIf(YulIf),
    YulSwitch(YulSwitch),
    YulForLoop(YulForLoop),
    YulAssignment(YulAssignment),
    YulVariableDeclaration(YulVariableDeclaration),
    YulExpressionStatement(YulExpressionStatement),
    YulFunctionDefinition(YulFunctionDefinition),
    YulBlock(YulBlock),
    YulLeave,
    YulBreak,
    YulContinue,
}

impl Node for YulStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            YulStatement::YulIf(node) => node.accept(visitor),
            YulStatement::YulSwitch(node) => node.accept(visitor),
            YulStatement::YulForLoop(node) => node.accept(visitor),
            YulStatement::YulAssignment(node) => node.accept(visitor),
            YulStatement::YulVariableDeclaration(node) => node.accept(visitor),
            YulStatement::YulExpressionStatement(node) => node.accept(visitor),
            YulStatement::YulFunctionDefinition(node) => node.accept(visitor),
            YulStatement::YulBlock(node) => node.accept(visitor),
            YulStatement::YulLeave => Ok(()),
            YulStatement::YulBreak => Ok(()),
            YulStatement::YulContinue => Ok(()),
        }
    }
}

impl<'de> Deserialize<'de> for YulStatement {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType").unwrap().as_str().unwrap();

        match node_type {
            "YulIf" => Ok(YulStatement::YulIf(serde_json::from_value(json).unwrap())),
            "YulSwitch" => Ok(YulStatement::YulSwitch(
                serde_json::from_value(json).unwrap(),
            )),
            "YulForLoop" => Ok(YulStatement::YulForLoop(
                serde_json::from_value(json).unwrap(),
            )),
            "YulAssignment" => Ok(YulStatement::YulAssignment(
                serde_json::from_value(json).unwrap(),
            )),
            "YulVariableDeclaration" => Ok(YulStatement::YulVariableDeclaration(
                serde_json::from_value(json).unwrap(),
            )),
            "YulExpressionStatement" => Ok(YulStatement::YulExpressionStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "YulFunctionDefinition" => Ok(YulStatement::YulFunctionDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "YulBlock" => Ok(YulStatement::YulBlock(
                serde_json::from_value(json).unwrap(),
            )),
            "YulLeave" => Ok(YulStatement::YulLeave),
            "YulBreak" => Ok(YulStatement::YulBreak),
            "YulContinue" => Ok(YulStatement::YulContinue),
            _ => panic!("Invalid yul statement node type: {node_type}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulIf {
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
pub struct YulSwitch {
    pub cases: Vec<YulCase>,
    pub expression: YulExpression,
}

impl Node for YulSwitch {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_switch(self)? {
            self.expression.accept(visitor)?;
            list_accept(&self.cases, visitor)?;
        }
        visitor.end_visit_yul_switch(self)
    }
}

#[derive(Clone, Debug, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub body: YulBlock,
    pub value: Option<YulExpression>,
}

impl Node for YulCase {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_case(self)? {
            self.body.accept(visitor)?;
            if let Some(value) = &self.value {
                value.accept(visitor)?;
            }
        }
        visitor.end_visit_yul_case(self)
    }
}

impl<'de> Deserialize<'de> for YulCase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let body = json.get("body").unwrap();
        let value = json.get("value").unwrap();

        Ok(YulCase {
            body: serde_json::from_value(body.clone()).unwrap(),
            value: if matches!(value.as_str(), Some("default")) {
                None
            } else {
                Some(serde_json::from_value(value.clone()).unwrap())
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulForLoop {
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
pub struct YulAssignment {
    pub value: YulExpression,
    pub variable_names: Vec<YulIdentifier>,
}

impl Node for YulAssignment {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_assignment(self)? {
            self.value.accept(visitor)?;
            list_accept(&self.variable_names, visitor)?;
        }
        visitor.end_visit_yul_assignment(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub value: Option<YulExpression>,
    pub variables: Vec<YulTypedName>,
}

impl Node for YulVariableDeclaration {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_variable_declaration(self)? {
            if let Some(value) = &self.value {
                value.accept(visitor)?;
            }
            list_accept(&self.variables, visitor)?;
        }
        visitor.end_visit_yul_variable_declaration(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub r#type: String,
    pub name: String,
}

impl Node for YulTypedName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_yul_typed_name(self)?;
        visitor.end_visit_yul_typed_name(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
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
pub struct YulFunctionDefinition {
    pub name: String,
    pub parameters: Option<Vec<YulTypedName>>,
    pub return_parameters: Option<Vec<YulTypedName>>,
    pub body: YulBlock,
}

impl Node for YulFunctionDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_yul_function_definition(self)? {
            if let Some(parameters) = &self.parameters {
                list_accept(parameters, visitor)?;
            }
            if let Some(return_parameters) = &self.return_parameters {
                list_accept(return_parameters, visitor)?;
            }
            self.body.accept(visitor)?;
        }
        visitor.end_visit_yul_function_definition(self)
    }
}
