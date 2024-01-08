use crate::{ast::*, context::loader::ContextLoader};

pub trait SeekParent {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit>;
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition>;
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition>;
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition>;
}

// ArrayTypeName SeekParent allows us to finction an ArrayTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ArrayTypeName {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.array_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.array_type_names.get(self).and_then(move |x| {
            if x.contract_definition_id.is_none() {
                return None;
            }
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.array_type_names.get(self).and_then(move |x| {
            if x.function_definition_id.is_none() {
                return None;
            }
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.array_type_names.get(self).and_then(move |x| {
            if x.modifier_definition_id.is_none() {
                return None;
            }
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Assignment SeekParent allows us to finction an Assignment's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for Assignment {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.assignments.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.assignments.get(self).and_then(move |x| {
            if x.contract_definition_id.is_none() {
                return None;
            }
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.assignments.get(self).and_then(move |x| {
            if x.function_definition_id.is_none() {
                return None;
            }
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.assignments.get(self).and_then(move |x| {
            if x.modifier_definition_id.is_none() {
                return None;
            }
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}
