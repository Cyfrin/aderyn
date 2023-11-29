use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub library_name: IdentifierPath,
    pub type_name: Option<TypeName>,
    pub src: String,
    pub id: NodeID,
}

impl Display for UsingForDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "using {} for {}",
            self.library_name,
            match self.type_name.as_ref() {
                Some(type_name) => format!("{}", type_name),
                None => "_".to_string(),
            }
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct UsingForDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub using_for_directive: &'a UsingForDirective,
}
