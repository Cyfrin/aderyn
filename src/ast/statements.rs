use super::yul::*;
use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Statement {
    VariableDeclarationStatement(VariableDeclarationStatement),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    UncheckedBlock(Block),
    Return(Return),
    RevertStatement(RevertStatement),
    ExpressionStatement(ExpressionStatement),
    Block(Block),
    InlineAssembly(InlineAssembly),
    Continue { src: String, id: NodeID },
    Break { src: String, id: NodeID },
    PlaceholderStatement { src: String, id: NodeID },
}

impl<'de> Deserialize<'de> for Statement {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType").unwrap().as_str().unwrap();

        match node_type {
            "VariableDeclarationStatement" => Ok(Statement::VariableDeclarationStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "IfStatement" => Ok(Statement::IfStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "ForStatement" => Ok(Statement::ForStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "WhileStatement" => Ok(Statement::WhileStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "DoWhileStatement" => Ok(Statement::DoWhileStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "EmitStatement" => Ok(Statement::EmitStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "TryStatement" => Ok(Statement::TryStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "UncheckedBlock" => Ok(Statement::UncheckedBlock(
                serde_json::from_value(json).unwrap(),
            )),
            "Return" => Ok(Statement::Return(serde_json::from_value(json).unwrap())),
            "RevertStatement" => Ok(Statement::RevertStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "ExpressionStatement" => Ok(Statement::ExpressionStatement(
                serde_json::from_value(json).unwrap(),
            )),
            "Block" => Ok(Statement::Block(serde_json::from_value(json).unwrap())),
            "InlineAssembly" => Ok(Statement::InlineAssembly(
                serde_json::from_value(json).unwrap(),
            )),
            "Continue" => Ok(Statement::Continue {
                src: json.get("src").unwrap().as_str().unwrap().to_string(),
                id: json.get("id").unwrap().as_i64().unwrap(),
            }),
            "Break" => Ok(Statement::Continue {
                src: json.get("src").unwrap().as_str().unwrap().to_string(),
                id: json.get("id").unwrap().as_i64().unwrap(),
            }),
            "PlaceholderStatement" => Ok(Statement::Continue {
                src: json.get("src").unwrap().as_str().unwrap().to_string(),
                id: json.get("id").unwrap().as_i64().unwrap(),
            }),
            _ => panic!("Invalid statement node type: {node_type:?}"),
        }
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
            Statement::DoWhileStatement(stmt) => stmt.fmt(f),
            Statement::EmitStatement(stmt) => stmt.fmt(f),
            Statement::TryStatement(stmt) => stmt.fmt(f),
            Statement::RevertStatement(stmt) => stmt.fmt(f),
            Statement::UncheckedBlock(stmt) => stmt.fmt(f),
            Statement::Return(stmt) => stmt.fmt(f),
            Statement::ExpressionStatement(stmt) => stmt.fmt(f),
            Statement::Block(stmt) => stmt.fmt(f),
            Statement::InlineAssembly(_) => {
                write!(f, "assembly {{ /* WARNING: not implemented */ }}")
            }
            Statement::Continue { .. } => write!(f, "continue"),
            Statement::Break { .. } => write!(f, "break"),
            Statement::PlaceholderStatement { .. } => write!(f, "_"),
        }
    }
}

pub struct StatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.expression))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub assignments: Vec<Option<NodeID>>,
    pub declarations: Vec<Option<VariableDeclaration>>,
    pub initial_value: Option<Expression>,
    pub src: String,
    pub id: NodeID,
}

impl Display for VariableDeclarationStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.declarations.len() == 1 {
            if let Some(declaration) = self.declarations[0].as_ref() {
                f.write_fmt(format_args!("{}", declaration))?;
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
                    f.write_fmt(format_args!("{}", declaration))?;
                }
            }

            f.write_str(")")?;
        }

        if let Some(initial_value) = self.initial_value.as_ref() {
            f.write_fmt(format_args!(" = {}", initial_value))?;
        }

        Ok(())
    }
}

pub struct VariableDeclarationStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub variable_declaration_statement: &'a VariableDeclarationStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum BlockOrStatement {
    Block(Box<Block>),
    Statement(Box<Statement>),
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

pub struct BlockOrStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub block_or_statement: &'a BlockOrStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub condition: Expression,
    pub true_body: BlockOrStatement,
    pub false_body: Option<BlockOrStatement>,
    pub src: String,
    pub id: NodeID,
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("if ({}) {}", self.condition, self.true_body))?;

        if let Some(false_body) = self.false_body.as_ref() {
            f.write_fmt(format_args!("\nelse {}", false_body))?;
        }

        Ok(())
    }
}

pub struct IfStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub if_statement: &'a IfStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub initialization_expression: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub loop_expression: Option<Box<Statement>>,
    pub body: BlockOrStatement,
    pub src: String,
    pub id: NodeID,
}

impl Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("for (")?;

        if let Some(initialization_expression) = self.initialization_expression.as_ref() {
            f.write_fmt(format_args!("{}", initialization_expression))?;
        }

        f.write_str("; ")?;

        if let Some(condition) = self.condition.as_ref() {
            f.write_fmt(format_args!("{}", condition))?;
        }

        f.write_str("; ")?;

        if let Some(loop_expression) = self.loop_expression.as_ref() {
            f.write_fmt(format_args!("{}", loop_expression))?;
        }

        f.write_fmt(format_args!(") {}", self.body))
    }
}

pub struct ForStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub for_statement: &'a ForStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockOrStatement,
    pub src: String,
    pub id: NodeID,
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("while ({}) {}", self.condition, self.body))
    }
}

pub struct WhileStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub while_statement: &'a WhileStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub body: BlockOrStatement,
    pub condition: Expression,
    pub src: String,
    pub id: NodeID,
}

impl Display for DoWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("do {} while ({})", self.body, self.condition))
    }
}

pub struct DoWhileStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub do_while_statement: &'a DoWhileStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub event_call: Expression,
}

impl Display for EmitStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("emit {}", self.event_call))
    }
}

pub struct EmitStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub emit_statement: &'a EmitStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub clauses: Vec<TryCatchClause>,
    pub external_call: FunctionCall,
}

impl Display for TryStatement {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

pub struct TryStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub try_statement: &'a TryStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub error_call: FunctionCall,
}

impl Display for RevertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("revert {}", self.error_call))
    }
}

pub struct RevertStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub revert_statement: &'a RevertStatement,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub block: Block,
    pub error_name: Option<String>,
    pub parameters: Option<ParameterList>,
}

impl Display for TryCatchClause {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub function_return_parameters: NodeID,
    pub expression: Option<Expression>,
    pub src: String,
    pub id: NodeID,
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("return")?;

        if let Some(expression) = self.expression.as_ref() {
            f.write_fmt(format_args!(" {}", expression))?;
        }

        Ok(())
    }
}

pub struct ReturnContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub return_statement: &'a Return,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
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

pub struct InlineAssemblyContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
}
