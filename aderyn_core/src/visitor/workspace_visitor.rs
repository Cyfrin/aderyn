use super::{
    ast_visitor::ASTConstVisitor,
    macros::generate_visit_methods_for_workspace_context_with_insert_node,
};
use crate::{
    ast::*,
    context::workspace::{NodeContext, WorkspaceContext},
};
use eyre::Result;

impl ASTConstVisitor for WorkspaceContext {
    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ContractDefinition(node.clone()));
        self.contract_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        self.last_contract_definition_id = Some(node.id);
        Ok(true)
    }

    fn end_visit_contract_definition(&mut self, _: &ContractDefinition) -> Result<()> {
        self.last_contract_definition_id = None;
        Ok(())
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::FunctionDefinition(node.clone()));
        self.function_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        self.last_function_definition_id = Some(node.id);
        Ok(true)
    }

    fn end_visit_function_definition(&mut self, _: &FunctionDefinition) -> Result<()> {
        self.last_function_definition_id = None;
        Ok(())
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ModifierDefinition(node.clone()));
        self.modifier_definitions_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        self.last_modifier_definition_id = Some(node.id);
        Ok(true)
    }

    fn end_visit_modifier_definition(&mut self, _: &ModifierDefinition) -> Result<()> {
        self.last_modifier_definition_id = None;
        Ok(())
    }

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::SourceUnit(node.clone()));
        self.source_units_context.push(node.clone());
        self.last_source_unit_id = node.id;
        Ok(true)
    }

    fn visit_yul_function_call(&mut self, node: &YulFunctionCall) -> Result<bool> {
        self.yul_function_calls_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_yul_identifier(&mut self, node: &YulIdentifier) -> Result<bool> {
        // No node ID in Yul
        self.yul_identifiers_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    fn visit_yul_assignment(&mut self, node: &YulAssignment) -> Result<bool> {
        self.yul_assignments_context.insert(
            node.clone(),
            NodeContext {
                source_unit_id: self.last_source_unit_id,
                contract_definition_id: self.last_contract_definition_id,
                function_definition_id: self.last_function_definition_id,
                modifier_definition_id: self.last_modifier_definition_id,
            },
        );
        Ok(true)
    }

    // Read the following like follows -
    generate_visit_methods_for_workspace_context_with_insert_node! {
        // Explanation for the 1st one : Create a method called `visit_assignment` that takes in `Assignment` as parameter and puts it inside `assignments_context`
        visit_assignment | Assignment => assignments_context |,
        visit_binary_operation | BinaryOperation => binary_operations_context |,
        visit_block | Block => blocks_context |,
        visit_conditional | Conditional => conditionals_context |,
        visit_elementary_type_name_expression | ElementaryTypeNameExpression => elementary_type_name_expressions_context |,
        visit_enum_definition | EnumDefinition => enum_definitions_context |,
        visit_enum_value | EnumValue => enum_values_context |,
        visit_event_definition | EventDefinition => event_definitions_context |,
        visit_error_definition | ErrorDefinition => error_definitions_context |,
        visit_function_call | FunctionCall => function_calls_context |,
        visit_function_call_options | FunctionCallOptions => function_call_options_context |,
        visit_for_statement | ForStatement => for_statements_context |,
        visit_identifier | Identifier => identifiers_context |,
        visit_identifier_path | IdentifierPath => identifier_paths_context |,
        visit_if_statement | IfStatement => if_statements_context |,
        visit_import_directive | ImportDirective => import_directives_context |,
        visit_index_access | IndexAccess => index_accesses_context |,
        visit_index_range_access | IndexRangeAccess => index_range_accesses_context |,
        visit_inheritance_specifier | InheritanceSpecifier => inheritance_specifiers_context |,
        visit_inline_assembly | InlineAssembly => inline_assemblies_context |,
        visit_literal | Literal => literals_context |,
        visit_member_access | MemberAccess => member_accesses_context |,
        visit_new_expression | NewExpression => new_expressions_context |,
        visit_modifier_invocation | ModifierInvocation => modifier_invocations_context |,
        visit_override_specifier | OverrideSpecifier => override_specifiers_context |,
        visit_parameter_list | ParameterList => parameter_lists_context |,
        visit_pragma_directive | PragmaDirective => pragma_directives_context |,
        visit_return | Return => returns_context |,
        visit_struct_definition | StructDefinition => struct_definitions_context |,
        visit_structured_documentation | StructuredDocumentation => structured_documentations_context |,
        visit_tuple_expression | TupleExpression => tuple_expressions_context |,
        visit_unary_operation | UnaryOperation => unary_operations_context |,
        visit_unchecked_block | UncheckedBlock => unchecked_blocks_context |,
        visit_user_defined_value_type_definition | UserDefinedValueTypeDefinition => user_defined_value_type_definitions_context |,
        visit_using_for_directive | UsingForDirective => using_for_directives_context |,
        visit_variable_declaration | VariableDeclaration => variable_declarations_context |,
        visit_variable_declaration_statement | VariableDeclarationStatement => variable_declaration_statements_context |,
        visit_while_statement | WhileStatement => while_statements_context |,
        visit_do_while_statement | DoWhileStatement => do_while_statements_context |,
        visit_break_statement | Break => break_statements_context |,
        visit_continue_statement | Continue => continue_statements_context |,
        visit_placeholder_statement | PlaceholderStatement => placeholder_statements_context |,
        visit_array_type_name | ArrayTypeName => array_type_names_context |,
        visit_mapping | Mapping => mappings_context |,
        visit_try_statement | TryStatement => try_statements_context |,
        visit_try_catch_clause | TryCatchClause => try_catch_clauses_context |,
        visit_user_defined_type_name | UserDefinedTypeName => user_defined_type_names_context |,
        visit_expression_statement | ExpressionStatement => expression_statements_context |,
        visit_revert_statement | RevertStatement => revert_statements_context |,
        visit_emit_statement | EmitStatement => emit_statements_context |,
        visit_elementary_type_name | ElementaryTypeName => elementary_type_names_context |,
        visit_function_type_name | FunctionTypeName => function_type_names_context |,
    }

    fn visit_immediate_children(
        &mut self,
        node_id: NodeID,
        node_children_ids: Vec<NodeID>,
    ) -> Result<()> {
        for id in node_children_ids {
            self.parent_link.insert(id, node_id);
        }
        Ok(())
    }
}
