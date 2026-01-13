use serde::{Deserialize, Serialize};

use crate::ast::with_node_types;

pub type NodeID = i64;

macro_rules! define_node_types {
    (
        regular: $( $node:ident ),* $(,)*;
        yul: $( $yul_node:ident ),* $(,)*;
        yul_sourceless: $( $yul_sourceless_node:ident ),* $(,)*;
    ) => {
        define_node_types! {
            $( $node ),*,
            $( $yul_node ),*,
            $( $yul_sourceless_node ),*,
        }
    };
    (
        $( $node:ident ),* $(,)*
    ) => {

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub enum NodeType {
            $( $node ),*,
            SourceUnit,
            Throw, // suppport pre 0.5 solidity code
        }
    };
}

with_node_types!(define_node_types);
