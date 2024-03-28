use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{Expression, NodeID},
    capture,
    context::{browser::ExtractFunctionCalls, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct Create2InsteadOfCreateDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for Create2InsteadOfCreateDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for function in context.function_definitions() {
            // println!("\n\nFunction: {}\n", function.name);

            for call in ExtractFunctionCalls::from(function).extracted.iter() {
                match *call.expression.clone() {
                    Expression::FunctionCallOptions(function_call_options) => {
                        if let Some(type_string) =
                            function_call_options.type_descriptions.type_string.clone()
                        {
                            if type_string.contains("contract")
                                && !function_call_options.names.contains(&"salt".to_owned())
                            {
                                capture!(self, context, call);
                            }
                        }
                    }
                    Expression::ElementaryTypeNameExpression(elementary_type_name_expr) => {
                        if let Some(type_string) =
                            elementary_type_name_expr.type_descriptions.type_string
                        {
                            if type_string.contains("contract") {
                                capture!(self, context, call);
                            }
                        }
                    }
                    _ => (),
                };
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("create2 opcode should be used instead of create")
    }

    fn description(&self) -> String {
        String::from("create2 makes deployment addresses consistent and is less susceptible to chain reorg attacks especially in L2 scaling solutions")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::Create2InsteadOfCreate)
    }
}

#[cfg(test)]
mod create2_instead_of_create_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::Create2InsteadOfCreateDetector;

    #[test]
    fn test_create2_instead_of_create() {
        let context = load_contract("../tests/contract-playground/out/Create.sol/CarFactory.json");

        let mut detector = Create2InsteadOfCreateDetector::default();
        let found = detector.detect(&context).unwrap();

        // assert that the detector found an abi encode packed
        assert!(found);

        println!("{:?}", detector.instances());

        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 3);
    }
}
