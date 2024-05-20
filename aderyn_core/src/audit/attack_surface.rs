use prettytable::{row, Row};

use super::auditor::AuditorDetector;
use crate::{
    ast::{Expression, FunctionCallKind, MemberAccess, NodeID, NodeType, TypeName},
    context::{
        browser::{GetClosestAncestorOfTypeX, Peek},
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::helpers::get_calls_and_delegate_calls,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::{self, Display},
};

pub enum AddressSource {
    Storage,
    Havoc,
}

#[derive(Clone, Eq, PartialEq)]
pub struct AttackSurfaceInstance {
    pub contract_name: String,
    pub function_name: String,
    pub source_code: String,
    pub address_source: String,
}

use std::cmp::{Ord, Ordering, PartialOrd};

impl Ord for AttackSurfaceInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_contract = self.contract_name.cmp(&other.contract_name);
        if by_contract != Ordering::Equal {
            return by_contract;
        }

        let by_function = self.function_name.cmp(&other.function_name);
        if by_function != Ordering::Equal {
            return by_function;
        }

        let by_source = self.source_code.cmp(&other.source_code);
        if by_source != Ordering::Equal {
            return by_source;
        }

        self.address_source.cmp(&other.address_source)
    }
}

impl PartialOrd for AttackSurfaceInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for AddressSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Havoc => "Havoc",
            Self::Storage => "Storage",
        };
        write!(f, "{}", s)
    }
}

#[derive(Default)]
pub struct AttackSurfaceDetector {
    found_instances: BTreeSet<AttackSurfaceInstance>,
}

impl AuditorDetector for AttackSurfaceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut surface_points: BTreeMap<NodeID, AddressSource> = BTreeMap::new();

        for member_access in get_calls_and_delegate_calls(context) {
            let address_source = find_address_source_if_direct_call(context, member_access)
                .or_else(|| find_address_source_if_function_call(context, member_access))
                .unwrap_or(AddressSource::Havoc);

            surface_points.insert(member_access.id, address_source);
        }

        self.found_instances = transform_surface_points(context, &surface_points);

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("External Contract `call` and `delegatecall` Instances")
    }

    fn table_titles(&self) -> Row {
        row!["Contract", "Function", "Code", "Address Source"]
    }

    fn table_rows(&self) -> Vec<Row> {
        self.found_instances
            .iter()
            .map(|instance| {
                row![
                    instance.contract_name,
                    instance.function_name,
                    instance.source_code,
                    instance.address_source
                ]
            })
            .collect()
    }

    fn skeletal_clone(&self) -> Box<dyn AuditorDetector> {
        Box::<AttackSurfaceDetector>::default()
    }
}

fn transform_surface_points(
    context: &WorkspaceContext,
    surface_points: &BTreeMap<NodeID, AddressSource>,
) -> BTreeSet<AttackSurfaceInstance> {
    let mut auditor_instances: BTreeSet<AttackSurfaceInstance> = BTreeSet::new();

    for (id, address_storage) in surface_points {
        if let Some(ast_node) = context.nodes.get(id) {
            let contract = ast_node.closest_ancestor_of_type(context, NodeType::ContractDefinition);
            let function = ast_node.closest_ancestor_of_type(context, NodeType::FunctionDefinition);
            if let Some(ASTNode::ContractDefinition(contract)) = contract {
                if let Some(ASTNode::FunctionDefinition(function)) = function {
                    if let Some(source_code) = ast_node.peek(context) {
                        let contract_name = contract.name.to_string();
                        let function_name = function.name.to_string();
                        auditor_instances.insert(AttackSurfaceInstance {
                            contract_name,
                            function_name,
                            source_code,
                            address_source: address_storage.to_string(),
                        });
                    }
                }
            }
        }
    }

    auditor_instances
}

fn find_address_source_if_direct_call(
    context: &WorkspaceContext,
    member_access: &MemberAccess,
) -> Option<AddressSource> {
    if let Expression::Identifier(identifier) = &*member_access.expression {
        let referenced_declaration = context.nodes.get(&identifier.referenced_declaration);
        if let Some(ASTNode::VariableDeclaration(variable_declaration)) = referenced_declaration {
            if variable_declaration.state_variable {
                return Some(AddressSource::Storage);
            }
        }
        return Some(AddressSource::Havoc);
    }
    None
}

fn find_address_source_if_function_call(
    context: &WorkspaceContext,
    member_access: &MemberAccess,
) -> Option<AddressSource> {
    if let Expression::FunctionCall(function_call) = &*member_access.expression {
        if function_call.kind == FunctionCallKind::TypeConversion {
            if let Expression::ElementaryTypeNameExpression(elementary_type_name_expression) =
                &*function_call.expression
            {
                if let TypeName::ElementaryTypeName(elementary_type_name) =
                    &elementary_type_name_expression.type_name
                {
                    if elementary_type_name.name == "address" {
                        if let Expression::Identifier(identifier) = &function_call.arguments[0] {
                            let referenced_declaration =
                                context.nodes.get(&identifier.referenced_declaration);
                            if let Some(ASTNode::VariableDeclaration(variable_declaration)) =
                                referenced_declaration
                            {
                                if variable_declaration.state_variable {
                                    return Some(AddressSource::Storage);
                                }
                            }
                        }
                    }
                }
            }
        }
        return Some(AddressSource::Havoc);
    }
    None
}

#[cfg(test)]
mod attack_surface_detector_tests {
    use crate::audit::{attack_surface::AttackSurfaceDetector, auditor::AuditorDetector};

    #[test]
    fn test_attack_surface_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/auditor_mode/ExternalCalls.sol",
        );

        let mut detector = AttackSurfaceDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        assert!(detector.found_instances.len() == 8);
    }
}
