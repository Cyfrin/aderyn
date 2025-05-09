mod callgraph;
mod preprocess;
mod traits;
mod utils;

use std::collections::HashMap;

pub use callgraph::*;
pub use traits::*;

use derive_more::From;

use crate::ast::{ASTNode, NodeID};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct LegacyWorkspaceCallGraph {
    pub raw_callgraph: RawCallGraph,
}

#[derive(Debug, Default)]
pub struct WorkspaceCallGraphs {
    // Key => Contract Definition NodeID
    pub inward_callgraphs: HashMap<NodeID, RawCallGraph>,
    pub outward_callgraphs: HashMap<NodeID, RawCallGraph>,
}

/**
 * Every NodeID in RawCallGraph should corresponds to [`crate::ast::FunctionDefinition`] or
 * [`crate::ast::ModifierDefinition`]
 */
pub type RawCallGraph = HashMap<NodeID, Vec<NodeID>>;

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
