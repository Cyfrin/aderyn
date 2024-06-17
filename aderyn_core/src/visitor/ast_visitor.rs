use eyre::Result;

use crate::ast::*;

pub trait ASTConstVisitor {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_assignment(&mut self, node: &Assignment) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_block(&mut self, node: &Block) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_conditional(&mut self, node: &Conditional) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_emit_statement(&mut self, node: &EmitStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_enum_value(&mut self, node: &EnumValue) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_event_definition(&mut self, node: &EventDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_for_statement(&mut self, node: &ForStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_identifier(&mut self, node: &Identifier) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_if_statement(&mut self, node: &IfStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_import_directive(&mut self, node: &ImportDirective) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_index_access(&mut self, node: &IndexAccess) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_literal(&mut self, node: &Literal) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_member_access(&mut self, node: &MemberAccess) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_new_expression(&mut self, node: &NewExpression) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_mapping(&mut self, node: &Mapping) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_parameter_list(&mut self, node: &ParameterList) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_return(&mut self, node: &Return) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_revert_statement(&mut self, node: &RevertStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_source_unit(&mut self, node: &SourceUnit) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_struct_definition(&mut self, node: &StructDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_try_statement(&mut self, node: &TryStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_while_statement(&mut self, node: &WhileStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_do_while_statement(&mut self, node: &DoWhileStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_do_visit_while_statement(&mut self, node: &DoWhileStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_continue_statement(&mut self, node: &Continue) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_continue_statement(&mut self, node: &Continue) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_placeholder_statement(&mut self, node: &PlaceholderStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_placeholder_statement(&mut self, node: &PlaceholderStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_break_statement(&mut self, node: &Break) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_break_statement(&mut self, node: &Break) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_block(&mut self, node: &YulBlock) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_block(&mut self, node: &YulBlock) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_statement(&mut self, node: &YulStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_statement(&mut self, node: &YulStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_expression(&mut self, node: &YulExpression) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_expression(&mut self, node: &YulExpression) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_literal(&mut self, node: &YulLiteral) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_literal(&mut self, node: &YulLiteral) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_identifier(&mut self, node: &YulIdentifier) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_identifier(&mut self, node: &YulIdentifier) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_function_call(&mut self, node: &YulFunctionCall) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_function_call(&mut self, node: &YulFunctionCall) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_if(&mut self, node: &YulIf) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_if(&mut self, node: &YulIf) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_switch(&mut self, node: &YulSwitch) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_switch(&mut self, node: &YulSwitch) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_case(&mut self, node: &YulCase) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_case(&mut self, node: &YulCase) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_for_loop(&mut self, node: &YulForLoop) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_for_loop(&mut self, node: &YulForLoop) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_assignment(&mut self, node: &YulAssignment) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_assignment(&mut self, node: &YulAssignment) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_variable_declaration(&mut self, node: &YulVariableDeclaration) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_variable_declaration(&mut self, node: &YulVariableDeclaration) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_typed_name(&mut self, node: &YulTypedName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_typed_name(&mut self, node: &YulTypedName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_expression_statement(&mut self, node: &YulExpressionStatement) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_expression_statement(&mut self, node: &YulExpressionStatement) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_yul_function_definition(&mut self, node: &YulFunctionDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_yul_function_definition(&mut self, node: &YulFunctionDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_node(&mut self, _node: &impl Node) -> Result<bool> {
        Ok(true)
    }
    fn end_visit_node(&mut self, _node: &impl Node) -> Result<()> {
        Ok(())
    }

    fn visit_immediate_children(
        &mut self,
        _node_id: NodeID,
        _node_children_ids: Vec<NodeID>,
    ) -> Result<()> {
        Ok(())
    }

    fn visit_node_id(&mut self, _node_id: Option<NodeID>) -> Result<()> {
        Ok(())
    }
}

pub trait Node {
    /// [`Node::accept`] is designed to propagate
    fn accept(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    /// [`Node::accept_metadata`] is designed to propagate into the AST subtree
    /// although it doesn't happen by itself. [`Node::accept`] triggers the propagation
    fn accept_metadata(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    /// [`Node::accept_id`] is not designed to propagate into the AST subtree
    fn accept_id(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
}

pub fn list_accept(list: &Vec<impl Node>, visitor: &mut impl ASTConstVisitor) -> Result<()> {
    for elem in list {
        elem.accept(visitor)?;
    }
    Ok(())
}
