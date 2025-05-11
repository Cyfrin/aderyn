use semver::{Error, VersionReq};

use crate::{
    ast::{
        ASTNode, Expression, FunctionDefinition, Identifier, LiteralKind, MemberAccess, NodeID,
        PragmaDirective, Visibility,
    },
    context::{
        browser::{
            ExtractBinaryOperations, ExtractFunctionCallOptions, ExtractFunctionCalls,
            ExtractMemberAccesses,
        },
        workspace::WorkspaceContext,
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
    context.function_definitions().into_iter().filter(|function| {
        (function.visibility == Visibility::Public || function.visibility == Visibility::External)
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

// Check if an ast_node has a `msg.sender` binary operation.
// Examples:
// ```
// function foo() public {
//     require(msg.sender == owner);
// }
// ```
pub fn has_msg_sender_binary_operation(ast_node: &ASTNode) -> bool {
    // Directly return the evaluation of the condition
    ExtractBinaryOperations::from(ast_node).extracted.iter().any(|binary_operation| {
        ExtractMemberAccesses::from(binary_operation).extracted.iter().any(|member_access| {
            member_access.member_name == "sender"
                && if let Expression::Identifier(identifier) = member_access.expression.as_ref() {
                    identifier.name == "msg"
                } else {
                    false
                }
        })
    })
}

// Check if an ast_node sends native eth
// Examples:
// ```
// function foo() public {
//     address(0x1).call{value: 10}("...")
// }
// ```
pub fn has_calls_that_sends_native_eth(ast_node: &ASTNode) -> bool {
    // Check for address(..).call{value: 10}("...") pattern
    let function_call_ops = ExtractFunctionCallOptions::from(ast_node).extracted;
    for function_call in &function_call_ops {
        let call_carries_value = function_call.names.contains(&String::from("value"));
        if !call_carries_value {
            continue;
        }
        if let Expression::MemberAccess(member_access) = function_call.expression.as_ref() {
            let is_call = member_access.member_name == "call";
            if !is_call {
                continue;
            }
        }
        return true;
    }

    // Now, check for :-

    // payable(address(..)).transfer(100)
    // payable(address(..)).send(100)
    // address.sendValue(..) (from openzeppelin)

    let function_calls = ExtractFunctionCalls::from(ast_node).extracted;

    for function_call in function_calls {
        if let Expression::MemberAccess(member_access) = function_call.expression.as_ref() {
            if member_access.member_name == "transfer"
                || member_access.member_name == "send"
                || member_access.member_name == "sendValue"
            {
                if let Some(type_description) = member_access.expression.type_descriptions() {
                    if type_description
                        .type_string
                        .as_ref()
                        .is_some_and(|type_string| type_string.starts_with("address"))
                    {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Detects for the pattern
/// x.delegatecall("...") where x is not a state variable
/// That means, it can be
/// a) An Identifier that references a variable declaration which is not `state_variable`
/// b) A literal address
pub fn has_delegate_calls_on_non_state_variables(
    ast_node: &ASTNode,
    context: &WorkspaceContext,
) -> bool {
    let member_accesses = ExtractMemberAccesses::from(ast_node).extracted;
    member_accesses.into_iter().any(|member| {
        let is_delegate_call = member.member_name == "delegatecall";
        let mut is_on_non_state_variable = false;
        if let Expression::Identifier(identifier) = member.expression.as_ref() {
            if let Some(referenced_id) = identifier.referenced_declaration {
                if let Some(ASTNode::VariableDeclaration(v)) = context.nodes.get(&referenced_id) {
                    if !v.state_variable {
                        is_on_non_state_variable = true;
                    }
                }
            }
        } else if let Expression::Literal(_) = member.expression.as_ref() {
            is_on_non_state_variable = true;
        }
        is_delegate_call && is_on_non_state_variable
    })
}

/// Detects for the pattern
/// x.call("...") where x is not a state variable
/// That means, it can be
/// a) An Identifier that references a variable declaration which is not `state_variable`
/// b) A literal address
pub fn get_low_level_calls_on_non_state_variable_addresses(
    ast_node: &ASTNode,
    context: &WorkspaceContext,
) -> Vec<MemberAccess> {
    ExtractMemberAccesses::from(ast_node)
        .extracted
        .into_iter()
        .filter_map(|member| {
            if member.member_name != "call" {
                return None;
            }
            if let Expression::Identifier(identifier) = member.expression.as_ref() {
                if let Some(referenced_id) = identifier.referenced_declaration {
                    if let Some(ASTNode::VariableDeclaration(v)) = context.nodes.get(&referenced_id)
                    {
                        if !v.state_variable {
                            return Some(member);
                        }
                    }
                }
            } else if let Expression::Literal(_) = member.expression.as_ref() {
                return Some(member);
            }
            None
        })
        .collect::<_>()
}

pub fn has_binary_checks_on_some_address(ast_node: &ASTNode) -> bool {
    let binary_operations = ExtractBinaryOperations::from(ast_node).extracted;
    binary_operations.into_iter().any(|op| {
        [op.left_expression, op.right_expression].iter().any(|op| {
            op.as_ref().type_descriptions().is_some_and(|desc| {
                desc.type_string.as_ref().is_some_and(|type_string| {
                    type_string == "address" || type_string == "address payable"
                })
            })
        })
    })
}

pub fn get_literal_value_or_constant_variable_value(
    node_id: NodeID,
    context: &WorkspaceContext,
) -> Option<String> {
    match context.nodes.get(&node_id)? {
        ASTNode::Literal(literal) => return literal.value.to_owned(),
        ASTNode::VariableDeclaration(variable) => {
            if variable.mutability() == Some(&crate::ast::Mutability::Constant) {
                if let Some(Expression::Literal(literal)) = variable.value.as_ref() {
                    return literal.value.to_owned();
                }
            }
        }
        _ => (),
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

/*
Check if expression in constant
Expression::Literal whose value is false/true
Expression::Identifier that refers to a constant boolean variable declaration
Expression::UnaryOperation with ! operator followed by a sub expression that could be either of the above
*/
pub fn is_constant_boolean(context: &WorkspaceContext, ast_node: &Expression) -> bool {
    if let Expression::Literal(literal) = ast_node {
        if literal.kind == LiteralKind::Bool
            && literal.value.as_ref().is_some_and(|value| value == "false" || value == "true")
        {
            return true;
        }
    }
    if let Expression::Identifier(Identifier { referenced_declaration: Some(id), .. }) = ast_node {
        if let Some(ASTNode::VariableDeclaration(variable_declaration)) = context.nodes.get(id) {
            if variable_declaration
                .type_descriptions
                .type_string
                .as_ref()
                .is_some_and(|value| value == "bool")
                && variable_declaration.mutability() == Some(&crate::ast::Mutability::Constant)
            {
                return true;
            }
        }
    }
    if let Expression::UnaryOperation(operation) = ast_node {
        if operation.operator == "!" {
            return is_constant_boolean(context, operation.sub_expression.as_ref());
        }
    }
    false
}

/// List of [`ASTNode`]s that are in some kind of a loop
/// Typically used as starting points to explore inward from callgraph
pub fn get_explore_centers_of_loops(context: &WorkspaceContext) -> Vec<&ASTNode> {
    let mut explore_node_ids: Vec<Option<NodeID>> = vec![];

    for for_loop in context.for_statements() {
        if let Some(loop_expression) = for_loop.loop_expression.as_ref() {
            explore_node_ids.push(loop_expression.expression.get_node_id());
        }
        if let Some(condition) = &for_loop.condition {
            explore_node_ids.push(condition.get_node_id());
        }
        explore_node_ids.push(for_loop.body.get_node_id());
    }

    for while_loop in context.while_statements() {
        explore_node_ids.push(while_loop.condition.get_node_id());
        explore_node_ids.push(while_loop.body.get_node_id());
    }

    for do_while_loop in context.do_while_statements() {
        explore_node_ids.push(do_while_loop.condition.get_node_id());
        explore_node_ids.push(Some(do_while_loop.body.id));
    }

    let mut explore_nodes = Vec::with_capacity(explore_node_ids.len());

    for id in explore_node_ids.iter().flatten() {
        if let Some(ast_node) = context.nodes.get(id) {
            explore_nodes.push(ast_node);
        }
    }

    explore_nodes
}
