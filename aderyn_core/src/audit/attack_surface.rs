use prettytable::{format, row, Table};

use super::auditor::AuditorDetector;
use crate::{
    ast::{Expression, FunctionCallKind, NodeID, TypeName},
    context::{
        browser::Peek,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::helpers::get_calls_and_delegate_calls,
};
use std::{collections::BTreeMap, error::Error};

enum AddressSource {
    Storage,
    Havoc,
}

pub struct AttackSurfaceContext {
    id: NodeID,
    address_source: AddressSource,
}

#[derive(Default)]
pub struct AttackSurfaceDetector {
    found_instances: BTreeMap<(String, usize, String), AttackSurfaceContext>,
}

impl AuditorDetector for AttackSurfaceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in get_calls_and_delegate_calls(context) {
            let mut attack_surface_context = AttackSurfaceContext {
                id: member_access.id,
                address_source: AddressSource::Havoc,
            };

            // someAddress.call();
            if let Expression::Identifier(identifier) = &*member_access.expression {
                let referenced_declaration = context.nodes.get(&identifier.referenced_declaration);
                if let Some(ASTNode::VariableDeclaration(variable_declaration)) =
                    referenced_declaration
                {
                    if variable_declaration.state_variable {
                        attack_surface_context.address_source = AddressSource::Storage;
                    }
                }
            }
            // address(someContract).call();
            if let Expression::FunctionCall(function_call) = &*member_access.expression {
                if function_call.kind == FunctionCallKind::TypeConversion {
                    if let Expression::ElementaryTypeNameExpression(
                        elementary_type_name_expression,
                    ) = &*function_call.expression
                    {
                        if let TypeName::ElementaryTypeName(elementary_type_name) =
                            &elementary_type_name_expression.type_name
                        {
                            if elementary_type_name.name == "address" {
                                if let Expression::Identifier(identifier) =
                                    &function_call.arguments[0]
                                {
                                    let referenced_declaration =
                                        context.nodes.get(&identifier.referenced_declaration);
                                    if let Some(ASTNode::VariableDeclaration(
                                        variable_declaration,
                                    )) = referenced_declaration
                                    {
                                        if variable_declaration.state_variable {
                                            attack_surface_context.address_source =
                                                AddressSource::Storage;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self.found_instances.insert(
                context.get_node_sort_key(&member_access.into()),
                attack_surface_context,
            );
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Attack Surface - External Contract `call` and `delegatecall` Instances")
    }

    fn print(&self, context: &WorkspaceContext) {
        let mut table = Table::new();

        println!();
        println!("{}:", self.title());
        table.set_titles(row!["Contract", "Function", "Code", "Address Source"]);

        for attack_surface_context in self.found_instances.values() {
            let ast_node = context.nodes.get(&attack_surface_context.id).unwrap();
            if let ASTNode::MemberAccess(member_access) = ast_node {
                let member_access_context =
                    context.member_accesses_context.get(member_access).unwrap();
                let contract_ast_node = context
                    .nodes
                    .get(&member_access_context.contract_definition_id.unwrap())
                    .unwrap();
                if let ASTNode::ContractDefinition(contract_definition) = contract_ast_node {
                    let function_ast_node = context
                        .nodes
                        .get(&member_access_context.function_definition_id.unwrap())
                        .unwrap();
                    if let ASTNode::FunctionDefinition(function_definition) = function_ast_node {
                        table.add_row(row![
                            contract_definition.name,
                            function_definition.name,
                            member_access.peek(context).unwrap(),
                            match attack_surface_context.address_source {
                                AddressSource::Storage => "Storage",
                                AddressSource::Havoc => "Havoc",
                            }
                        ]);
                    }
                }
            }
        }

        // Set the format of the table
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    }
}

#[cfg(test)]
mod attack_surface_detector_tests {
    use crate::{
        audit::{attack_surface::AttackSurfaceDetector, auditor::AuditorDetector},
        detect::detector::detector_test_helpers::load_contract,
    };

    #[test]
    fn test_attack_surface_detector() {
        let context =
            load_contract("../tests/contract-playground/out/ExternalCalls.sol/ExternalCalls.json");

        let mut detector = AttackSurfaceDetector::default();
        let found = detector.detect(&context).unwrap();
        detector.print(&context);
        // assert that the detector found an issue
        assert!(found);
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Attack Surface - External Contract Calls")
        );
    }
}
