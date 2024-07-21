mod standard;
mod traits;

pub use standard::*;
pub use traits::*;

use derive_more::From;

use crate::ast::{ASTNode, NodeID};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    // region: -- standard::* errors
    ForwardCallgraphNotAvailable,
    BackwardCallgraphNotAvailable,
    UnidentifiedEntryPointNode(ASTNode),
    InvalidEntryPointId(NodeID),
    EntryPointVisitError,
    UpstreamFunctionDefinitionVisitError,
    UpstreamModifierDefinitionVisitError,
    DownstreamFunctionDefinitionVisitError,
    DownstreamModifierDefinitionVisitError,
    UpstreamSideEffectFunctionDefinitionVisitError,
    UpstreamSideEffectModifierDefinitionVisitError,
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
