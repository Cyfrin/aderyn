use crate::{ast::*, context::loader::ContextLoader};

/// GetParent allows us to finction an ASTNode's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
pub trait GetParent {
    /// Get the parent SourceUnit of an ASTNode
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit>;
    /// Get the parent ContractDefinition of an ASTNode
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition>;
    /// Get the parent FunctionDefinition of an ASTNode
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition>;
    /// Get the parent ModifierDefinition of an ASTNode
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition>;
}

// ArrayTypeName GetParent allows us to finction an ArrayTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ArrayTypeName {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.array_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// Assignment GetParent allows us to finction an Assignment's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Assignment {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.assignments.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// BinaryOperation GetParent allows us to finction an BinaryOperation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for BinaryOperation {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.binary_operations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// Block GetParent allows us to finction an Block's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Block {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.blocks.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// Conditional GetParent allows us to finction an Conditional's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Conditional {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.conditionals.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// ContractDefinition GetParent allows us to finction an ContractDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ContractDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.contract_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
        None
    }

    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ElementaryTypeName GetParent allows us to finction an ElementaryTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ElementaryTypeName {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.elementary_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// ElementaryTypeNameExpression GetParent allows us to finction an ElementaryTypeNameExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ElementaryTypeNameExpression {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
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

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// EmitStatement GetParent allows us to finction an EmitStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EmitStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.emit_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// EnumDefinition GetParent allows us to finction an EnumDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EnumDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.enum_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// EnumValue GetParent allows us to finction an EnumValue's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EnumValue {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.enum_values.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// EventDefinition GetParent allows us to finction an EventDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EventDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.event_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ErrorDefinition GetParent allows us to finction an ErrorDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ErrorDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.error_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ExpressionStatement GetParent allows us to finction an ExpressionStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ExpressionStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.expression_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// FunctionCall GetParent allows us to finction an FunctionCall's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionCall {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_calls.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// FunctionCallOptions GetParent allows us to finction an FunctionCallOptions' parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionCallOptions {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_call_options.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// FunctionDefinition GetParent allows us to finction an FunctionDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// FunctionTypeName GetParent allows us to finction an FunctionTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionTypeName {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.function_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
        loader.function_type_names.get(self).and_then(move |x| {
            loader
                .contract_definitions
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// ForStatement GetParent allows us to finction an ForStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ForStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.for_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// Identifier GetParent allows us to finction an Identifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Identifier {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.identifiers.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// IdentifierPath GetParent allows us to finction an IdentifierPath's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IdentifierPath {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.identifier_paths.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// IfStatement GetParent allows us to finction an IfStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IfStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.if_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// ImportDirective GetParent allows us to finction an ImportDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ImportDirective {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.import_directives.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// IndexAccess GetParent allows us to finction an IndexAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IndexAccess {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.index_accesses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// IndexRangeAccess GetParent allows us to finction an IndexRangeAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IndexRangeAccess {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.index_range_accesses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// InheritanceSpecifier GetParent allows us to finction an InheritanceSpecifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for InheritanceSpecifier {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.inheritance_specifiers.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// InlineAssembly GetParent allows us to finction an InlineAssembly's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for InlineAssembly {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.inline_assemblies.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// Literal GetParent allows us to finction an Literal's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Literal {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.literals.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// MemberAccess GetParent allows us to finction an MemberAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for MemberAccess {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.member_accesses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// NewExpression GetParent allows us to finction an NewExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for NewExpression {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.new_expressions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// Mapping GetParent allows us to finction an Mapping's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Mapping {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.mappings.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// ModifierDefinition GetParent allows us to finction an ModifierDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ModifierDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.modifier_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ModifierInvocation GetParent allows us to finction an ModifierInvocation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ModifierInvocation {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.modifier_invocations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// OverrideSpecifier GetParent allows us to finction an OverrideSpecifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for OverrideSpecifier {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.override_specifiers.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// ParameterList GetParent allows us to finction an ParameterList's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ParameterList {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.parameter_lists.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// PragmaDirective GetParent allows us to finction an PragmaDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for PragmaDirective {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.pragma_directives.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
        None
    }
    fn function_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition_of<'a>(
        &self,
        _loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// Return GetParent allows us to finction an Return's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Return {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.returns.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// RevertStatement GetParent allows us to finction an RevertStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for RevertStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.revert_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// StructDefinition GetParent allows us to finction an StructDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for StructDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.struct_definitions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// StructuredDocumentation GetParent allows us to finction an StructuredDocumentation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for StructuredDocumentation {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
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
    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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
    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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
    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// TryStatement GetParent allows us to finction an TryStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for TryStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.try_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// TryCatchClause GetParent allows us to finction an TryCatchClause's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for TryCatchClause {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.try_catch_clauses.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// TupleExpression GetParent allows us to finction an TupleExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for TupleExpression {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.tuple_expressions.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// UnaryOperation GetParent allows us to finction an UnaryOperation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UnaryOperation {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.unary_operations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// UserDefinedTypeName GetParent allows us to finction an UserDefinedTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UserDefinedTypeName {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.user_defined_type_names.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// UsingStatement GetParent allows us to finction an UsingStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UserDefinedValueTypeDefinition {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
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

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// UsingForDirective GetParent allows us to finction an UsingForDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UsingForDirective {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.using_for_directives.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// VariableDeclaration GetParent allows us to finction an VariableDeclaration's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for VariableDeclaration {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.variable_declarations.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// VariableDeclarationStatement GetParent allows us to finction an VariableDeclarationStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for VariableDeclarationStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
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

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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

// WhileStatement GetParent allows us to finction an WhileStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for WhileStatement {
    fn source_unit_of<'a>(&self, loader: &'a ContextLoader) -> Option<&'a SourceUnit> {
        loader.while_statements.get(self).and_then(move |x| {
            loader
                .source_units
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ContractDefinition> {
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

    fn function_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a FunctionDefinition> {
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

    fn modifier_definition_of<'a>(
        &self,
        loader: &'a ContextLoader,
    ) -> Option<&'a ModifierDefinition> {
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
