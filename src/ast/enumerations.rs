use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub name_location: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Display for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub members: Vec<EnumValue>,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Display for EnumDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("enum {} {{\n", self.name))?;

        for member in self.members.iter() {
            f.write_fmt(format_args!("\t{},", member))?;
        }

        f.write_str("}")
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: Option<&'a ContractDefinition>,
    pub enum_definition: &'a EnumDefinition,
}
