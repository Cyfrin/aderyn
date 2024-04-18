use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;

use crate::ast::{ImportDirective, NodeID, SourceUnit, TypeName};

use crate::capture;
use crate::context::browser::{
    ExtractIdentifiers, ExtractImportDirectives, ExtractInheritanceSpecifiers,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Default)]
pub struct UnusedImportsDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

// Standalone helper function
fn process_imports<'a>(
    distant_relatives: &'a mut Vec<NodeID>,
    import: &'a ImportDirective,
    context: &'a WorkspaceContext,
) {
    distant_relatives.push(import.source_unit);
    let import_source_unit = context
        .source_units()
        .into_iter()
        .find(|source_unit| source_unit.id == import.source_unit)
        .unwrap();
    let next_level_imports = ExtractImportDirectives::from(import_source_unit).extracted;
    next_level_imports.iter().for_each(|next_level_import| {
        process_imports(distant_relatives, next_level_import, context);
    });
}

impl IssueDetector for UnusedImportsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // capture!(self, context, item);
        context.source_units().into_iter().for_each(|source_unit| {
            // Load distant relatives map
            let mut distant_relative_source_unit_to_original_import: HashMap<
                NodeID,
                &ImportDirective,
            > = HashMap::new();
            let imports = ExtractImportDirectives::from(source_unit).extracted;
            if imports.is_empty() {
                return;
            }

            for import in &imports {
                let mut distant_relatives: Vec<NodeID> = Vec::new();
                process_imports(&mut distant_relatives, import, context);
                distant_relatives.into_iter().for_each(|distant_relative| {
                    distant_relative_source_unit_to_original_import
                        .insert(distant_relative, import);
                })
            }

            let mut imports_used: HashSet<&ImportDirective> = HashSet::new();
            // Do this with identifiers
            let identifiers = ExtractIdentifiers::from(source_unit).extracted;
            for identifier in &identifiers {
                let declaring_source_unit =
                    context.get_source_unit_from_child_node_id(identifier.referenced_declaration);

                match declaring_source_unit {
                    Some(declaring_source_unit) => {
                        let used_import = distant_relative_source_unit_to_original_import
                            .get(&declaring_source_unit.id);

                        match used_import {
                            Some(import) => {
                                imports_used.insert(import);
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            // Do the same with InheritanceSpecifier
            let inheritance_specifiers = ExtractInheritanceSpecifiers::from(source_unit).extracted;
            for inheritance_specifier in &inheritance_specifiers {
                let declaring_source_unit = context.get_source_unit_from_child_node_id(
                    inheritance_specifier
                        .base_name
                        .referenced_declaration
                        .unwrap(),
                );

                match declaring_source_unit {
                    Some(declaring_source_unit) => {
                        let used_import = distant_relative_source_unit_to_original_import
                            .get(&declaring_source_unit.id);

                        match used_import {
                            Some(import) => {
                                imports_used.insert(import);
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }

            for import in &imports {
                if !imports_used.contains(import) {
                    capture!(self, context, import);
                }
            }
        });

        println!("found_instances: {:#?}", self.found_instances);
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Unused Import")
    }

    fn description(&self) -> String {
        String::from("This import is not used, consider removing it.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("unused-imports")
    }
}

#[cfg(test)]
mod unused_imports_detector_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_multiple_contracts, IssueDetector},
        low::UnusedImportsDetector,
    };

    #[test]
    fn test_unused_imports_detector() {
        let context = load_multiple_contracts(vec![
            "../tests/contract-playground/out/FourthLevel.sol/FourthLevel.json",
            "../tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
            "../tests/contract-playground/out/InheritanceBase.sol/InheritanceBase.0.8.20.json",
            "../tests/contract-playground/out/IContractInheritance.sol/IContractInheritance.0.8.20.json",
            "../tests/contract-playground/out/EnumerableSet.sol/EnumerableSet.json",
        ]);

        let mut detector = UnusedImportsDetector::default();
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
        // assert the title is correct
        assert_eq!(detector.title(), String::from("Low Issue Title"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Description of the low issue.")
        );
    }
}
