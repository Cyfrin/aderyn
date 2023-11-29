use super::{node::*, *};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Documentation {
    String(Option<String>),
    Structured(Option<StructuredDocumentation>),
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub text: String,
    pub src: String,
    pub id: NodeID,
}
