use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub src: String,
    pub id: NodeID,
}

impl Display for ErrorDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("error {}{}", self.name, self.parameters))
    }
}

pub struct ErrorDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: Option<&'a ContractDefinition>,
    pub error_definition: &'a ErrorDefinition,
}
