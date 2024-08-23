use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::{ASTNode, NodeID};

use crate::capture;
use crate::context::browser::ExtractReferencedDeclarations;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnusedImportDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnusedImportDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // For each source unit `s` in context:
        //    Let R = Set of all referencedDeclarations in `s`
        //    For each source unit ID `i` that `s` imports:
        //         Let e = Set of all exportedSymbols from `i`
        //         If (R ∩ e = Φ) then:
        //             Declare `i` as an unused import

        for source_unit in context.source_units() {
            let mut all_references_set = HashSet::new();

            let all_references = ExtractReferencedDeclarations::from(source_unit).extracted;
            for reference in all_references {
                all_references_set.insert(reference);
            }

            for imported_source_unit in source_unit.import_directives() {
                let imported_source_unit_id = imported_source_unit.source_unit;
                if let Some(ASTNode::SourceUnit(i)) = context.nodes.get(&imported_source_unit_id) {
                    if let Some(exported_symbols) = i.exported_symbols.as_ref() {
                        if exported_symbols
                            .values()
                            .flatten()
                            .all(|symbol| !all_references_set.contains(symbol))
                        {
                            capture!(self, context, imported_source_unit);
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Unused Imports")
    }

    fn description(&self) -> String {
        String::from("Redundant import statement. Consider removing it.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnusedImport)
    }
}

#[cfg(test)]
mod unused_imports_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, low::unused_imports::UnusedImportDetector};

    #[test]
    #[serial]
    fn test_unused_imports() {
        let context =
            crate::detect::test_utils::load_multiple_solidity_source_units_into_single_context(&[
                "../tests/contract-playground/src/UnusedImport.sol",
                "../tests/contract-playground/src/U2.sol",
            ]);

        let mut detector = UnusedImportDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
