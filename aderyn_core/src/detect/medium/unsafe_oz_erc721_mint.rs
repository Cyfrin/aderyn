use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{browser::GetParent, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC721MintDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for UnsafeERC721MintDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers.keys() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call identifier.accept(self)
            let source_unit = GetParent::source_unit_of(identifier, context).unwrap();

            let import_directives = source_unit.import_directives();
            if import_directives.iter().any(|directive| {
                directive
                    .absolute_path
                    .as_ref()
                    .map_or(false, |path| path.contains("openzeppelin"))
            }) && identifier.name == "_mint"
            {
                capture!(self, context, identifier);
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

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnsafeOzERC721Mint)
    }
}

#[cfg(test)]
mod unsafe_erc721_mint_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        medium::unsafe_oz_erc721_mint::UnsafeERC721MintDetector,
    };

    #[test]
    fn test_unsafe_erc721_mint_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/UnsafeERC721Mint.sol/UnsafeERC721Mint.json",
        );

        let mut detector = UnsafeERC721MintDetector::default();
        let found = detector.detect(&context).unwrap();
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
