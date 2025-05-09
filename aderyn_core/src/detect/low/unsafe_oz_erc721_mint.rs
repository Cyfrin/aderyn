use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{
        browser::GetClosestAncestorOfTypeX,
        workspace::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC721MintDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnsafeERC721MintDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call identifier.accept(self)
            if let Some(ASTNode::SourceUnit(source_unit)) =
                identifier.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                let import_directives = source_unit.import_directives();
                if import_directives.iter().any(|directive| {
                    directive
                        .absolute_path
                        .as_ref()
                        .is_some_and(|path| path.contains("openzeppelin"))
                }) && identifier.name == "_mint"
                {
                    let this_contract_definition = identifier
                        .closest_ancestor_of_type(context, NodeType::ContractDefinition)
                        .unwrap();
                    if let ASTNode::ContractDefinition(contract_definition) =
                        this_contract_definition
                    {
                        for base_contract in contract_definition.base_contracts.iter() {
                            if let Some(base_name) = base_contract.base_name.name() {
                                if base_name.contains("ERC721") {
                                    capture!(self, context, identifier);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unsafe `ERC721::_mint()`")
    }

    fn description(&self) -> String {
        String::from(
            "Using `ERC721::_mint()` can mint ERC721 tokens to addresses which don't support ERC721 tokens. Use `_safeMint()` instead of `_mint()` for ERC721 tokens.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnsafeOzERC721Mint)
    }
}

#[cfg(test)]
mod unsafe_erc721_mint_tests {
    use crate::detect::{detector::IssueDetector, low::UnsafeERC721MintDetector};

    #[test]

    fn test_unsafe_erc721_mint_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnsafeERC721Mint.sol",
        );

        let mut detector = UnsafeERC721MintDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
