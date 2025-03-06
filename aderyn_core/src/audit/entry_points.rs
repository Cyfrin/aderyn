use prettytable::{row, Row};

use super::auditor::AuditorDetector;
use crate::{
    ast::{FunctionKind, NodeType},
    audit::auditor::AuditorDetectorNamePool,
    context::workspace_context::{ASTNode, WorkspaceContext},
    detect::helpers::get_implemented_external_and_public_functions,
};
use std::{cmp::Ordering, collections::BTreeSet, error::Error};

#[derive(Clone, Eq, PartialEq)]
pub struct EntryPointsInstance {
    pub contract_name: String,
    pub function_name: String,
    pub function_kind: FunctionKind,
}

impl Ord for EntryPointsInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_contract = self.contract_name.cmp(&other.contract_name);
        if by_contract == Ordering::Equal {
            self.function_name.cmp(&other.function_name)
        } else {
            by_contract
        }
    }
}

impl PartialOrd for EntryPointsInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct EntryPointsDetector {
    // contract_name, function_name
    found_instances: BTreeSet<EntryPointsInstance>,
}

impl AuditorDetector for EntryPointsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let functions = get_implemented_external_and_public_functions(context);
        functions.for_each(|function_definition| {
            if let ASTNode::ContractDefinition(contract_definition) = context
                .get_closest_ancestor(function_definition.id, NodeType::ContractDefinition)
                .unwrap()
            {
                let contract_name = contract_definition.name.clone();
                self.found_instances.insert(EntryPointsInstance {
                    contract_name,
                    function_name: function_definition.name.clone(),
                    function_kind: function_definition.kind().clone(),
                });
            }
        });
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Contract Entry Points")
    }

    fn name(&self) -> String {
        format!("{}", AuditorDetectorNamePool::EntryPoints)
    }

    fn table_titles(&self) -> Row {
        row!["Contract", "Function Kind", "Function Name"]
    }

    fn table_rows(&self) -> Vec<Row> {
        self.found_instances
            .iter()
            .map(|instance| {
                row![instance.contract_name, instance.function_kind, instance.function_name,]
            })
            .collect()
    }
}

#[cfg(test)]
mod entry_points_test {
    use serial_test::serial;

    use crate::{
        audit::{auditor::AuditorDetector, entry_points::EntryPointsDetector},
        detect::test_utils::load_solidity_source_unit,
    };

    #[test]
    #[serial]
    fn test_entry_points() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/auditor_mode/PublicFunctionsWithoutSenderCheck.sol",
        );

        let mut detector = EntryPointsDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        assert!(detector.found_instances.len() == 11);
    }
}
