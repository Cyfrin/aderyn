use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

#[derive(Clone, Debug, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    MemberAccess(MemberAccess),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    TupleExpression(TupleExpression),
    NewExpression(NewExpression),
}

impl<'de> Deserialize<'de> for Expression {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType").unwrap().as_str().unwrap();

        match node_type {
            "Literal" => Ok(Expression::Literal(serde_json::from_value(json).unwrap())),
            "Identifier" => Ok(Expression::Identifier(
                serde_json::from_value(json).unwrap(),
            )),
            "UnaryOperation" => Ok(Expression::UnaryOperation(
                serde_json::from_value(json).unwrap(),
            )),
            "BinaryOperation" => Ok(Expression::BinaryOperation(
                serde_json::from_value(json).unwrap(),
            )),
            "Conditional" => Ok(Expression::Conditional(
                serde_json::from_value(json).unwrap(),
            )),
            "Assignment" => Ok(Expression::Assignment(
                serde_json::from_value(json).unwrap(),
            )),
            "FunctionCall" => Ok(Expression::FunctionCall(
                serde_json::from_value(json).unwrap(),
            )),
            "FunctionCallOptions" => Ok(Expression::FunctionCallOptions(
                serde_json::from_value(json).unwrap(),
            )),
            "IndexAccess" => Ok(Expression::IndexAccess(
                serde_json::from_value(json).unwrap(),
            )),
            "IndexRangeAccess" => Ok(Expression::IndexRangeAccess(
                serde_json::from_value(json).unwrap(),
            )),
            "MemberAccess" => Ok(Expression::MemberAccess(
                serde_json::from_value(json).unwrap(),
            )),
            "ElementaryTypeNameExpression" => Ok(Expression::ElementaryTypeNameExpression(
                serde_json::from_value(json).unwrap(),
            )),
            "TupleExpression" => Ok(Expression::TupleExpression(
                serde_json::from_value(json).unwrap(),
            )),
            "NewExpression" => Ok(Expression::NewExpression(
                serde_json::from_value(json).unwrap(),
            )),
            _ => panic!("Invalid expression node type: {node_type:?}"),
        }
    }
}

impl Expression {
    pub fn root_expression(&self) -> Option<&Expression> {
        match self {
            Expression::Identifier(_) => Some(self),
            Expression::TupleExpression(_) => Some(self),
            Expression::Assignment(assignment) => assignment.left_hand_side.root_expression(),
            Expression::IndexAccess(index_access) => index_access.base_expression.root_expression(),
            Expression::IndexRangeAccess(index_range_access) => {
                index_range_access.base_expression.root_expression()
            }
            Expression::MemberAccess(_) => Some(self),
            _ => None,
        }
    }

    pub fn referenced_declarations(&self) -> Vec<NodeID> {
        let mut result = vec![];

        match self {
            Expression::Identifier(identifier) => {
                result.push(identifier.referenced_declaration);
            }

            Expression::Assignment(assignment) => {
                result.extend(assignment.left_hand_side.referenced_declarations());
                result.extend(assignment.right_hand_side.referenced_declarations());
            }

            Expression::IndexAccess(index_access) => {
                result.extend(index_access.base_expression.referenced_declarations());
            }

            Expression::IndexRangeAccess(index_range_access) => {
                result.extend(index_range_access.base_expression.referenced_declarations());
            }

            Expression::MemberAccess(member_access) => {
                result.extend(member_access.expression.referenced_declarations());

                if let Some(referenced_declaration) = member_access.referenced_declaration {
                    result.push(referenced_declaration);
                }
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter().flatten() {
                    result.extend(component.referenced_declarations());
                }
            }

            Expression::FunctionCall(function_call) => {
                result.extend(function_call.expression.referenced_declarations());

                for argument in function_call.arguments.iter() {
                    result.extend(argument.referenced_declarations());
                }
            }

            _ => {}
        }

        result
    }

    pub fn contains_operation(&self, operator: &str) -> bool {
        match self {
            Expression::UnaryOperation(unary_operation) => {
                unary_operation.contains_operation(operator)
            }
            Expression::BinaryOperation(binary_operation) => {
                binary_operation.contains_operation(operator)
            }
            Expression::Conditional(conditional) => conditional.contains_operation(operator),
            Expression::Assignment(assignment) => assignment.contains_operation(operator),
            Expression::FunctionCall(function_call) => function_call.contains_operation(operator),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.contains_operation(operator)
            }
            Expression::IndexAccess(index_access) => index_access.contains_operation(operator),
            Expression::IndexRangeAccess(index_range_access) => {
                index_range_access.contains_operation(operator)
            }
            Expression::MemberAccess(member_access) => member_access.contains_operation(operator),
            Expression::TupleExpression(tuple_expression) => {
                tuple_expression.contains_operation(operator)
            }
            _ => false,
        }
    }

    pub fn type_descriptions(&self) -> Option<&TypeDescriptions> {
        match self {
            Expression::Literal(Literal {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Identifier(Identifier {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::UnaryOperation(UnaryOperation {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::BinaryOperation(BinaryOperation {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Conditional(Conditional {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Assignment(Assignment {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::FunctionCall(FunctionCall {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::FunctionCallOptions(FunctionCallOptions {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::IndexAccess(IndexAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::IndexRangeAccess(IndexRangeAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::MemberAccess(MemberAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_descriptions,
                ..
            }) => Some(type_descriptions),
            Expression::TupleExpression(TupleExpression {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::NewExpression(NewExpression {
                type_descriptions, ..
            }) => Some(type_descriptions),
        }
    }

    pub fn source_line(&self, source_unit: &SourceUnit) -> std::io::Result<usize> {
        source_unit.source_line(match self {
            Expression::Literal(Literal { src, .. }) => src.as_str(),
            Expression::Identifier(Identifier { src, .. }) => src.as_str(),
            Expression::UnaryOperation(UnaryOperation { src, .. }) => src.as_str(),
            Expression::BinaryOperation(BinaryOperation { src, .. }) => src.as_str(),
            Expression::Conditional(Conditional { src, .. }) => src.as_str(),
            Expression::Assignment(Assignment { src, .. }) => src.as_str(),
            Expression::FunctionCall(FunctionCall { src, .. }) => src.as_str(),
            Expression::FunctionCallOptions(FunctionCallOptions { src, .. }) => src.as_str(),
            Expression::IndexAccess(IndexAccess { src, .. }) => src.as_str(),
            Expression::IndexRangeAccess(IndexRangeAccess { src, .. }) => src.as_str(),
            Expression::MemberAccess(MemberAccess { src, .. }) => src.as_str(),
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                src, ..
            }) => src.as_str(),
            Expression::TupleExpression(TupleExpression { src, .. }) => src.as_str(),
            Expression::NewExpression(NewExpression { src, .. }) => src.as_str(),
        })
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(expr) => expr.fmt(f)?,
            Expression::Identifier(expr) => expr.fmt(f)?,
            Expression::UnaryOperation(expr) => expr.fmt(f)?,
            Expression::BinaryOperation(expr) => expr.fmt(f)?,
            Expression::Conditional(expr) => expr.fmt(f)?,
            Expression::Assignment(expr) => expr.fmt(f)?,
            Expression::FunctionCall(expr) => expr.fmt(f)?,
            Expression::FunctionCallOptions(expr) => expr.fmt(f)?,
            Expression::IndexAccess(expr) => expr.fmt(f)?,
            Expression::IndexRangeAccess(expr) => expr.fmt(f)?,
            Expression::MemberAccess(expr) => expr.fmt(f)?,
            Expression::ElementaryTypeNameExpression(expr) => expr.fmt(f)?,
            Expression::TupleExpression(expr) => expr.fmt(f)?,
            Expression::NewExpression(expr) => expr.fmt(f)?,
            _ => {}
        }

        Ok(())
    }
}

pub struct ExpressionContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub expression: &'a Expression,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub prefix: bool,
    pub sub_expression: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl UnaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator || self.sub_expression.contains_operation(operator)
    }
}

impl Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}",
            self.sub_expression,
            self.operator.as_str()
        ))
    }
}

pub struct UnaryOperationContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub unary_operation: &'a UnaryOperation,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub common_type: TypeDescriptions,
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl BinaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator
            || self.left_expression.contains_operation(operator)
            || self.right_expression.contains_operation(operator)
    }
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.left_expression, self.operator, self.right_expression
        ))
    }
}

pub struct BinaryOperationContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub binary_operation: &'a BinaryOperation,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub true_expression: Box<Expression>,
    pub false_expression: Box<Expression>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Conditional {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.condition.contains_operation(operator)
            || self.true_expression.contains_operation(operator)
            || self.false_expression.contains_operation(operator)
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ? {} : {}",
            self.condition, self.true_expression, self.false_expression
        ))
    }
}

pub struct ConditionalContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub conditional: &'a Conditional,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub left_hand_side: Box<Expression>,
    pub right_hand_side: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Assignment {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator || self.right_hand_side.contains_operation(operator)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.left_hand_side, self.operator, self.right_hand_side
        ))
    }
}

pub struct AssignmentContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub assignment: &'a Assignment,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FunctionCallKind {
    FunctionCall,
    TypeConversion,
    StructConstructorCall,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub kind: FunctionCallKind,
    pub try_call: Option<bool>,
    pub names: Vec<String>,
    pub arguments: Vec<Expression>,
    pub expression: Box<Expression>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl FunctionCall {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for argument in self.arguments.iter() {
            if argument.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.expression))?;
        f.write_str("(")?;

        for (i, argument) in self.arguments.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{}", argument))?;
        }

        f.write_str(")")
    }
}

pub struct FunctionCallContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub function_call: &'a FunctionCall,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {
    pub names: Vec<String>,
    pub options: Vec<Expression>,
    pub arguments: Option<Vec<Expression>>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl FunctionCallOptions {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for option in self.options.iter() {
            if option.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl Display for FunctionCallOptions {
    #[allow(clippy::print_in_format_impl)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let option_count = self.options.len();

        if self.names.len() != option_count {
            eprintln!(
                "ERROR: invalid FunctionCallOptions: {:?}, {:?}",
                self.names, self.options
            );

            return Err(std::fmt::Error);
        }

        f.write_fmt(format_args!("{}", self.expression))?;

        f.write_char('{')?;

        for i in 0..option_count {
            if i > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{}: {}", self.names[i], self.options[i]))?;
        }

        f.write_char('}')?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_char('(')?;

            for (i, argument) in arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                f.write_fmt(format_args!("{}", argument))?;
            }

            f.write_char(')')?;
        }

        Ok(())
    }
}

pub struct FunctionCallOptionsContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub function_call_options: &'a FunctionCallOptions,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub base_expression: Box<Expression>,
    pub index_expression: Option<Box<Expression>>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl IndexAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.index_expression
            .as_ref()
            .map(|x| x.contains_operation(operator))
            .unwrap_or(false)
    }
}

impl Display for IndexAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}]",
            self.base_expression,
            match self.index_expression.as_ref() {
                Some(x) => format!("{x}"),
                None => String::new(),
            }
        )
    }
}

pub struct IndexAccessContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub index_access: &'a IndexAccess,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub base_expression: Box<Expression>,
    pub start_expression: Option<Box<Expression>>,
    pub end_expression: Option<Box<Expression>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl IndexRangeAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.start_expression
            .as_ref()
            .map(|expr| expr.contains_operation(operator))
            .unwrap_or(false)
            || self
                .end_expression
                .as_ref()
                .map(|expr| expr.contains_operation(operator))
                .unwrap_or(false)
    }
}

impl Display for IndexRangeAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}[", self.base_expression))?;

        if let Some(start_expression) = self.start_expression.as_ref() {
            f.write_fmt(format_args!("{}", start_expression))?;
        }

        f.write_str(":")?;

        if let Some(end_expression) = self.end_expression.as_ref() {
            f.write_fmt(format_args!("{}", end_expression))?;
        }

        f.write_str("]")
    }
}

pub struct IndexRangeAccessContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub index_range_access: &'a IndexRangeAccess,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {
    pub member_name: String,
    pub expression: Box<Expression>,
    pub referenced_declaration: Option<NodeID>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl MemberAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.expression.contains_operation(operator)
    }
}

impl Display for MemberAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.expression, self.member_name))
    }
}

pub struct MemberAccessContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub member_access: &'a MemberAccess,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub type_name: TypeName,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Display for ElementaryTypeNameExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.type_name))
    }
}

pub struct ElementaryTypeNameExpressionContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub elementary_type_name_expression: &'a ElementaryTypeNameExpression,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpression {
    pub components: Vec<Option<Expression>>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_inline_array: bool,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl TupleExpression {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for component in self.components.iter() {
            if component
                .as_ref()
                .map(|expr| expr.contains_operation(operator))
                .unwrap_or(false)
            {
                return true;
            }
        }

        false
    }
}

impl Display for TupleExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;

        for (i, component) in self.components.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }

            if let Some(component) = component {
                f.write_fmt(format_args!("{}", component))?;
            }
        }

        f.write_str(")")
    }
}

pub struct TupleExpressionContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub tuple_expression: &'a TupleExpression,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub type_descriptions: TypeDescriptions,
    pub type_name: TypeName,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub src: String,
    pub id: NodeID,
}

impl Display for NewExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("new {}", self.type_name))
    }
}

pub struct NewExpressionContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub new_expression: &'a NewExpression,
}
