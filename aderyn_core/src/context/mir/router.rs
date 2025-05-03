//! Router
//!
//! Function router.
//!
//! TODO
//! * Help resolve internal calls to corresponding function definitions
//! * Help guess external calls to corresponding function definitions

use std::collections::HashMap;

use crate::ast::*;

pub struct Router {
    /// Key - ASTNode Id of [`ContractDefinition`]
    pub contracts: HashMap<NodeID, RContract>,
}

pub struct RContract {
    pub entrypoint: Vec<Entrypoint>,
}

pub struct Entrypoint {
    pub read_fn: ReadFn,
    pub write_fn: WriteFn,
}

#[cfg(test)]
mod function_router {

    #[test]
    pub fn same_contract_internal_call() {}
}
