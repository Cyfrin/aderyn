use crate::{ast::ContractDefinition, context::workspace::WorkspaceContext};

impl WorkspaceContext {
    #[inline]
    pub fn deployable_contracts(&self) -> impl Iterator<Item = &ContractDefinition> {
        self.contract_definitions().into_iter().filter(|c| c.is_deployable_contract())
    }
}
