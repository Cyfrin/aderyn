use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;

use crate::ast::{NodeID, VariableDeclaration};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ConstantDefinedMultipleTimesDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ConstantDefinedMultipleTimesDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // capture!(self, context, item);

        let mut vars_by_name: HashMap<String, Vec<VariableDeclaration>> = HashMap::new();

        for var_decl in context
            .variable_declarations()
            .into_iter()
            .filter(|v| v.state_variable && v.constant)
        {
            match vars_by_name.entry(var_decl.name.clone()) {
                Entry::Occupied(mut o) => {
                    o.get_mut().push(var_decl.to_owned());
                }
                Entry::Vacant(e) => {
                    e.insert(vec![var_decl.to_owned()]);
                }
            }
        }

        for (_, const_potentially_defined_multiple_times) in vars_by_name.into_iter() {
            let mut locations: HashSet<_> = HashSet::new();
            for c in &const_potentially_defined_multiple_times {
                locations.insert(context.get_node_sort_key(&c.into()));
            }
            if locations.len() > 1 {
                for c in &const_potentially_defined_multiple_times {
                    capture!(self, context, c);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Constants defined multiple times")
    }

    fn description(&self) -> String {
        String::from("Constants defined multiple times is not ideal. Consider renaming or re-use the same constant")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ConstantDefinedMultipleTimes)
    }
}

#[cfg(test)]
mod constants_with_multiple_definitions {
    use crate::detect::{
        detector::{detector_test_helpers::load_multiple_contracts, IssueDetector},
        low::const_multiple_definitions::ConstantDefinedMultipleTimesDetector,
    };

    #[test]
    fn test_constants_with_multiple_definitions() {
        let context = load_multiple_contracts(vec![
            "../tests/contract-playground/out/C1.sol/C1.json",
            "../tests/contract-playground/out/C2.sol/C2.json",
        ]);

        let mut detector = ConstantDefinedMultipleTimesDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Constants defined multiple times")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Constants defined multiple times is not ideal. Consider renaming or re-use the same constant")
        );
    }
}
