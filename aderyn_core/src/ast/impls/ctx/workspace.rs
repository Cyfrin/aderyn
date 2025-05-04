use crate::{
    ast::{ContractDefinition, ContractKind},
    context::workspace_context::WorkspaceContext,
};

impl WorkspaceContext {
    pub fn deployable_contracts(&self) -> impl Iterator<Item = &ContractDefinition> {
        self.contract_definitions()
            .into_iter()
            .filter(|c| c.kind == ContractKind::Contract && !c.is_abstract)
    }
}
