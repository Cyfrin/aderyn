use semver::{Error, VersionReq};

use crate::{
    ast::{
        ASTNode, Expression, FunctionDefinition, MemberAccess, NodeID, PragmaDirective,
        VariableDeclaration, Visibility,
    },
    context::{
        browser::{ExtractBinaryOperations, ExtractMemberAccesses},
        workspace_context::WorkspaceContext,
    },
};

/// Count the number of identifiers that reference a given ID in the context.
pub fn count_identifiers_that_reference_an_id(
    context: &WorkspaceContext,
    function_id: NodeID,
) -> i32 {
    let mut count = 0;
    context.identifiers().iter().for_each(|&identifier| {
        if let Some(reference_id) = identifier.referenced_declaration {
            if reference_id == function_id {
                count += 1;
            }
        }
    });
    count
}

pub fn get_calls_and_delegate_calls(context: &WorkspaceContext) -> Vec<&MemberAccess> {
    context
        .member_accesses()
        .into_iter()
        .filter(|member_access| {
            member_access.member_name == "call" || member_access.member_name == "delegatecall"
        })
        .collect()
}

// Get all implemented external and public functions in the context.
pub fn get_implemented_external_and_public_functions(
    context: &WorkspaceContext,
) -> impl Iterator<Item = &FunctionDefinition> {
    context
        .function_definitions()
        .into_iter()
        .filter(|function| {
            (function.visibility == Visibility::Public
                || function.visibility == Visibility::External)
                && function.implemented
        })
}

pub fn pragma_directive_to_semver(pragma_directive: &PragmaDirective) -> Result<VersionReq, Error> {
    let mut version_string = String::new();

    for literal in &pragma_directive.literals {
        if literal == "solidity" {
            continue;
        }
        if version_string.is_empty() && literal.contains("0.") {
            version_string.push('=');
        }
        if version_string.len() > 5 && (literal == "<" || literal == "=") {
            version_string.push(',');
        }
        version_string.push_str(literal);
    }
    VersionReq::parse(&version_string)
}

// Check if a function definition has a `msg.sender` binary operation.
// Examples:
// ```
// function foo() public {
//     require(msg.sender == owner);
// }
// ```
pub fn has_msg_sender_binary_operation(function_definition: &FunctionDefinition) -> bool {
    // Directly return the evaluation of the condition
    ExtractBinaryOperations::from(function_definition)
        .extracted
        .iter()
        .any(|binary_operation| {
            ExtractMemberAccesses::from(binary_operation)
                .extracted
                .iter()
                .any(|member_access| {
                    member_access.member_name == "sender"
                        && if let Expression::Identifier(identifier) =
                            member_access.expression.as_ref()
                        {
                            identifier.name == "msg"
                        } else {
                            false
                        }
                })
        })
}

pub fn get_literal_value_or_constant_variable_value(
    node_id: NodeID,
    context: &WorkspaceContext,
) -> Option<String> {
    fn get_constant_variable_declaration_value(variable: &VariableDeclaration) -> Option<String> {
        if variable.mutability() == Some(&crate::ast::Mutability::Constant) {
            if let Some(Expression::Literal(literal)) = variable.value.as_ref() {
                return literal.value.to_owned();
            }
        }
        None
    }

    if let Some(node) = context.nodes.get(&node_id) {
        match node {
            ASTNode::Literal(literal) => return literal.value.to_owned(),
            ASTNode::VariableDeclaration(variable) => {
                return get_constant_variable_declaration_value(variable);
            }
            _ => (),
        }
    }
    None
}

pub fn get_node_offset(node: &ASTNode) -> Option<usize> {
    let src_location = node.src()?;

    let chopped_location = match src_location.rfind(':') {
        Some(index) => &src_location[..index],
        None => src_location, // No colon found, return the original string
    }
    .to_string();

    let (offset, _) = chopped_location.split_once(':').unwrap();
    offset.parse::<usize>().ok()
}
