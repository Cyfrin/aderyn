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

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub function_name: YulIdentifier,
    pub arguments: Vec<YulExpression>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub statements: Vec<YulStatement>,
}

pub struct YulBlockContext<'a, 'b> {
    pub yul_blocks: &'b mut Vec<&'a YulBlock>,
    pub yul_block: &'a YulBlock,
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

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub cases: Vec<YulCase>,
    pub expression: YulExpression,
}

#[derive(Clone, Debug, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub body: YulBlock,
    pub value: Option<YulExpression>,
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

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub value: YulExpression,
    pub variable_names: Vec<YulIdentifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub value: Option<YulExpression>,
    pub variables: Vec<YulTypedName>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub r#type: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub expression: YulExpression,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionDefinition {
    pub name: String,
    pub parameters: Option<Vec<YulTypedName>>,
    pub return_parameters: Option<Vec<YulTypedName>>,
    pub body: YulBlock,
}
