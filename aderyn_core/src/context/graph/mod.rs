mod callgraph;
mod callgraph_tests;
mod serialize_callgraph;
mod traits;
mod workspace_callgraph;

pub use callgraph::*;
pub use traits::*;
pub use workspace_callgraph::*;

use derive_more::From;

use crate::ast::{ASTNode, NodeID};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    // region: -- standard::* errors
    WorkspaceCallGraphDFSError,
    InwardCallgraphNotAvailable,
    OutwardCallgraphNotAvailable,
    UnidentifiedEntryPointNode(ASTNode),
    InvalidEntryPointId(NodeID),
    EntryPointVisitError,
    OutwardFunctionDefinitionVisitError,
    OutwardModifierDefinitionVisitError,
    InwardFunctionDefinitionVisitError,
    InwardModifierDefinitionVisitError,
    OutwardSideEffectFunctionDefinitionVisitError,
    OutwardSideEffectModifierDefinitionVisitError,
    // endregion
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

impl std::error::Error for Error {}
