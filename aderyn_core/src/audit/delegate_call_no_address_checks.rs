use prettytable::{row, Row};

use super::{auditor::AuditorDetector, investigators::SimpleInvestigator};
use crate::{
    context::workspace_context::WorkspaceContext, detect::helpers,
    visitor::ast_visitor::ASTConstVisitor,
};
use std::error::Error;

#[derive(Debug)]
struct DelegateCallNoChecksInstance {
    pub filename: String,
    pub line_no: usize,
    pub func_name: String,
}

impl DelegateCallNoChecksInstance {
    fn encode_from(func_name: String, node_key: (String, usize, String)) -> Self {
        let (filename, line_no, _) = node_key;
        Self {
            filename,
            line_no,
            func_name,
        }
    }
}

#[derive(Default)]
pub struct DelegateCallNoChecksDetector {
    found_instances: Vec<DelegateCallNoChecksInstance>,
}

impl AuditorDetector for DelegateCallNoChecksDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for func in helpers::get_implemented_external_and_public_functions(context) {
            let mut tracker = DelegateCallNoAddressChecksTracker {
                has_address_checks: false,
                has_delegate_call_on_non_state_variable_address: false,
                context,
            };
            let investigator: SimpleInvestigator = SimpleInvestigator::for_node(func, context)?;
            investigator.investigate(context, &mut tracker)?;

            if tracker.has_delegate_call_on_non_state_variable_address
                && !tracker.has_address_checks
            {
                self.found_instances
                    .push(DelegateCallNoChecksInstance::encode_from(
                        func.name.to_owned(),
                        context.get_node_sort_key_pure(&func.into()),
                    ));
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Delegate calls sent to an unprotected address.")
    }

    fn table_titles(&self) -> Row {
        row!["Filename", "Line No", "Function"]
    }

    fn table_rows(&self) -> Vec<Row> {
        self.found_instances
            .iter()
            .map(|instance| row![instance.filename, instance.line_no, instance.func_name])
            .collect()
    }

    fn skeletal_clone(&self) -> Box<dyn AuditorDetector> {
        Box::<DelegateCallNoChecksDetector>::default()
    }
}

pub struct DelegateCallNoAddressChecksTracker<'a> {
    pub has_address_checks: bool,
    pub has_delegate_call_on_non_state_variable_address: bool,
    context: &'a WorkspaceContext,
}

impl<'a> ASTConstVisitor for DelegateCallNoAddressChecksTracker<'a> {
    fn visit_modifier_definition(
        &mut self,
        node: &crate::ast::ModifierDefinition,
    ) -> eyre::Result<bool> {
        if !self.has_address_checks && helpers::has_binary_checks_on_some_address(&node.into()) {
            self.has_address_checks = true;
        }
        if !self.has_delegate_call_on_non_state_variable_address
            && helpers::has_delegate_calls_on_non_state_variables(&node.into(), self.context)
        {
            self.has_delegate_call_on_non_state_variable_address = true;
        }
        eyre::Ok(true)
    }
    fn visit_function_definition(
        &mut self,
        node: &crate::ast::FunctionDefinition,
    ) -> eyre::Result<bool> {
        if !self.has_address_checks && helpers::has_binary_checks_on_some_address(&node.into()) {
            self.has_address_checks = true;
        }
        if !self.has_delegate_call_on_non_state_variable_address
            && helpers::has_delegate_calls_on_non_state_variables(&node.into(), self.context)
        {
            self.has_delegate_call_on_non_state_variable_address = true;
        }
        eyre::Ok(true)
    }
}

#[cfg(test)]
mod delegate_call_no_address_checks {
    use crate::audit::{
        auditor::AuditorDetector, delegate_call_no_address_checks::DelegateCallNoChecksDetector,
    };

    #[test]
    fn test_delegate_call_no_address_checks() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/auditor_mode/delegate_call_no_checks/DelegateCallWithoutAddressChecks.sol",
        );

        let mut detector = DelegateCallNoChecksDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:#?}", detector.found_instances);

        assert!(found);
        assert!(detector.found_instances.len() == 1);
    }
}
