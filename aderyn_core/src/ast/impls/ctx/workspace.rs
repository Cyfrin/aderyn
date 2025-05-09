use rayon::iter::{IntoParallelIterator, ParallelBridge};

use crate::{ast::ContractDefinition, context::workspace::WorkspaceContext};

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
}
