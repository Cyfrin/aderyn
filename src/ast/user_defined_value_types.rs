use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub underlying_type: TypeName,
    pub name: String,
    pub name_location: Option<String>,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Display for UserDefinedValueTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "type {} is {}",
            self.name, self.underlying_type,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct UserDefinedValueTypeDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: Option<&'a ContractDefinition>,
    pub user_defined_value_type_definition: &'a UserDefinedValueTypeDefinition,
}
