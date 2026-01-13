use super::render::*;
use crate::{
    ast::{
        ASTNode, ContractDefinition, ExtractVariableDeclarations, FunctionKind, NodeType,
        Visibility,
    },
    context::{browser::GetClosestAncestorOfTypeX, workspace::WorkspaceContext},
};
use rmcp::ErrorData as McpError;

pub fn get_total_state_variables(
    context: &WorkspaceContext,
    original_contract: &ContractDefinition,
) -> usize {
    let Some(state_vars) =
        original_contract.get_all_state_variables_in_linearized_base_contracts_chain(context)
    else {
        return 0;
    };
    state_vars.len()
}

pub fn get_classified_entrypoint_functions(
    context: &WorkspaceContext,
    original_contract: &ContractDefinition,
) -> Result<EntrypointFunctions, McpError> {
    let Some(functions) = context.entrypoint_functions(original_contract) else {
        // TODO: investigate when you hit this case, it has been a while since I made the router
        return Ok(EntrypointFunctions::default());
    };
    let mut public_functions_info = vec![];
    let mut external_functions_info = vec![];
    let mut receive_function_info = None;
    let mut fallback_function_info = None;
    for func in functions {
        let Some(ASTNode::ContractDefinition(container)) =
            func.closest_ancestor_of_type(context, NodeType::ContractDefinition)
        else {
            continue;
        };
        let container = ContainingContractBuilder::default()
            .name(container.name.clone())
            .node_id(container.id)
            .build()
            .expect("failed to build container");

        let function_info = FunctionInfoBuilder::default()
            .name(func.name.clone())
            .node_id(func.id)
            .containing_contract(container)
            .build()
            .expect("failed to build function info");

        match *func.kind() {
            FunctionKind::Function => match func.visibility {
                Visibility::Public => {
                    public_functions_info.push(function_info);
                }
                Visibility::External => {
                    external_functions_info.push(function_info);
                }
                Visibility::Internal | Visibility::Private => {} // not an entrypoint
            },
            FunctionKind::Receive => {
                receive_function_info = Some(function_info);
            }
            FunctionKind::Fallback => {
                fallback_function_info = Some(function_info);
            }
            FunctionKind::Constructor => {} // For now, constructor is not an entrypoint
            FunctionKind::FreeFunction => unreachable!(), // Free function is never an entrypoint
        };
    }

    let entrypoints = EntrypointFunctionsBuilder::default()
        .external_functions(external_functions_info)
        .public_functions(public_functions_info)
        .fallback_function(fallback_function_info)
        .receive_function(receive_function_info)
        .build()
        .expect("failed to build entrypoints in inheritance chain");
    Ok(entrypoints)
}

pub fn get_inheritance_chain_info(
    context: &WorkspaceContext,
    original_contract: &ContractDefinition,
) -> Result<Vec<ContractInfo>, McpError> {
    let mut reversed_chain = vec![];
    for contract in original_contract.c3(context).collect::<Vec<_>>().into_iter().rev() {
        let variables = ExtractVariableDeclarations::from(contract).extracted;
        let state_variables = variables
            .iter()
            .filter(|v| v.state_variable)
            .map(|v| v.name.clone())
            .collect::<Vec<_>>();
        let (filepath, _, _) = context.get_node_sort_key_from_capturable(&contract.into());

        let contract_info = ContractInfoBuilder::default()
            .name(contract.name.clone())
            .node_id(contract.id)
            .state_variables(state_variables)
            .filepath(filepath)
            .build()
            .expect("failed to build contract info in inheritance chain");
        reversed_chain.push(contract_info);
    }
    Ok(reversed_chain)
}
