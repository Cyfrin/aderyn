use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PragmaDirective {
    pub literals: Vec<String>,
    pub src: String,
    pub id: NodeID,
}

pub struct PragmaDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub pragma_directive: &'a PragmaDirective,
}
