use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Expression,
    pub local: Option<String>,
    pub name_location: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportDirective {
    pub file: String,
    pub source_unit: NodeID,
    pub scope: NodeID,
    pub absolute_path: Option<String>,
    pub unit_alias: String,
    pub name_location: Option<String>,
    pub symbol_aliases: Vec<SymbolAlias>,
    pub src: String,
    pub id: NodeID,
}

pub struct ImportDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub import_directive: &'a ImportDirective,
}
