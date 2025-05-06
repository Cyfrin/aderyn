use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::{capture, detect::detector::IssueDetectorNamePool};
use phf::phf_set;

use crate::{
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct BuiltinSymbolShadowingDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for BuiltinSymbolShadowingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Variable Declaration names
        for variable_declaration in context.variable_declarations() {
            if DENY_LIST.contains(&variable_declaration.name) {
                capture!(self, context, variable_declaration);
            }
        }

        // Function Definition names
        for function in context.function_definitions() {
            if DENY_LIST.contains(&function.name) {
                capture!(self, context, function);
            }
        }

        // Modifier definition names
        for modifier in context.modifier_definitions() {
            if DENY_LIST.contains(&modifier.name) {
                capture!(self, context, modifier);
            }
        }

        // Event definition names
        for event in context.event_definitions() {
            if DENY_LIST.contains(&event.name) {
                capture!(self, context, event);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Builtin Symbol Shadowing")
    }

    fn description(&self) -> String {
        String::from("Name clashes with a built-in-symbol. Consider renaming it.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::BuiltinSymbolShadowing)
    }
}

// Copied from SLITHER
static DENY_LIST: phf::Set<&'static str> = phf_set! {
    "assert",
    "require",
    "revert",
    "block",
    "blockhash",
    "gasleft",
    "msg",
    "now",
    "tx",
    "this",
    "addmod",
    "mulmod",
    "keccak256",
    "sha256",
    "sha3",
    "ripemd160",
    "ecrecover",
    "selfdestruct",
    "suicide",
    "abi",
    "fallback",
    "receive",
    "abstract",
    "after",
    "alias",
    "apply",
    "auto",
    "case",
    "catch",
    "copyof",
    "default",
    "define",
    "final",
    "immutable",
    "implements",
    "in",
    "inline",
    "let",
    "macro",
    "match",
    "mutable",
    "null",
    "of",
    "override",
    "partial",
    "promise",
    "reference",
    "relocatable",
    "sealed",
    "sizeof",
    "static",
    "supports",
    "switch",
    "try",
    "type",
    "typedef",
    "typeof",
    "unchecked",
};

#[cfg(test)]
mod builtin_symbol_shadowing_tests {

    use crate::detect::{
        detector::IssueDetector, low::builtin_symbol_shadowing::BuiltinSymbolShadowingDetector,
    };

    #[test]

    fn test_builtin_symbol_shadow() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/BuiltinSymbolShadow.sol",
        );

        let mut detector = BuiltinSymbolShadowingDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 4);
    }
}
