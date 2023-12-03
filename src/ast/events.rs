use super::*;
use super::{node::*, *};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
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

impl BaseNode for EventDefinition {
    fn accept(&self, visitor: &mut impl AstBaseVisitor) -> Result<()> {
        if visitor.visit_event_definition(self)? {
            if self.documentation.is_some() {
                self.documentation.as_ref().unwrap().accept(visitor)?;
            }
            self.parameters.accept(visitor)?;
        }
        visitor.end_visit_event_definition(self)
    }
}

impl Display for EventDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("event {}{}", self.name, self.parameters))
    }
}

#[derive(Debug, PartialEq)]
pub struct EventDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub event_definition: &'a EventDefinition,
}
