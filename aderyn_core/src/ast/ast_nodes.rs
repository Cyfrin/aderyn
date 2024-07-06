use std::collections::BTreeMap;

use super::macros::ast_node;
use super::*;

use serde::{Deserialize, Serialize};

ast_node!(
    struct Block {
        statements: Vec<Statement>,
    }
);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum ContractDefinitionNode {
    UsingForDirective(UsingForDirective),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub base_name: UserDefinedTypeNameOrIdentifierPath,
    pub arguments: Option<Vec<Expression>>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub documentation: Option<Documentation>,
    #[serde(rename = "contractKind")]
    pub kind: ContractKind,
    #[serde(rename = "abstract")]
    pub is_abstract: Option<bool>,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub canonical_name: Option<String>,
    pub contract_dependencies: Vec<NodeID>,
    pub used_errors: Option<Vec<NodeID>>,
    pub used_events: Option<Vec<usize>>,
    #[serde(default, rename = "internalFunctionIDs")]
    pub internal_function_ids: BTreeMap<String, usize>,
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: NodeID,
    pub fully_implemented: Option<bool>,
    pub linearized_base_contracts: Option<Vec<NodeID>>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum Documentation {
    String(Option<String>),
    Structured(Option<StructuredDocumentation>),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub text: String,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub name_location: Option<String>,
    pub src: String,
    pub id: NodeID,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub members: Vec<EnumValue>,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}
