use super::{macros::ast_node, Statement};
use serde::{Deserialize, Serialize};

ast_node!(
    struct Block {
        statements: Vec<Statement>,
    }
);
