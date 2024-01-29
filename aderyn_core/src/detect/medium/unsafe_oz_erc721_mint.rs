use std::{collections::BTreeMap, error::Error};

use crate::{
    capture,
    context::{
        browser::GetParent,
        loader::{ASTNode, ContextLoader},
    },
    detect::detector::{Detector, DetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC721MintDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), i64>,
}

impl Detector for UnsafeERC721MintDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for identifier in loader.identifiers.keys() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call identifier.accept(self)
            let source_unit = GetParent::source_unit_of(identifier, loader).unwrap();

            let import_directives = source_unit.import_directives();
            if import_directives.iter().any(|directive| {
                directive
                    .absolute_path
                    .as_ref()
                    .map_or(false, |path| path.contains("openzeppelin"))
            }) && identifier.name == "_mint"
            {
                capture!(self, loader, identifier);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Using `ERC721::_mint()` can be dangerous")
    }

    fn description(&self) -> String {
        String::from(
            "Using `ERC721::_mint()` can mint ERC721 tokens to addresses which don't support ERC721 tokens. Use `_safeMint()` instead of `_mint()` for ERC721.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn instances(&self) -> BTreeMap<(String, usize), i64> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", DetectorNamePool::UnsafeOzERC721Mint)
    }
}

#[cfg(test)]
mod unsafe_erc721_mint_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, Detector},
        medium::unsafe_oz_erc721_mint::UnsafeERC721MintDetector,
    };

    #[test]
    fn test_unsafe_erc721_mint_detector() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/UnsafeERC721Mint.sol/UnsafeERC721Mint.json",
        );

        let mut detector = UnsafeERC721MintDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct number of instance
        assert_eq!(detector.instances().len(), 1);
        // assert that the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Medium
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Using `ERC721::_mint()` can be dangerous")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Using `ERC721::_mint()` can mint ERC721 tokens to addresses which don't support ERC721 tokens. Use `_safeMint()` instead of `_mint()` for ERC721."
            )
        );
    }
}
