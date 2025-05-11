use rayon::iter::{IntoParallelIterator, ParallelBridge};

use crate::{
    ast::{ContractDefinition, FunctionDefinition},
    context::{
        graph::{CallGraphConsumer, CallGraphDirection},
        workspace::WorkspaceContext,
    },
};

impl WorkspaceContext {
    #[inline]
    pub fn deployable_contracts(&self) -> impl Iterator<Item = &ContractDefinition> {
        self.contract_definitions().into_iter().filter(|c| c.is_deployable_contract())
    }

    #[inline]
    pub fn par_deployable_contracts(
        &self,
    ) -> impl IntoParallelIterator<Item = &ContractDefinition> {
        self.contract_definitions().into_iter().filter(|c| c.is_deployable_contract()).par_bridge()
    }

    pub fn entrypoints_of_deployable_contracts(&self) -> Vec<&FunctionDefinition> {
        let mut entrypoints = vec![];
        for contract in self.deployable_contracts() {
            if let Some(entrypoint_funcs) = contract.entrypoint_functions(self) {
                entrypoints.extend(entrypoint_funcs);
            }
        }
        entrypoints
    }

    pub fn entrypoints_with_callgraphs(
        &self,
    ) -> Vec<(&FunctionDefinition, Vec<CallGraphConsumer>)> {
        let mut result = vec![];
        for func in self.entrypoints_of_deployable_contracts() {
            let Ok(callgraphs) =
                CallGraphConsumer::get(self, &[&(func.into())], CallGraphDirection::Inward)
            else {
                continue;
            };
            result.push((func, callgraphs));
        }
        result
    }
}
