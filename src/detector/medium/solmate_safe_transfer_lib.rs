use std::error::Error;

use crate::ast::ImportDirective;
use crate::visitor::ast_visitor::Node;
use crate::{
    ast::MemberAccess,
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::ASTConstVisitor,
};
use eyre::Result;

#[derive(Default)]
pub struct SolmateSafeTransferLibDetector {
    found_solmate_import: bool,
    found_transfer_usage: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for SolmateSafeTransferLibDetector {
    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        if !self.found_solmate_import {
            // If the import directive absolute_path contains the strings "solmate" and "SafeTransferLib", flip the found_solmate_import flag to true
            if node.absolute_path.as_ref().unwrap().contains("solmate")
                && node
                    .absolute_path
                    .as_ref()
                    .unwrap()
                    .contains("SafeTransferLib")
            {
                self.found_solmate_import = true;
            }
        }
        Ok(true)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        // If the member access member_name is any of the following names, add it to the list of found
        // found_transfer_usage vector: ["safeTransfer", "safeTransferFrom", "safeApprove"]
        if node.member_name == "safeTransfer"
            || node.member_name == "safeTransferFrom"
            || node.member_name == "safeApprove"
        {
            self.found_transfer_usage
                .push(Some(ASTNode::MemberAccess(node.clone())));
        }

        Ok(true)
    }
}

impl Detector for SolmateSafeTransferLibDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for import_directive in loader.get_import_directives() {
            import_directive.accept(self)?;
        }

        for member_access in loader.get_member_accesses() {
            member_access.accept(self)?;
        }

        if self.found_solmate_import && !self.found_transfer_usage.is_empty() {
            return Ok(true);
        }

        Ok(false)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Solmate\'s SafeTransferLib does not check for token contract\'s existence")
    }

    fn description(&self) -> String {
        String::from("There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.\nhttps://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 \n`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`\n")
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_transfer_usage.clone()
    }
}

#[cfg(test)]
mod solmate_safe_transfer_lib_tests {
    use crate::detector::{
        detector::{detector_test_helpers::load_contract, Detector},
        medium::solmate_safe_transfer_lib::SolmateSafeTransferLibDetector,
    };

    #[test]
    fn test_solmate_safe_transfer_lib() {
        let context_loader =
            load_contract("./tests/contract-playground/out/T11sTranferer.sol/T11sTranferer.json");
        let mut detector = SolmateSafeTransferLibDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found a delegate call in a loop
        assert!(found);
        // assert that the detector found the correct number of instances (1)
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detector::detector::IssueSeverity::Medium
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from(
                "Solmate\'s SafeTransferLib does not check for token contract\'s existence"
            )
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.\nhttps://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 \n`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`\n"
            )
        );
    }
}
