use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(untagged)]
pub enum Documentation {
    String(Option<String>),
    Structured(Option<StructuredDocumentation>),
}

impl Node for Documentation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            Documentation::String(_opt_string) => {
                // TODO check if this is okay
                Ok(())
            }
            Documentation::Structured(opt_structured_documentation) => {
                if opt_structured_documentation.is_some() {
                    opt_structured_documentation
                        .as_ref()
                        .unwrap()
                        .accept(visitor)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub text: String,
    pub src: String,
    pub id: NodeID,
}

impl Node for StructuredDocumentation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_structured_documentation(self)?;
        visitor.end_visit_structured_documentation(self)
    }
}
