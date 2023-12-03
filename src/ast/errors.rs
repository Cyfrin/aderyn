use super::*;
use super::{node::*, *};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub src: String,
    pub id: NodeID,
}

impl BaseNode for ErrorDefinition {
    fn accept(&self, visitor: &mut impl AstBaseVisitor) -> Result<()> {
        if visitor.visit_error_definition(self)? && self.documentation.is_some() {
            self.documentation.as_ref().unwrap().accept(visitor)?;
            self.parameters.accept(visitor)?;
        }
        visitor.end_visit_error_definition(self)
    }
}

impl Display for ErrorDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("error {}{}", self.name, self.parameters))
    }
}

#[derive(Debug, PartialEq)]
pub struct ErrorDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: Option<&'a ContractDefinition>,
    pub error_definition: &'a ErrorDefinition,
}
