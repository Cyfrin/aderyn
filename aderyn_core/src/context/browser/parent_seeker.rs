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
            x.contract_definition_id?;
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
            x.function_definition_id?;
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
            x.modifier_definition_id?;
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
            x.contract_definition_id?;
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
            x.function_definition_id?;
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
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// BinaryOperation SeekParent allows us to finction an BinaryOperation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for BinaryOperation {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.binary_operations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.binary_operations.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.binary_operations.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.binary_operations.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Block SeekParent allows us to finction an Block's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for Block {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.blocks.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.blocks.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.blocks.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.blocks.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Conditional SeekParent allows us to finction an Conditional's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for Conditional {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.conditionals.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.conditionals.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.conditionals.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.conditionals.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ContractDefinition SeekParent allows us to finction an ContractDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ContractDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.contract_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
        None
    }

    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ElementaryTypeName SeekParent allows us to finction an ElementaryTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ElementaryTypeName {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.elementary_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.elementary_type_names.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.elementary_type_names.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.elementary_type_names.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ElementaryTypeNameExpression SeekParent allows us to finction an ElementaryTypeNameExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ElementaryTypeNameExpression {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader
            .elementary_type_name_expressions
            .get(self)
            .and_then(move |x| {
                loader
                    .source_units
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader
            .elementary_type_name_expressions
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                loader
                    .contract_definitions
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader
            .elementary_type_name_expressions
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                loader
                    .function_definitions
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader
            .elementary_type_name_expressions
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                loader
                    .modifier_definitions
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// EmitStatement SeekParent allows us to finction an EmitStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for EmitStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.emit_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.emit_statements.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.emit_statements.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.emit_statements.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// EnumDefinition SeekParent allows us to finction an EnumDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for EnumDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.enum_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.enum_definitions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// EnumValue SeekParent allows us to finction an EnumValue's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for EnumValue {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.enum_values.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.enum_values.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// EventDefinition SeekParent allows us to finction an EventDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for EventDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.event_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.event_definitions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ErrorDefinition SeekParent allows us to finction an ErrorDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ErrorDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.error_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.error_definitions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ExpressionStatement SeekParent allows us to finction an ExpressionStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ExpressionStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.expression_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.expression_statements.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.expression_statements.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.expression_statements.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// FunctionCall SeekParent allows us to finction an FunctionCall's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for FunctionCall {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_calls.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.function_calls.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.function_calls.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.function_calls.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// FunctionCallOptions SeekParent allows us to finction an FunctionCallOptions' parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for FunctionCallOptions {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_call_options.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.function_call_options.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.function_call_options.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.function_call_options.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// FunctionDefinition SeekParent allows us to finction an FunctionDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for FunctionDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.function_definitions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// FunctionTypeName SeekParent allows us to finction an FunctionTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for FunctionTypeName {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.function_type_names.get(self).and_then(move |x| {
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.function_type_names.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.function_type_names.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ForStatement SeekParent allows us to finction an ForStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ForStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.for_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.for_statements.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.for_statements.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.for_statements.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Identifier SeekParent allows us to finction an Identifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for Identifier {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.identifiers.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.identifiers.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.identifiers.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.identifiers.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// IdentifierPath SeekParent allows us to finction an IdentifierPath's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for IdentifierPath {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.identifier_paths.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.identifier_paths.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.identifier_paths.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.identifier_paths.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// IfStatement SeekParent allows us to finction an IfStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for IfStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.if_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.if_statements.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.if_statements.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.if_statements.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ImportDirective SeekParent allows us to finction an ImportDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ImportDirective {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.import_directives.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.import_directives.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.import_directives.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.import_directives.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// IndexAccess SeekParent allows us to finction an IndexAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for IndexAccess {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.index_accesses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.index_accesses.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.index_accesses.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.index_accesses.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// IndexRangeAccess SeekParent allows us to finction an IndexRangeAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for IndexRangeAccess {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.index_range_accesses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.index_range_accesses.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.index_range_accesses.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.index_range_accesses.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// InheritanceSpecifier SeekParent allows us to finction an InheritanceSpecifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for InheritanceSpecifier {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.inheritance_specifiers.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.inheritance_specifiers.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// InlineAssembly SeekParent allows us to finction an InlineAssembly's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for InlineAssembly {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.inline_assemblies.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.inline_assemblies.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.inline_assemblies.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.inline_assemblies.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Literal SeekParent allows us to finction an Literal's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for Literal {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.literals.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.literals.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.literals.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.literals.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// MemberAccess SeekParent allows us to finction an MemberAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for MemberAccess {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.member_accesses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.member_accesses.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.member_accesses.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.member_accesses.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// NewExpression SeekParent allows us to finction an NewExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for NewExpression {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.new_expressions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.new_expressions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.new_expressions.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.new_expressions.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Mapping SeekParent allows us to finction an Mapping's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for Mapping {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.mappings.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.mappings.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.mappings.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.mappings.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ModifierDefinition SeekParent allows us to finction an ModifierDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ModifierDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.modifier_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.modifier_definitions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.modifier_definitions.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ModifierInvocation SeekParent allows us to finction an ModifierInvocation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ModifierInvocation {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.modifier_invocations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.modifier_invocations.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.modifier_invocations.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// OverrideSpecifier SeekParent allows us to finction an OverrideSpecifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for OverrideSpecifier {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.override_specifiers.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.override_specifiers.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.override_specifiers.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.override_specifiers.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ParameterList SeekParent allows us to finction an ParameterList's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for ParameterList {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.parameter_lists.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.parameter_lists.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.parameter_lists.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.parameter_lists.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// PragmaDirective SeekParent allows us to finction an PragmaDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for PragmaDirective {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.pragma_directives.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
        None
    }
    fn function_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// Return SeekParent allows us to finction an Return's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for Return {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.returns.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.returns.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.returns.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.returns.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// RevertStatement SeekParent allows us to finction an RevertStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for RevertStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.revert_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.revert_statements.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.revert_statements.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.revert_statements.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// StructDefinition SeekParent allows us to finction an StructDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for StructDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.struct_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.struct_definitions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.struct_definitions.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.struct_definitions.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// StructuredDocumentation SeekParent allows us to finction an StructuredDocumentation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for StructuredDocumentation {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader
            .structured_documentations
            .get(self)
            .and_then(move |x| {
                loader
                    .source_units
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader
            .structured_documentations
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                loader
                    .contract_definitions
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader
            .structured_documentations
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                loader
                    .function_definitions
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader
            .structured_documentations
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                loader
                    .modifier_definitions
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// TryStatement SeekParent allows us to finction an TryStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for TryStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.try_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.try_statements.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.try_statements.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.try_statements.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// TryCatchClause SeekParent allows us to finction an TryCatchClause's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for TryCatchClause {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.try_catch_clauses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.try_catch_clauses.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.try_catch_clauses.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.try_catch_clauses.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// TupleExpression SeekParent allows us to finction an TupleExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for TupleExpression {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.tuple_expressions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.tuple_expressions.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.tuple_expressions.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.tuple_expressions.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// UnaryOperation SeekParent allows us to finction an UnaryOperation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for UnaryOperation {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.unary_operations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.unary_operations.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.unary_operations.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.unary_operations.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// UserDefinedTypeName SeekParent allows us to finction an UserDefinedTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for UserDefinedTypeName {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.user_defined_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.user_defined_type_names.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.user_defined_type_names.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.user_defined_type_names.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// UsingStatement SeekParent allows us to finction an UsingStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for UserDefinedValueTypeDefinition {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader
            .user_defined_value_type_definitions
            .get(self)
            .and_then(move |x| {
                loader
                    .source_units
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader
            .user_defined_value_type_definitions
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                loader
                    .contract_definitions
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader
            .user_defined_value_type_definitions
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                loader
                    .function_definitions
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader
            .user_defined_value_type_definitions
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                loader
                    .modifier_definitions
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// UsingForDirective SeekParent allows us to finction an UsingForDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for UsingForDirective {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.using_for_directives.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.using_for_directives.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.using_for_directives.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.using_for_directives.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// VariableDeclaration SeekParent allows us to finction an VariableDeclaration's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for VariableDeclaration {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.variable_declarations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.variable_declarations.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.variable_declarations.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.variable_declarations.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// VariableDeclarationStatement SeekParent allows us to finction an VariableDeclarationStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for VariableDeclarationStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader
            .variable_declaration_statements
            .get(self)
            .and_then(move |x| {
                loader
                    .source_units
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader
            .variable_declaration_statements
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                loader
                    .contract_definitions
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader
            .variable_declaration_statements
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                loader
                    .function_definitions
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader
            .variable_declaration_statements
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                loader
                    .modifier_definitions
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// WhileStatement SeekParent allows us to finction an WhileStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl SeekParent for WhileStatement {
    fn source_unit<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.while_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ContractDefinition> {
        loader.while_statements.get(self).and_then(move |x| {
            x.contract_definition_id?;
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a FunctionDefinition> {
        loader.while_statements.get(self).and_then(move |x| {
            x.function_definition_id?;
            loader
                .function_definitions
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition<'a>(&self, loader: &'a ContextLoader) -> Option<&'a ModifierDefinition> {
        loader.while_statements.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            loader
                .modifier_definitions
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}
