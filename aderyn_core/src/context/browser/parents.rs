use crate::{ast::*, context::workspace_context::WorkspaceContext};

/// GetParent allows us to finction an ASTNode's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
pub trait GetParent {
    /// Get the parent SourceUnit of an ASTNode
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit>;
    /// Get the parent ContractDefinition of an ASTNode
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition>;
    /// Get the parent FunctionDefinition of an ASTNode
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition>;
    /// Get the parent ModifierDefinition of an ASTNode
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition>;
}

// ArrayTypeName GetParent allows us to finction an ArrayTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ArrayTypeName {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .array_type_names_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .array_type_names_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .array_type_names_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .array_type_names_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// Assignment GetParent allows us to finction an Assignment's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Assignment {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.assignments_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.assignments_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.assignments_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.assignments_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// BinaryOperation GetParent allows us to finction an BinaryOperation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for BinaryOperation {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .binary_operations_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .binary_operations_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .binary_operations_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .binary_operations_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// Block GetParent allows us to finction an Block's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Block {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.blocks_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.blocks_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.blocks_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.blocks_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Conditional GetParent allows us to finction an Conditional's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Conditional {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.conditionals_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.conditionals_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.conditionals_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.conditionals_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ContractDefinition GetParent allows us to finction an ContractDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ContractDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .contract_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        None
    }

    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ElementaryTypeName GetParent allows us to finction an ElementaryTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ElementaryTypeName {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .elementary_type_names_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .elementary_type_names_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .elementary_type_names_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .elementary_type_names_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// ElementaryTypeNameExpression GetParent allows us to finction an ElementaryTypeNameExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ElementaryTypeNameExpression {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .elementary_type_name_expressions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .elementary_type_name_expressions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .elementary_type_name_expressions_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .elementary_type_name_expressions_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// EmitStatement GetParent allows us to finction an EmitStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EmitStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .emit_statements_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .emit_statements_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .emit_statements_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .emit_statements_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// EnumDefinition GetParent allows us to finction an EnumDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EnumDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .enum_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .enum_definitions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// EnumValue GetParent allows us to finction an EnumValue's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EnumValue {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.enum_values_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.enum_values_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// EventDefinition GetParent allows us to finction an EventDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for EventDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .event_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .event_definitions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ErrorDefinition GetParent allows us to finction an ErrorDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ErrorDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .error_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .error_definitions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }

    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ExpressionStatement GetParent allows us to finction an ExpressionStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ExpressionStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .expression_statements_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .expression_statements_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .expression_statements_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .expression_statements_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// FunctionCall GetParent allows us to finction an FunctionCall's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionCall {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.function_calls_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.function_calls_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.function_calls_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.function_calls_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// FunctionCallOptions GetParent allows us to finction an FunctionCallOptions' parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionCallOptions {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .function_call_options_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .function_call_options_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .function_call_options_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .function_call_options_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// FunctionDefinition GetParent allows us to finction an FunctionDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .function_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .function_definitions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// FunctionTypeName GetParent allows us to finction an FunctionTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for FunctionTypeName {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .function_type_names_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .function_type_names_context
            .get(self)
            .and_then(move |x| {
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .function_type_names_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .function_type_names_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// ForStatement GetParent allows us to finction an ForStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ForStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.for_statements_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.for_statements_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.for_statements_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.for_statements_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// Identifier GetParent allows us to finction an Identifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Identifier {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.identifiers_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.identifiers_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.identifiers_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.identifiers_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// IdentifierPath GetParent allows us to finction an IdentifierPath's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IdentifierPath {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .identifier_paths_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .identifier_paths_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .identifier_paths_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .identifier_paths_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// IfStatement GetParent allows us to finction an IfStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IfStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.if_statements_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.if_statements_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.if_statements_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.if_statements_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ImportDirective GetParent allows us to finction an ImportDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ImportDirective {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .import_directives_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .import_directives_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .import_directives_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .import_directives_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// IndexAccess GetParent allows us to finction an IndexAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IndexAccess {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.index_accesses_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.index_accesses_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.index_accesses_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.index_accesses_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// IndexRangeAccess GetParent allows us to finction an IndexRangeAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for IndexRangeAccess {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .index_range_accesses_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .index_range_accesses_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .index_range_accesses_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .index_range_accesses_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// InheritanceSpecifier GetParent allows us to finction an InheritanceSpecifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for InheritanceSpecifier {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .inheritance_specifiers_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .inheritance_specifiers_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// InlineAssembly GetParent allows us to finction an InlineAssembly's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for InlineAssembly {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .inline_assemblies_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .inline_assemblies_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .inline_assemblies_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .inline_assemblies_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// Literal GetParent allows us to finction an Literal's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Literal {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.literals_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.literals_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.literals_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.literals_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// MemberAccess GetParent allows us to finction an MemberAccess's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for MemberAccess {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .member_accesses_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .member_accesses_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .member_accesses_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .member_accesses_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// NewExpression GetParent allows us to finction an NewExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for NewExpression {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .new_expressions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .new_expressions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .new_expressions_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .new_expressions_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// Mapping GetParent allows us to finction an Mapping's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Mapping {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.mappings_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.mappings_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.mappings_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.mappings_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// ModifierDefinition GetParent allows us to finction an ModifierDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ModifierDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .modifier_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .modifier_definitions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .modifier_definitions_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// ModifierInvocation GetParent allows us to finction an ModifierInvocation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ModifierInvocation {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .modifier_invocations_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .modifier_invocations_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .modifier_invocations_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// OverrideSpecifier GetParent allows us to finction an OverrideSpecifier's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for OverrideSpecifier {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .override_specifiers_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .override_specifiers_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .override_specifiers_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .override_specifiers_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// ParameterList GetParent allows us to finction an ParameterList's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for ParameterList {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .parameter_lists_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .parameter_lists_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .parameter_lists_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .parameter_lists_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// PragmaDirective GetParent allows us to finction an PragmaDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for PragmaDirective {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .pragma_directives_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        None
    }
    fn function_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        None
    }
    fn modifier_definition_of<'a>(
        &self,
        _context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        None
    }
}

// Return GetParent allows us to finction an Return's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for Return {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.returns_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.returns_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.returns_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.returns_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// RevertStatement GetParent allows us to finction an RevertStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for RevertStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .revert_statements_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .revert_statements_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .revert_statements_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .revert_statements_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// StructDefinition GetParent allows us to finction an StructDefinition's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for StructDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .struct_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .struct_definitions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .struct_definitions_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .struct_definitions_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// StructuredDocumentation GetParent allows us to finction an StructuredDocumentation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for StructuredDocumentation {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .structured_documentations_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }
    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .structured_documentations_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }
    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .structured_documentations_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }
    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .structured_documentations_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// TryStatement GetParent allows us to finction an TryStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for TryStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context.try_statements_context.get(self).and_then(move |x| {
            context
                .source_units_context
                .iter()
                .find(|source_unit| source_unit.id == x.source_unit_id)
        })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context.try_statements_context.get(self).and_then(move |x| {
            x.contract_definition_id?;
            context
                .contract_definitions_context
                .keys()
                .find(|contract_definition| {
                    Some(contract_definition.id) == x.contract_definition_id
                })
        })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context.try_statements_context.get(self).and_then(move |x| {
            x.function_definition_id?;
            context
                .function_definitions_context
                .keys()
                .find(|function_definition| {
                    Some(function_definition.id) == x.function_definition_id
                })
        })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context.try_statements_context.get(self).and_then(move |x| {
            x.modifier_definition_id?;
            context
                .modifier_definitions_context
                .keys()
                .find(|modifier_definition| {
                    Some(modifier_definition.id) == x.modifier_definition_id
                })
        })
    }
}

// TryCatchClause GetParent allows us to finction an TryCatchClause's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for TryCatchClause {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .try_catch_clauses_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .try_catch_clauses_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .try_catch_clauses_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .try_catch_clauses_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// TupleExpression GetParent allows us to finction an TupleExpression's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for TupleExpression {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .tuple_expressions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .tuple_expressions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .tuple_expressions_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .tuple_expressions_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// UnaryOperation GetParent allows us to finction an UnaryOperation's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UnaryOperation {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .unary_operations_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .unary_operations_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .unary_operations_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .unary_operations_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// UserDefinedTypeName GetParent allows us to finction an UserDefinedTypeName's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UserDefinedTypeName {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .user_defined_type_names_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .user_defined_type_names_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .user_defined_type_names_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .user_defined_type_names_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// UsingStatement GetParent allows us to finction an UsingStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UserDefinedValueTypeDefinition {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .user_defined_value_type_definitions_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .user_defined_value_type_definitions_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .user_defined_value_type_definitions_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .user_defined_value_type_definitions_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// UsingForDirective GetParent allows us to finction an UsingForDirective's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for UsingForDirective {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .using_for_directives_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .using_for_directives_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .using_for_directives_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .using_for_directives_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// VariableDeclaration GetParent allows us to finction an VariableDeclaration's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for VariableDeclaration {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .variable_declarations_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .variable_declarations_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .variable_declarations_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .variable_declarations_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// VariableDeclarationStatement GetParent allows us to finction an VariableDeclarationStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for VariableDeclarationStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .variable_declaration_statements_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .variable_declaration_statements_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .variable_declaration_statements_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .variable_declaration_statements_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}

// WhileStatement GetParent allows us to finction an WhileStatement's parent SourceUnit, ContractDefinition, FunctionDefinition or ModifierDefinition
impl GetParent for WhileStatement {
    fn source_unit_of<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a SourceUnit> {
        context
            .while_statements_context
            .get(self)
            .and_then(move |x| {
                context
                    .source_units_context
                    .iter()
                    .find(|source_unit| source_unit.id == x.source_unit_id)
            })
    }

    fn contract_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ContractDefinition> {
        context
            .while_statements_context
            .get(self)
            .and_then(move |x| {
                x.contract_definition_id?;
                context
                    .contract_definitions_context
                    .keys()
                    .find(|contract_definition| {
                        Some(contract_definition.id) == x.contract_definition_id
                    })
            })
    }

    fn function_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        context
            .while_statements_context
            .get(self)
            .and_then(move |x| {
                x.function_definition_id?;
                context
                    .function_definitions_context
                    .keys()
                    .find(|function_definition| {
                        Some(function_definition.id) == x.function_definition_id
                    })
            })
    }

    fn modifier_definition_of<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        context
            .while_statements_context
            .get(self)
            .and_then(move |x| {
                x.modifier_definition_id?;
                context
                    .modifier_definitions_context
                    .keys()
                    .find(|modifier_definition| {
                        Some(modifier_definition.id) == x.modifier_definition_id
                    })
            })
    }
}
