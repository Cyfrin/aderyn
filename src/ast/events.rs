use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub anonymous: bool,
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub src: String,
    pub id: NodeID,
}

impl Display for EventDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("event {}{}", self.name, self.parameters))
    }
}

pub struct EventDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub event_definition: &'a EventDefinition,
}
