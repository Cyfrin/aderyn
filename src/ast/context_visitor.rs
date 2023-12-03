use super::*;
use std::{collections::HashSet, io};
use yul::*;
#[derive(Debug, PartialEq)]
pub struct YulBlockContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_block: &'a YulBlock,
}
#[derive(Debug, PartialEq)]
pub struct YulStatementContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
}
#[derive(Debug, PartialEq)]
pub struct YulIfContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_if: &'a YulIf,
}
#[derive(Debug, PartialEq)]
pub struct YulSwitchContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_switch: &'a YulSwitch,
}
#[derive(Debug, PartialEq)]
pub struct YulForLoopContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_for_loop: &'a YulForLoop,
}
#[derive(Debug, PartialEq)]
pub struct YulCaseContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_switch: &'a YulSwitch,
    pub yul_case: &'a YulCase,
}
#[derive(Debug, PartialEq)]
pub struct YulAssignmentContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_assignment: &'a YulAssignment,
}
#[derive(Debug, PartialEq)]
pub struct YulVariableDeclarationContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_variable_declaration: &'a YulVariableDeclaration,
}
#[derive(Debug, PartialEq)]
pub struct YulExpressionStatementContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_expression_statement: &'a YulExpressionStatement,
}
#[derive(Debug, PartialEq)]
pub struct YulFunctionDefinitionContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: &'a YulStatement,
    pub yul_function_definition: &'a YulFunctionDefinition,
}
#[derive(Debug, PartialEq)]
pub struct YulTypedNameContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: Option<&'a YulStatement>,
    pub yul_typed_name: &'a YulTypedName,
}
#[derive(Debug, PartialEq)]
pub struct YulExpressionContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: Option<&'a YulStatement>,
    pub yul_expression: &'a YulExpression,
}
#[derive(Debug, PartialEq)]
pub struct YulLiteralContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: Option<&'a YulStatement>,
    pub yul_expression: &'a YulExpression,
    pub yul_literal: &'a YulLiteral,
}
#[derive(Debug, PartialEq)]
pub struct YulIdentifierContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: Option<&'a YulStatement>,
    pub yul_expression: Option<&'a YulExpression>,
    pub yul_identifier: &'a YulIdentifier,
}
#[derive(Debug, PartialEq)]
pub struct YulFunctionCallContext<'a, 'b, 'c> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
    pub inline_assembly: &'a InlineAssembly,
    pub yul_blocks: &'c mut Vec<&'a YulBlock>,
    pub yul_statement: Option<&'a YulStatement>,
    pub yul_expression: &'a YulExpression,
    pub yul_function_call: &'a YulFunctionCall,
}

#[allow(unused_variables)]
pub trait AstContextVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        Ok(())
    }
    fn leave_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        Ok(())
    }

    fn visit_pragma_directive<'a>(
        &mut self,
        context: &mut PragmaDirectiveContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_pragma_directive<'a>(
        &mut self,
        context: &mut PragmaDirectiveContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_import_directive<'a>(
        &mut self,
        context: &mut ImportDirectiveContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_import_directive<'a>(
        &mut self,
        context: &mut ImportDirectiveContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_struct_definition<'a>(
        &mut self,
        context: &mut StructDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_struct_definition<'a>(
        &mut self,
        context: &mut StructDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_enum_definition<'a>(
        &mut self,
        context: &mut EnumDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_enum_definition<'a>(
        &mut self,
        context: &mut EnumDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_contract_definition<'a>(
        &mut self,
        context: &mut ContractDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_contract_definition<'a>(
        &mut self,
        context: &mut ContractDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_using_for_directive<'a>(
        &mut self,
        context: &mut UsingForDirectiveContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_using_for_directive<'a>(
        &mut self,
        context: &mut UsingForDirectiveContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_variable_declaration<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_variable_declaration<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_event_definition<'a>(
        &mut self,
        context: &mut EventDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_event_definition<'a>(
        &mut self,
        context: &mut EventDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_error_definition<'a>(
        &mut self,
        context: &mut ErrorDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_error_definition<'a>(
        &mut self,
        context: &mut ErrorDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_modifier_definition<'a>(
        &mut self,
        context: &mut ModifierDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_modifier_definition<'a>(
        &mut self,
        context: &mut ModifierDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_user_defined_value_type_definition<'a>(
        &mut self,
        context: &mut UserDefinedValueTypeDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_user_defined_value_type_definition<'a>(
        &mut self,
        context: &mut UserDefinedValueTypeDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_function_definition<'a>(
        &mut self,
        context: &mut FunctionDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_function_definition<'a>(
        &mut self,
        context: &mut FunctionDefinitionContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_modifier_invocation<'a>(
        &mut self,
        context: &mut ModifierInvocationContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_modifier_invocation<'a>(
        &mut self,
        context: &mut ModifierInvocationContext<'a>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        Ok(())
    }
    fn leave_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        Ok(())
    }

    fn visit_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_variable_declaration_statement<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_variable_declaration_statement<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_if_statement<'a, 'b>(
        &mut self,
        context: &mut IfStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_if_statement<'a, 'b>(
        &mut self,
        context: &mut IfStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_for_statement<'a, 'b>(
        &mut self,
        context: &mut ForStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_for_statement<'a, 'b>(
        &mut self,
        context: &mut ForStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_while_statement<'a, 'b>(
        &mut self,
        context: &mut WhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_while_statement<'a, 'b>(
        &mut self,
        context: &mut WhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_do_while_statement<'a, 'b>(
        &mut self,
        context: &mut DoWhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_do_while_statement<'a, 'b>(
        &mut self,
        context: &mut DoWhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_emit_statement<'a, 'b>(
        &mut self,
        context: &mut EmitStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_emit_statement<'a, 'b>(
        &mut self,
        context: &mut EmitStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_try_statement<'a, 'b>(
        &mut self,
        context: &mut TryStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_try_statement<'a, 'b>(
        &mut self,
        context: &mut TryStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_revert_statement<'a, 'b>(
        &mut self,
        context: &mut RevertStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_revert_statement<'a, 'b>(
        &mut self,
        context: &mut RevertStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_continue_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_continue_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_break_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_break_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_placeholder_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_placeholder_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_block_or_statement<'a, 'b>(
        &mut self,
        context: &mut BlockOrStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_block_or_statement<'a, 'b>(
        &mut self,
        context: &mut BlockOrStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> {
        Ok(())
    }
    fn leave_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> {
        Ok(())
    }

    fn visit_expression<'a, 'b>(
        &mut self,
        context: &mut ExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_expression<'a, 'b>(
        &mut self,
        context: &mut ExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        Ok(())
    }
    fn leave_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        Ok(())
    }

    fn visit_identifier<'a, 'b>(
        &mut self,
        context: &mut IdentifierContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_identifier<'a, 'b>(
        &mut self,
        context: &mut IdentifierContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_unary_operation<'a, 'b>(
        &mut self,
        context: &mut UnaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_unary_operation<'a, 'b>(
        &mut self,
        context: &mut UnaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_binary_operation<'a, 'b>(
        &mut self,
        context: &mut BinaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_binary_operation<'a, 'b>(
        &mut self,
        context: &mut BinaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_conditional<'a, 'b>(
        &mut self,
        context: &mut ConditionalContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_conditional<'a, 'b>(
        &mut self,
        context: &mut ConditionalContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_assignment<'a, 'b>(
        &mut self,
        context: &mut AssignmentContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_assignment<'a, 'b>(
        &mut self,
        context: &mut AssignmentContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_function_call<'a, 'b>(
        &mut self,
        context: &mut FunctionCallContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_function_call<'a, 'b>(
        &mut self,
        context: &mut FunctionCallContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_function_call_options<'a, 'b>(
        &mut self,
        context: &mut FunctionCallOptionsContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_function_call_options<'a, 'b>(
        &mut self,
        context: &mut FunctionCallOptionsContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_index_access<'a, 'b>(
        &mut self,
        context: &mut IndexAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_index_access<'a, 'b>(
        &mut self,
        context: &mut IndexAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_index_range_access<'a, 'b>(
        &mut self,
        context: &mut IndexRangeAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_index_range_access<'a, 'b>(
        &mut self,
        context: &mut IndexRangeAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_member_access<'a, 'b>(
        &mut self,
        context: &mut MemberAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_member_access<'a, 'b>(
        &mut self,
        context: &mut MemberAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_elementary_type_name_expression<'a, 'b>(
        &mut self,
        context: &mut ElementaryTypeNameExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_elementary_type_name_expression<'a, 'b>(
        &mut self,
        context: &mut ElementaryTypeNameExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_tuple_expression<'a, 'b>(
        &mut self,
        context: &mut TupleExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_tuple_expression<'a, 'b>(
        &mut self,
        context: &mut TupleExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_new_expression<'a, 'b>(
        &mut self,
        context: &mut NewExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_new_expression<'a, 'b>(
        &mut self,
        context: &mut NewExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_inline_assembly<'a, 'b>(
        &mut self,
        context: &mut InlineAssemblyContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_inline_assembly<'a, 'b>(
        &mut self,
        context: &mut InlineAssemblyContext<'a, 'b>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_block<'a, 'b, 'c>(
        &mut self,
        context: &mut YulBlockContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_block<'a, 'b, 'c>(
        &mut self,
        context: &mut YulBlockContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_if<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIfContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_if<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIfContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_switch<'a, 'b, 'c>(
        &mut self,
        context: &mut YulSwitchContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_switch<'a, 'b, 'c>(
        &mut self,
        context: &mut YulSwitchContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_for_loop<'a, 'b, 'c>(
        &mut self,
        context: &mut YulForLoopContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_for_loop<'a, 'b, 'c>(
        &mut self,
        context: &mut YulForLoopContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_case<'a, 'b, 'c>(
        &mut self,
        context: &mut YulCaseContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_case<'a, 'b, 'c>(
        &mut self,
        context: &mut YulCaseContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_assignment<'a, 'b, 'c>(
        &mut self,
        context: &mut YulAssignmentContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_assignment<'a, 'b, 'c>(
        &mut self,
        context: &mut YulAssignmentContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_variable_declaration<'a, 'b, 'c>(
        &mut self,
        context: &mut YulVariableDeclarationContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_variable_declaration<'a, 'b, 'c>(
        &mut self,
        context: &mut YulVariableDeclarationContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_expression_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_expression_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_function_definition<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionDefinitionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_function_definition<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionDefinitionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_leave<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_leave<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_break<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_break<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_continue<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_continue<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_typed_name<'a, 'b, 'c>(
        &mut self,
        context: &mut YulTypedNameContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_typed_name<'a, 'b, 'c>(
        &mut self,
        context: &mut YulTypedNameContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_expression<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_expression<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_literal<'a, 'b, 'c>(
        &mut self,
        context: &mut YulLiteralContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_literal<'a, 'b, 'c>(
        &mut self,
        context: &mut YulLiteralContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_identifier<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIdentifierContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_identifier<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIdentifierContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_function_call<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionCallContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn leave_yul_function_call<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionCallContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        Ok(())
    }
}

pub struct AstContextVisitorData<'a> {
    pub analyzed_paths: HashSet<String>,
    pub visitors: Vec<Box<dyn AstContextVisitor + 'a>>,
}

impl AstContextVisitor for AstContextVisitorData<'_> {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_source_unit(context)?;
        }

        for node in context.current_source_unit.nodes.iter() {
            match node {
                SourceUnitNode::PragmaDirective(pragma_directive) => {
                    let mut context = context.create_pragma_directive_context(pragma_directive);
                    self.visit_pragma_directive(&mut context)?;
                    self.leave_pragma_directive(&mut context)?;
                }

                SourceUnitNode::ImportDirective(import_directive) => {
                    let mut context = context.create_import_directive_context(import_directive);
                    self.visit_import_directive(&mut context)?;
                    self.leave_import_directive(&mut context)?;
                }

                SourceUnitNode::ContractDefinition(contract_definition) => {
                    let mut context =
                        context.create_contract_definition_context(contract_definition);
                    self.visit_contract_definition(&mut context)?;
                    self.leave_contract_definition(&mut context)?;
                }

                SourceUnitNode::StructDefinition(struct_definition) => {
                    let mut context = context.create_struct_definition_context(struct_definition);
                    self.visit_struct_definition(&mut context)?;
                    self.leave_struct_definition(&mut context)?;
                }

                SourceUnitNode::EnumDefinition(enum_definition) => {
                    let mut context = context.create_enum_definition_context(enum_definition);
                    self.visit_enum_definition(&mut context)?;
                    self.leave_enum_definition(&mut context)?;
                }

                SourceUnitNode::ErrorDefinition(error_definition) => {
                    let mut context = context.create_error_definition_context(error_definition);
                    self.visit_error_definition(&mut context)?;
                    self.leave_error_definition(&mut context)?;
                }

                SourceUnitNode::VariableDeclaration(variable_declaration) => {
                    let mut context =
                        context.create_variable_declaration_context(variable_declaration);
                    self.visit_variable_declaration(&mut context)?;
                    self.leave_variable_declaration(&mut context)?;
                }

                SourceUnitNode::UserDefinedValueTypeDefinition(
                    user_defined_value_type_definition,
                ) => {
                    let mut context = context.create_user_defined_value_type_definition_context(
                        user_defined_value_type_definition,
                    );
                    self.visit_user_defined_value_type_definition(&mut context)?;
                    self.leave_user_defined_value_type_definition(&mut context)?;
                }
            }
        }

        Ok(())
    }

    fn leave_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_source_unit(context)?;
        }

        Ok(())
    }

    fn visit_pragma_directive<'a>(
        &mut self,
        context: &mut PragmaDirectiveContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_pragma_directive(context)?;
        }

        Ok(())
    }

    fn leave_pragma_directive<'a>(
        &mut self,
        context: &mut PragmaDirectiveContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_pragma_directive(context)?;
        }

        Ok(())
    }

    fn visit_import_directive<'a>(
        &mut self,
        context: &mut ImportDirectiveContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_import_directive(context)?;
        }

        Ok(())
    }

    fn leave_import_directive<'a>(
        &mut self,
        context: &mut ImportDirectiveContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_import_directive(context)?;
        }

        Ok(())
    }

    fn visit_struct_definition<'a>(
        &mut self,
        context: &mut StructDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_struct_definition(context)?;
        }

        Ok(())
    }

    fn leave_struct_definition<'a>(
        &mut self,
        context: &mut StructDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_struct_definition(context)?;
        }

        Ok(())
    }

    fn visit_enum_definition<'a>(
        &mut self,
        context: &mut EnumDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_enum_definition(context)?;
        }

        Ok(())
    }

    fn leave_enum_definition<'a>(
        &mut self,
        context: &mut EnumDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_enum_definition(context)?;
        }

        Ok(())
    }

    fn visit_contract_definition<'a>(
        &mut self,
        context: &mut ContractDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_contract_definition(context)?;
        }

        for definition_node in context.contract_definition.nodes.iter() {
            match definition_node {
                ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                    let mut context = context
                        .create_using_for_directive_context(definition_node, using_for_directive);
                    self.visit_using_for_directive(&mut context)?;
                    self.leave_using_for_directive(&mut context)?;
                }

                ContractDefinitionNode::StructDefinition(struct_definition) => {
                    let mut context = context.create_struct_definition_context(struct_definition);
                    self.visit_struct_definition(&mut context)?;
                    self.leave_struct_definition(&mut context)?;
                }

                ContractDefinitionNode::EnumDefinition(enum_definition) => {
                    let mut context = context.create_enum_definition_context(enum_definition);
                    self.visit_enum_definition(&mut context)?;
                    self.leave_enum_definition(&mut context)?;
                }

                ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                    let mut blocks = vec![];

                    let mut context = context.create_variable_declaration_context(
                        definition_node,
                        &mut blocks,
                        variable_declaration,
                    );

                    self.visit_variable_declaration(&mut context)?;
                    self.leave_variable_declaration(&mut context)?;
                }

                ContractDefinitionNode::EventDefinition(event_definition) => {
                    let mut context = context.create_event_definition_context(event_definition);
                    self.visit_event_definition(&mut context)?;
                    self.leave_event_definition(&mut context)?;
                }

                ContractDefinitionNode::ErrorDefinition(error_definition) => {
                    let mut context = context.create_error_definition_context(error_definition);
                    self.visit_error_definition(&mut context)?;
                    self.leave_error_definition(&mut context)?;
                }

                ContractDefinitionNode::FunctionDefinition(function_definition) => {
                    let mut context = context
                        .create_function_definition_context(definition_node, function_definition);
                    self.visit_function_definition(&mut context)?;
                    self.leave_function_definition(&mut context)?;
                }

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                    let mut context = context
                        .create_modifier_definition_context(definition_node, modifier_definition);
                    self.visit_modifier_definition(&mut context)?;
                    self.leave_modifier_definition(&mut context)?;
                }

                ContractDefinitionNode::UserDefinedValueTypeDefinition(
                    user_defined_value_type_definition,
                ) => {
                    let mut context = context.create_user_defined_value_type_definition_context(
                        user_defined_value_type_definition,
                    );
                    self.visit_user_defined_value_type_definition(&mut context)?;
                    self.leave_user_defined_value_type_definition(&mut context)?;
                }
            }
        }

        Ok(())
    }

    fn leave_contract_definition<'a>(
        &mut self,
        context: &mut ContractDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_contract_definition(context)?;
        }

        Ok(())
    }

    fn visit_using_for_directive<'a>(
        &mut self,
        context: &mut UsingForDirectiveContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_using_for_directive(context)?;
        }

        Ok(())
    }

    fn leave_using_for_directive<'a>(
        &mut self,
        context: &mut UsingForDirectiveContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_using_for_directive(context)?;
        }

        Ok(())
    }

    fn visit_variable_declaration<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_variable_declaration(context)?;
        }

        Ok(())
    }

    fn leave_variable_declaration<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_variable_declaration(context)?;
        }

        Ok(())
    }

    fn visit_event_definition<'a>(
        &mut self,
        context: &mut EventDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_event_definition(context)?;
        }

        Ok(())
    }

    fn leave_event_definition<'a>(
        &mut self,
        context: &mut EventDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_event_definition(context)?;
        }

        Ok(())
    }

    fn visit_error_definition<'a>(
        &mut self,
        context: &mut ErrorDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_error_definition(context)?;
        }

        Ok(())
    }

    fn leave_error_definition<'a>(
        &mut self,
        context: &mut ErrorDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_error_definition(context)?;
        }

        Ok(())
    }

    fn visit_modifier_definition<'a>(
        &mut self,
        context: &mut ModifierDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_modifier_definition(context)?;
        }

        let mut blocks = vec![];
        let mut context =
            context.create_block_context(&context.modifier_definition.body, &mut blocks);

        self.visit_block(&mut context)?;
        self.leave_block(&mut context)?;

        Ok(())
    }

    fn leave_modifier_definition<'a>(
        &mut self,
        context: &mut ModifierDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_modifier_definition(context)?;
        }

        Ok(())
    }

    fn visit_function_definition<'a>(
        &mut self,
        context: &mut FunctionDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_function_definition(context)?;
        }

        for variable_declaration in context.function_definition.parameters.parameters.iter() {
            let mut context = VariableDeclarationContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: Some(context.contract_definition),
                definition_node: Some(context.definition_node),
                blocks: None,
                variable_declaration,
            };

            self.visit_variable_declaration(&mut context)?;
            self.leave_variable_declaration(&mut context)?;
        }

        for modifier_invocation in context.function_definition.modifiers.iter() {
            let mut modifier_context = ModifierInvocationContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                modifier_invocation,
            };

            self.visit_modifier_invocation(&mut modifier_context)?;
            self.visit_modifier_invocation(&mut modifier_context)?;
        }

        if let Some(block) = context.function_definition.body.as_ref() {
            let mut blocks = vec![];

            let mut context = BlockContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: &mut blocks,
                block,
            };

            self.visit_block(&mut context)?;
            self.leave_block(&mut context)?;
        }

        Ok(())
    }

    fn leave_function_definition<'a>(
        &mut self,
        context: &mut FunctionDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_function_definition(context)?;
        }

        Ok(())
    }

    fn visit_user_defined_value_type_definition<'a>(
        &mut self,
        context: &mut UserDefinedValueTypeDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_user_defined_value_type_definition(context)?;
        }

        Ok(())
    }

    fn leave_user_defined_value_type_definition<'a>(
        &mut self,
        context: &mut UserDefinedValueTypeDefinitionContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_user_defined_value_type_definition(context)?;
        }

        Ok(())
    }

    fn visit_modifier_invocation<'a>(
        &mut self,
        context: &mut ModifierInvocationContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_modifier_invocation(context)?;
        }

        if let Some(arguments) = context.modifier_invocation.arguments.as_ref() {
            for expression in arguments.iter() {
                let mut context = ExpressionContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: &mut vec![],
                    statement: None,
                    expression,
                };

                self.visit_expression(&mut context)?;
                self.leave_expression(&mut context)?;
            }
        }

        Ok(())
    }

    fn leave_modifier_invocation<'a>(
        &mut self,
        context: &mut ModifierInvocationContext<'a>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_modifier_invocation(context)?;
        }

        Ok(())
    }

    fn visit_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_block(context)?;
        }

        context.blocks.push(context.block);

        for statement in context.block.statements.iter() {
            let mut context = StatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement,
            };

            self.visit_statement(&mut context)?;
            self.leave_statement(&mut context)?;
        }

        Ok(())
    }

    fn leave_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        context.blocks.pop();

        for visitor in self.visitors.iter_mut() {
            visitor.leave_block(context)?;
        }

        Ok(())
    }

    fn visit_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_statement(context)?;
        }

        match context.statement {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                let mut context = VariableDeclarationStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    variable_declaration_statement,
                };

                self.visit_variable_declaration_statement(&mut context)?;
                self.leave_variable_declaration_statement(&mut context)?;
            }

            Statement::IfStatement(if_statement) => {
                let mut context = IfStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    if_statement,
                };

                self.visit_if_statement(&mut context)?;
                self.leave_if_statement(&mut context)?;
            }

            Statement::ForStatement(for_statement) => {
                let mut context = ForStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    for_statement,
                };

                self.visit_for_statement(&mut context)?;
                self.leave_for_statement(&mut context)?;
            }

            Statement::WhileStatement(while_statement) => {
                let mut context = WhileStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    while_statement,
                };

                self.visit_while_statement(&mut context)?;
                self.leave_while_statement(&mut context)?;
            }

            Statement::DoWhileStatement(do_while_statement) => {
                let mut context = DoWhileStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    do_while_statement,
                };

                self.visit_do_while_statement(&mut context)?;
                self.leave_do_while_statement(&mut context)?;
            }

            Statement::EmitStatement(emit_statement) => {
                let mut context = EmitStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    emit_statement,
                };

                self.visit_emit_statement(&mut context)?;
                self.leave_emit_statement(&mut context)?;
            }

            Statement::TryStatement(try_statement) => {
                let mut context = TryStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    try_statement,
                };

                self.visit_try_statement(&mut context)?;
                self.leave_try_statement(&mut context)?;
            }

            Statement::RevertStatement(revert_statement) => {
                let mut context = RevertStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    revert_statement,
                };

                self.visit_revert_statement(&mut context)?;
                self.leave_revert_statement(&mut context)?;
            }

            Statement::UncheckedBlock(block) => {
                let mut context = BlockContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    block,
                };

                self.visit_block(&mut context)?;
                self.leave_block(&mut context)?;
            }

            Statement::Return(return_statement) => {
                let mut context = ReturnContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: Some(context.statement),
                    return_statement,
                };

                self.visit_return(&mut context)?;
                self.leave_return(&mut context)?;
            }

            Statement::ExpressionStatement(expression_statement) => {
                let mut context = ExpressionContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: Some(context.statement),
                    expression: &expression_statement.expression,
                };

                self.visit_expression(&mut context)?;
                self.leave_expression(&mut context)?;
            }

            Statement::Block(block) => {
                let mut context = BlockContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    block,
                };

                self.visit_block(&mut context)?;
                self.leave_block(&mut context)?;
            }

            Statement::InlineAssembly(inline_assembly) => {
                let mut context = InlineAssemblyContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly,
                };

                self.visit_inline_assembly(&mut context)?;
                self.leave_inline_assembly(&mut context)?;
            }

            Statement::Continue { .. } => {
                self.visit_continue_statement(context)?;
                self.leave_continue_statement(context)?;
            }

            Statement::Break { .. } => {
                self.visit_break_statement(context)?;
                self.leave_break_statement(context)?;
            }

            Statement::PlaceholderStatement { .. } => {
                self.visit_placeholder_statement(context)?;
                self.leave_placeholder_statement(context)?;
            }
        }

        Ok(())
    }

    fn leave_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_statement(context)?;
        }

        Ok(())
    }

    fn visit_variable_declaration_statement<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_variable_declaration_statement(context)?;
        }

        for variable_declaration in context
            .variable_declaration_statement
            .declarations
            .iter()
            .flatten()
        {
            let mut context = VariableDeclarationContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: Some(context.contract_definition),
                definition_node: Some(context.definition_node),
                blocks: Some(context.blocks),
                variable_declaration,
            };

            self.visit_variable_declaration(&mut context)?;
            self.leave_variable_declaration(&mut context)?;
        }

        if let Some(initial_value) = context
            .variable_declaration_statement
            .initial_value
            .as_ref()
        {
            let mut context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: None,
                expression: initial_value,
            };

            self.visit_expression(&mut context)?;
            self.leave_expression(&mut context)?;
        }

        Ok(())
    }

    fn leave_variable_declaration_statement<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_variable_declaration_statement(context)?;
        }

        Ok(())
    }

    fn visit_if_statement<'a, 'b>(
        &mut self,
        context: &mut IfStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_if_statement(context)?;
        }

        let mut true_body_context = BlockOrStatementContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            block_or_statement: &context.if_statement.true_body,
        };

        self.visit_block_or_statement(&mut true_body_context)?;
        self.leave_block_or_statement(&mut true_body_context)?;

        if let Some(false_body) = context.if_statement.false_body.as_ref() {
            let mut false_body_context = BlockOrStatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                block_or_statement: false_body,
            };

            self.visit_block_or_statement(&mut false_body_context)?;
            self.leave_block_or_statement(&mut false_body_context)?;
        }

        Ok(())
    }

    fn leave_if_statement<'a, 'b>(
        &mut self,
        context: &mut IfStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_if_statement(context)?;
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(
        &mut self,
        context: &mut ForStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_for_statement(context)?;
        }

        if let Some(statement) = context.for_statement.initialization_expression.as_ref() {
            let mut context = StatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement,
            };

            self.visit_statement(&mut context)?;
            self.leave_statement(&mut context)?;
        }

        if let Some(expression) = context.for_statement.condition.as_ref() {
            let mut context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: None,
                expression,
            };

            self.visit_expression(&mut context)?;
            self.leave_expression(&mut context)?;
        }

        if let Some(statement) = context.for_statement.loop_expression.as_ref() {
            let mut context = StatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement,
            };

            self.visit_statement(&mut context)?;
            self.leave_statement(&mut context)?;
        }

        let mut context = BlockOrStatementContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            block_or_statement: &context.for_statement.body,
        };

        self.visit_block_or_statement(&mut context)?;
        self.leave_block_or_statement(&mut context)?;

        Ok(())
    }

    fn leave_for_statement<'a, 'b>(
        &mut self,
        context: &mut ForStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_for_statement(context)?;
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(
        &mut self,
        context: &mut WhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_while_statement(context)?;
        }

        let mut condition_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: None,
            expression: &context.while_statement.condition,
        };

        self.visit_expression(&mut condition_context)?;
        self.leave_expression(&mut condition_context)?;

        let mut context = BlockOrStatementContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            block_or_statement: &context.while_statement.body,
        };

        self.visit_block_or_statement(&mut context)?;
        self.leave_block_or_statement(&mut context)?;

        Ok(())
    }

    fn leave_while_statement<'a, 'b>(
        &mut self,
        context: &mut WhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_while_statement(context)?;
        }

        Ok(())
    }

    fn visit_do_while_statement<'a, 'b>(
        &mut self,
        context: &mut DoWhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_do_while_statement(context)?;
        }

        let mut body_context = BlockOrStatementContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            block_or_statement: &context.do_while_statement.body,
        };

        self.visit_block_or_statement(&mut body_context)?;
        self.leave_block_or_statement(&mut body_context)?;

        let mut condition_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: None,
            expression: &context.do_while_statement.condition,
        };

        self.visit_expression(&mut condition_context)?;
        self.leave_expression(&mut condition_context)?;

        Ok(())
    }

    fn leave_do_while_statement<'a, 'b>(
        &mut self,
        context: &mut DoWhileStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_do_while_statement(context)?;
        }

        Ok(())
    }

    fn visit_emit_statement<'a, 'b>(
        &mut self,
        context: &mut EmitStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_emit_statement(context)?;
        }

        Ok(())
    }

    fn leave_emit_statement<'a, 'b>(
        &mut self,
        context: &mut EmitStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_emit_statement(context)?;
        }

        Ok(())
    }

    fn visit_try_statement<'a, 'b>(
        &mut self,
        context: &mut TryStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_try_statement(context)?;
        }

        for clause in context.try_statement.clauses.iter() {
            let mut context = BlockContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                block: &clause.block,
            };

            self.visit_block(&mut context)?;
            self.leave_block(&mut context)?;
        }

        Ok(())
    }

    fn leave_try_statement<'a, 'b>(
        &mut self,
        context: &mut TryStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_try_statement(context)?;
        }

        Ok(())
    }

    fn visit_revert_statement<'a, 'b>(
        &mut self,
        context: &mut RevertStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_revert_statement(context)?;
        }

        Ok(())
    }

    fn leave_revert_statement<'a, 'b>(
        &mut self,
        context: &mut RevertStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_revert_statement(context)?;
        }

        Ok(())
    }

    fn visit_continue_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_continue_statement(context)?;
        }

        Ok(())
    }

    fn leave_continue_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_continue_statement(context)?;
        }

        Ok(())
    }

    fn visit_break_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_break_statement(context)?;
        }

        Ok(())
    }

    fn leave_break_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_break_statement(context)?;
        }

        Ok(())
    }

    fn visit_placeholder_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_placeholder_statement(context)?;
        }

        Ok(())
    }

    fn leave_placeholder_statement<'a, 'b>(
        &mut self,
        context: &mut StatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_placeholder_statement(context)?;
        }

        Ok(())
    }

    fn visit_block_or_statement<'a, 'b>(
        &mut self,
        context: &mut BlockOrStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_block_or_statement(context)?;
        }

        match context.block_or_statement {
            BlockOrStatement::Block(block) => {
                let mut context = BlockContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    block,
                };

                self.visit_block(&mut context)?;
                self.leave_block(&mut context)?;
            }

            BlockOrStatement::Statement(statement) => {
                let mut context = StatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement,
                };

                self.visit_statement(&mut context)?;
                self.leave_statement(&mut context)?;
            }
        }

        Ok(())
    }

    fn leave_block_or_statement<'a, 'b>(
        &mut self,
        context: &mut BlockOrStatementContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_block_or_statement(context)?;
        }

        Ok(())
    }

    fn visit_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_return(context)?;
        }

        if let Some(expression) = context.return_statement.expression.as_ref() {
            let mut condition_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression,
            };

            self.visit_expression(&mut condition_context)?;
            self.leave_expression(&mut condition_context)?;
        }

        Ok(())
    }

    fn visit_expression<'a, 'b>(
        &mut self,
        context: &mut ExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_expression(context)?;
        }

        match context.expression {
            Expression::Literal(literal) => {
                let mut context = LiteralContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    literal,
                };

                self.visit_literal(&mut context)?;
                self.leave_literal(&mut context)?;
            }

            Expression::Identifier(identifier) => {
                let mut context = IdentifierContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    identifier,
                };

                self.visit_identifier(&mut context)?;
                self.leave_identifier(&mut context)?;
            }

            Expression::UnaryOperation(unary_operation) => {
                let mut context = UnaryOperationContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    unary_operation,
                };

                self.visit_unary_operation(&mut context)?;
                self.leave_unary_operation(&mut context)?;
            }

            Expression::BinaryOperation(binary_operation) => {
                let mut context = BinaryOperationContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    binary_operation,
                };

                self.visit_binary_operation(&mut context)?;
                self.leave_binary_operation(&mut context)?;
            }

            Expression::Conditional(conditional) => {
                let mut context = ConditionalContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    conditional,
                };

                self.visit_conditional(&mut context)?;
                self.leave_conditional(&mut context)?;
            }

            Expression::Assignment(assignment) => {
                let mut context = AssignmentContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    assignment,
                };

                self.visit_assignment(&mut context)?;
                self.leave_assignment(&mut context)?;
            }

            Expression::FunctionCall(function_call) => {
                let mut context = FunctionCallContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    function_call,
                };

                self.visit_function_call(&mut context)?;
                self.leave_function_call(&mut context)?;
            }

            Expression::FunctionCallOptions(function_call_options) => {
                let mut context = FunctionCallOptionsContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    function_call_options,
                };

                self.visit_function_call_options(&mut context)?;
                self.leave_function_call_options(&mut context)?;
            }

            Expression::IndexAccess(index_access) => {
                let mut context = IndexAccessContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    index_access,
                };

                self.visit_index_access(&mut context)?;
                self.leave_index_access(&mut context)?;
            }

            Expression::IndexRangeAccess(index_range_access) => {
                let mut context = IndexRangeAccessContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    index_range_access,
                };

                self.visit_index_range_access(&mut context)?;
                self.leave_index_range_access(&mut context)?;
            }

            Expression::MemberAccess(member_access) => {
                let mut context = MemberAccessContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    member_access,
                };

                self.visit_member_access(&mut context)?;
                self.leave_member_access(&mut context)?;
            }

            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                let mut context = ElementaryTypeNameExpressionContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    elementary_type_name_expression,
                };

                self.visit_elementary_type_name_expression(&mut context)?;
                self.leave_elementary_type_name_expression(&mut context)?;
            }

            Expression::TupleExpression(tuple_expression) => {
                let mut context = TupleExpressionContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    tuple_expression,
                };

                self.visit_tuple_expression(&mut context)?;
                self.leave_tuple_expression(&mut context)?;
            }

            Expression::NewExpression(new_expression) => {
                let mut context = NewExpressionContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    new_expression,
                };

                self.visit_new_expression(&mut context)?;
                self.leave_new_expression(&mut context)?;
            }
        }

        Ok(())
    }

    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_literal(context)?;
        }

        Ok(())
    }

    fn leave_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_literal(context)?;
        }

        Ok(())
    }

    fn visit_identifier<'a, 'b>(
        &mut self,
        context: &mut IdentifierContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_identifier(context)?;
        }

        Ok(())
    }

    fn leave_identifier<'a, 'b>(
        &mut self,
        context: &mut IdentifierContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_identifier(context)?;
        }

        Ok(())
    }

    fn visit_unary_operation<'a, 'b>(
        &mut self,
        context: &mut UnaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_unary_operation(context)?;
        }

        let mut sub_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.unary_operation.sub_expression.as_ref(),
        };

        self.visit_expression(&mut sub_context)?;
        self.leave_expression(&mut sub_context)?;

        Ok(())
    }

    fn leave_unary_operation<'a, 'b>(
        &mut self,
        context: &mut UnaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_unary_operation(context)?;
        }

        Ok(())
    }

    fn visit_binary_operation<'a, 'b>(
        &mut self,
        context: &mut BinaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_binary_operation(context)?;
        }

        let mut left_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.binary_operation.left_expression.as_ref(),
        };

        self.visit_expression(&mut left_context)?;
        self.leave_expression(&mut left_context)?;

        let mut right_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.binary_operation.right_expression.as_ref(),
        };

        self.visit_expression(&mut right_context)?;
        self.leave_expression(&mut right_context)?;

        Ok(())
    }

    fn leave_binary_operation<'a, 'b>(
        &mut self,
        context: &mut BinaryOperationContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_binary_operation(context)?;
        }

        Ok(())
    }

    fn visit_conditional<'a, 'b>(
        &mut self,
        context: &mut ConditionalContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_conditional(context)?;
        }

        let mut condition_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.conditional.condition.as_ref(),
        };

        self.visit_expression(&mut condition_context)?;
        self.leave_expression(&mut condition_context)?;

        let mut true_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.conditional.true_expression.as_ref(),
        };

        self.visit_expression(&mut true_context)?;
        self.leave_expression(&mut true_context)?;

        let mut false_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.conditional.false_expression.as_ref(),
        };

        self.visit_expression(&mut false_context)?;
        self.leave_expression(&mut false_context)?;

        Ok(())
    }

    fn leave_conditional<'a, 'b>(
        &mut self,
        context: &mut ConditionalContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_conditional(context)?;
        }

        Ok(())
    }

    fn visit_assignment<'a, 'b>(
        &mut self,
        context: &mut AssignmentContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_assignment(context)?;
        }

        let mut left_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.assignment.left_hand_side.as_ref(),
        };

        self.visit_expression(&mut left_context)?;
        self.leave_expression(&mut left_context)?;

        let mut right_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.assignment.right_hand_side.as_ref(),
        };

        self.visit_expression(&mut right_context)?;
        self.leave_expression(&mut right_context)?;

        Ok(())
    }

    fn leave_assignment<'a, 'b>(
        &mut self,
        context: &mut AssignmentContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_assignment(context)?;
        }

        Ok(())
    }

    fn visit_function_call<'a, 'b>(
        &mut self,
        context: &mut FunctionCallContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_function_call(context)?;
        }

        let mut expression_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.function_call.expression.as_ref(),
        };

        self.visit_expression(&mut expression_context)?;
        self.leave_expression(&mut expression_context)?;

        for argument in context.function_call.arguments.iter() {
            let mut argument_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression: argument,
            };

            self.visit_expression(&mut argument_context)?;
            self.leave_expression(&mut argument_context)?;
        }

        Ok(())
    }

    fn leave_function_call<'a, 'b>(
        &mut self,
        context: &mut FunctionCallContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_function_call(context)?;
        }

        Ok(())
    }

    fn visit_function_call_options<'a, 'b>(
        &mut self,
        context: &mut FunctionCallOptionsContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_function_call_options(context)?;
        }

        let mut expression_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.function_call_options.expression.as_ref(),
        };

        self.visit_expression(&mut expression_context)?;
        self.leave_expression(&mut expression_context)?;

        for option in context.function_call_options.options.iter() {
            let mut option_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression: option,
            };

            self.visit_expression(&mut option_context)?;
            self.leave_expression(&mut option_context)?;
        }

        Ok(())
    }

    fn leave_function_call_options<'a, 'b>(
        &mut self,
        context: &mut FunctionCallOptionsContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_function_call_options(context)?;
        }

        Ok(())
    }

    fn visit_index_access<'a, 'b>(
        &mut self,
        context: &mut IndexAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_index_access(context)?;
        }

        let mut base_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.index_access.base_expression.as_ref(),
        };

        self.visit_expression(&mut base_context)?;
        self.leave_expression(&mut base_context)?;

        if let Some(expression) = context.index_access.index_expression.as_ref() {
            let mut index_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression,
            };

            self.visit_expression(&mut index_context)?;
            self.leave_expression(&mut index_context)?;
        }

        Ok(())
    }

    fn leave_index_access<'a, 'b>(
        &mut self,
        context: &mut IndexAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_index_access(context)?;
        }

        Ok(())
    }

    fn visit_index_range_access<'a, 'b>(
        &mut self,
        context: &mut IndexRangeAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_index_range_access(context)?;
        }

        let mut base_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.index_range_access.base_expression.as_ref(),
        };

        self.visit_expression(&mut base_context)?;
        self.leave_expression(&mut base_context)?;

        if let Some(start_expression) = context.index_range_access.start_expression.as_ref() {
            let mut start_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression: start_expression.as_ref(),
            };

            self.visit_expression(&mut start_context)?;
            self.leave_expression(&mut start_context)?;
        }

        if let Some(end_expression) = context.index_range_access.end_expression.as_ref() {
            let mut end_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression: end_expression.as_ref(),
            };

            self.visit_expression(&mut end_context)?;
            self.leave_expression(&mut end_context)?;
        }

        Ok(())
    }

    fn leave_index_range_access<'a, 'b>(
        &mut self,
        context: &mut IndexRangeAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_index_range_access(context)?;
        }

        Ok(())
    }

    fn visit_member_access<'a, 'b>(
        &mut self,
        context: &mut MemberAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_member_access(context)?;
        }

        let mut expression_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.member_access.expression.as_ref(),
        };

        self.visit_expression(&mut expression_context)?;
        self.leave_expression(&mut expression_context)?;

        Ok(())
    }

    fn leave_member_access<'a, 'b>(
        &mut self,
        context: &mut MemberAccessContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_member_access(context)?;
        }

        Ok(())
    }

    fn visit_elementary_type_name_expression<'a, 'b>(
        &mut self,
        context: &mut ElementaryTypeNameExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_elementary_type_name_expression(context)?;
        }

        Ok(())
    }

    fn leave_elementary_type_name_expression<'a, 'b>(
        &mut self,
        context: &mut ElementaryTypeNameExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_elementary_type_name_expression(context)?;
        }

        Ok(())
    }

    fn visit_tuple_expression<'a, 'b>(
        &mut self,
        context: &mut TupleExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_tuple_expression(context)?;
        }

        for component in context.tuple_expression.components.iter().flatten() {
            let mut component_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression: component,
            };

            self.visit_expression(&mut component_context)?;
            self.leave_expression(&mut component_context)?;
        }

        Ok(())
    }

    fn leave_tuple_expression<'a, 'b>(
        &mut self,
        context: &mut TupleExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_tuple_expression(context)?;
        }

        Ok(())
    }

    fn visit_new_expression<'a, 'b>(
        &mut self,
        context: &mut NewExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_new_expression(context)?;
        }

        Ok(())
    }

    fn leave_new_expression<'a, 'b>(
        &mut self,
        context: &mut NewExpressionContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_new_expression(context)?;
        }

        Ok(())
    }

    fn visit_inline_assembly<'a, 'b>(
        &mut self,
        context: &mut InlineAssemblyContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_inline_assembly(context)?;
        }

        if let Some(yul_block) = context.inline_assembly.ast.as_ref() {
            let mut yul_blocks = vec![];

            let mut context = YulBlockContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                inline_assembly: context.inline_assembly,
                yul_blocks: &mut yul_blocks,
                yul_block,
            };

            self.visit_yul_block(&mut context)?;
            self.leave_yul_block(&mut context)?;
        }

        Ok(())
    }

    fn leave_inline_assembly<'a, 'b>(
        &mut self,
        context: &mut InlineAssemblyContext<'a, 'b>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_inline_assembly(context)?;
        }

        Ok(())
    }

    fn visit_yul_block<'a, 'b, 'c>(
        &mut self,
        context: &mut YulBlockContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_block(context)?;
        }

        context.yul_blocks.push(context.yul_block);

        for yul_statement in context.yul_block.statements.iter() {
            let mut context = YulStatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                inline_assembly: context.inline_assembly,
                yul_blocks: context.yul_blocks,
                yul_statement,
            };

            self.visit_yul_statement(&mut context)?;
            self.leave_yul_statement(&mut context)?;
        }

        Ok(())
    }

    fn leave_yul_block<'a, 'b, 'c>(
        &mut self,
        context: &mut YulBlockContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        context.yul_blocks.pop();

        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_block(context)?;
        }

        Ok(())
    }

    fn visit_yul_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_statement(context)?;
        }

        match context.yul_statement {
            YulStatement::YulIf(yul_if) => {
                let mut context = YulIfContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_if,
                };

                self.visit_yul_if(&mut context)?;
                self.leave_yul_if(&mut context)?;
            }

            YulStatement::YulSwitch(yul_switch) => {
                let mut context = YulSwitchContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_switch,
                };

                self.visit_yul_switch(&mut context)?;
                self.leave_yul_switch(&mut context)?;
            }

            YulStatement::YulForLoop(yul_for_loop) => {
                let mut context = YulForLoopContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_for_loop,
                };

                self.visit_yul_for_loop(&mut context)?;
                self.leave_yul_for_loop(&mut context)?;
            }

            YulStatement::YulAssignment(yul_assignment) => {
                let mut context = YulAssignmentContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_assignment,
                };

                self.visit_yul_assignment(&mut context)?;
                self.leave_yul_assignment(&mut context)?;
            }

            YulStatement::YulVariableDeclaration(yul_variable_declaration) => {
                let mut context = YulVariableDeclarationContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_variable_declaration,
                };

                self.visit_yul_variable_declaration(&mut context)?;
                self.leave_yul_variable_declaration(&mut context)?;
            }

            YulStatement::YulExpressionStatement(yul_expression_statement) => {
                let mut context = YulExpressionStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_expression_statement,
                };

                self.visit_yul_expression_statement(&mut context)?;
                self.leave_yul_expression_statement(&mut context)?;
            }

            YulStatement::YulFunctionDefinition(yul_function_definition) => {
                let mut context = YulFunctionDefinitionContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_function_definition,
                };

                self.visit_yul_function_definition(&mut context)?;
                self.leave_yul_function_definition(&mut context)?;
            }

            YulStatement::YulBlock(yul_block) => {
                let mut context = YulBlockContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_block,
                };

                self.visit_yul_block(&mut context)?;
                self.leave_yul_block(&mut context)?;
            }

            YulStatement::YulLeave => {
                self.visit_yul_leave(context)?;
                self.leave_yul_leave(context)?;
            }

            YulStatement::YulBreak => {
                self.visit_yul_break(context)?;
                self.leave_yul_break(context)?;
            }

            YulStatement::YulContinue => {
                self.visit_yul_continue(context)?;
                self.leave_yul_continue(context)?;
            }
        }

        Ok(())
    }

    fn leave_yul_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_statement(context)?;
        }

        Ok(())
    }

    fn visit_yul_if<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIfContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_if(context)?;
        }

        let mut condition_context = YulExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_statement: Some(context.yul_statement),
            yul_expression: &context.yul_if.condition,
        };

        self.visit_yul_expression(&mut condition_context)?;
        self.leave_yul_expression(&mut condition_context)?;

        let mut body_context = YulBlockContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_block: &context.yul_if.body,
        };

        self.visit_yul_block(&mut body_context)?;
        self.leave_yul_block(&mut body_context)?;

        Ok(())
    }

    fn leave_yul_if<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIfContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_if(context)?;
        }

        Ok(())
    }

    fn visit_yul_switch<'a, 'b, 'c>(
        &mut self,
        context: &mut YulSwitchContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_switch(context)?;
        }

        let mut expression_context = YulExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_statement: Some(context.yul_statement),
            yul_expression: &context.yul_switch.expression,
        };

        self.visit_yul_expression(&mut expression_context)?;
        self.leave_yul_expression(&mut expression_context)?;

        for yul_case in context.yul_switch.cases.iter() {
            let mut case_context = YulCaseContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                inline_assembly: context.inline_assembly,
                yul_blocks: context.yul_blocks,
                yul_statement: context.yul_statement,
                yul_switch: context.yul_switch,
                yul_case,
            };

            self.visit_yul_case(&mut case_context)?;
            self.leave_yul_case(&mut case_context)?;
        }

        Ok(())
    }

    fn leave_yul_switch<'a, 'b, 'c>(
        &mut self,
        context: &mut YulSwitchContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_switch(context)?;
        }

        Ok(())
    }

    fn visit_yul_for_loop<'a, 'b, 'c>(
        &mut self,
        context: &mut YulForLoopContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_for_loop(context)?;
        }

        let mut pre_context = YulBlockContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_block: &context.yul_for_loop.pre,
        };

        self.visit_yul_block(&mut pre_context)?;
        self.leave_yul_block(&mut pre_context)?;

        let mut condition_context = YulExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_statement: Some(context.yul_statement),
            yul_expression: &context.yul_for_loop.condition,
        };

        self.visit_yul_expression(&mut condition_context)?;
        self.leave_yul_expression(&mut condition_context)?;

        let mut post_context = YulBlockContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_block: &context.yul_for_loop.post,
        };

        self.visit_yul_block(&mut post_context)?;
        self.leave_yul_block(&mut post_context)?;

        let mut body_context = YulBlockContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_block: &context.yul_for_loop.body,
        };

        self.visit_yul_block(&mut body_context)?;
        self.leave_yul_block(&mut body_context)?;

        Ok(())
    }

    fn leave_yul_for_loop<'a, 'b, 'c>(
        &mut self,
        context: &mut YulForLoopContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_for_loop(context)?;
        }

        Ok(())
    }

    fn visit_yul_case<'a, 'b, 'c>(
        &mut self,
        context: &mut YulCaseContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_case(context)?;
        }

        if let Some(value) = context.yul_case.value.as_ref() {
            let mut value_context = YulExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                inline_assembly: context.inline_assembly,
                yul_blocks: context.yul_blocks,
                yul_statement: Some(context.yul_statement),
                yul_expression: value,
            };

            self.visit_yul_expression(&mut value_context)?;
            self.leave_yul_expression(&mut value_context)?;
        }

        let mut body_context = YulBlockContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_block: &context.yul_case.body,
        };

        self.visit_yul_block(&mut body_context)?;
        self.leave_yul_block(&mut body_context)?;

        Ok(())
    }

    fn leave_yul_case<'a, 'b, 'c>(
        &mut self,
        context: &mut YulCaseContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_case(context)?;
        }

        Ok(())
    }

    fn visit_yul_assignment<'a, 'b, 'c>(
        &mut self,
        context: &mut YulAssignmentContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_assignment(context)?;
        }

        for yul_identifier in context.yul_assignment.variable_names.iter() {
            let mut context = YulIdentifierContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                inline_assembly: context.inline_assembly,
                yul_blocks: context.yul_blocks,
                yul_statement: Some(context.yul_statement),
                yul_expression: None,
                yul_identifier,
            };

            self.visit_yul_identifier(&mut context)?;
            self.leave_yul_identifier(&mut context)?;
        }

        let mut value_context = YulExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_statement: Some(context.yul_statement),
            yul_expression: &context.yul_assignment.value,
        };

        self.visit_yul_expression(&mut value_context)?;
        self.leave_yul_expression(&mut value_context)?;

        Ok(())
    }

    fn leave_yul_assignment<'a, 'b, 'c>(
        &mut self,
        context: &mut YulAssignmentContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_assignment(context)?;
        }

        Ok(())
    }

    fn visit_yul_variable_declaration<'a, 'b, 'c>(
        &mut self,
        context: &mut YulVariableDeclarationContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_variable_declaration(context)?;
        }

        if let Some(value) = context.yul_variable_declaration.value.as_ref() {
            let mut value_context = YulExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                inline_assembly: context.inline_assembly,
                yul_blocks: context.yul_blocks,
                yul_statement: Some(context.yul_statement),
                yul_expression: value,
            };

            self.visit_yul_expression(&mut value_context)?;
            self.leave_yul_expression(&mut value_context)?;
        }

        Ok(())
    }

    fn leave_yul_variable_declaration<'a, 'b, 'c>(
        &mut self,
        context: &mut YulVariableDeclarationContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_variable_declaration(context)?;
        }

        Ok(())
    }

    fn visit_yul_typed_name<'a, 'b, 'c>(
        &mut self,
        context: &mut YulTypedNameContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_typed_name(context)?;
        }

        Ok(())
    }

    fn leave_yul_typed_name<'a, 'b, 'c>(
        &mut self,
        context: &mut YulTypedNameContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_typed_name(context)?;
        }

        Ok(())
    }

    fn visit_yul_expression_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_expression_statement(context)?;
        }

        let mut expression_context = YulExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_statement: Some(context.yul_statement),
            yul_expression: &context.yul_expression_statement.expression,
        };

        self.visit_yul_expression(&mut expression_context)?;
        self.leave_yul_expression(&mut expression_context)?;

        Ok(())
    }

    fn leave_yul_expression_statement<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_expression_statement(context)?;
        }

        Ok(())
    }

    fn visit_yul_function_definition<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionDefinitionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_function_definition(context)?;
        }

        if let Some(parameters) = context.yul_function_definition.parameters.as_ref() {
            for parameter in parameters.iter() {
                let mut context = YulTypedNameContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: Some(context.yul_statement),
                    yul_typed_name: parameter,
                };

                self.visit_yul_typed_name(&mut context)?;
                self.leave_yul_typed_name(&mut context)?;
            }
        }

        if let Some(return_parameters) = context.yul_function_definition.return_parameters.as_ref()
        {
            for parameter in return_parameters.iter() {
                let mut context = YulTypedNameContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: Some(context.yul_statement),
                    yul_typed_name: parameter,
                };

                self.visit_yul_typed_name(&mut context)?;
                self.leave_yul_typed_name(&mut context)?;
            }
        }

        let mut context = YulBlockContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_block: &context.yul_function_definition.body,
        };

        self.visit_yul_block(&mut context)?;
        self.leave_yul_block(&mut context)?;

        Ok(())
    }

    fn leave_yul_function_definition<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionDefinitionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_function_definition(context)?;
        }

        Ok(())
    }

    fn visit_yul_leave<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_leave(context)?;
        }

        Ok(())
    }

    fn leave_yul_leave<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_leave(context)?;
        }

        Ok(())
    }

    fn visit_yul_break<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_break(context)?;
        }

        Ok(())
    }

    fn leave_yul_break<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_break(context)?;
        }

        Ok(())
    }

    fn visit_yul_continue<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_continue(context)?;
        }

        Ok(())
    }

    fn leave_yul_continue<'a, 'b, 'c>(
        &mut self,
        context: &mut YulStatementContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_continue(context)?;
        }

        Ok(())
    }

    fn visit_yul_expression<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_expression(context)?;
        }

        match context.yul_expression {
            YulExpression::YulLiteral(yul_literal) => {
                let mut literal_context = YulLiteralContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_expression: context.yul_expression,
                    yul_literal,
                };

                self.visit_yul_literal(&mut literal_context)?;
                self.leave_yul_literal(&mut literal_context)?;
            }

            YulExpression::YulIdentifier(yul_identifier) => {
                let mut identifier_context = YulIdentifierContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_expression: Some(context.yul_expression),
                    yul_identifier,
                };

                self.visit_yul_identifier(&mut identifier_context)?;
                self.leave_yul_identifier(&mut identifier_context)?;
            }

            YulExpression::YulFunctionCall(yul_function_call) => {
                let mut function_call_context = YulFunctionCallContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    inline_assembly: context.inline_assembly,
                    yul_blocks: context.yul_blocks,
                    yul_statement: context.yul_statement,
                    yul_expression: context.yul_expression,
                    yul_function_call,
                };

                self.visit_yul_function_call(&mut function_call_context)?;
                self.leave_yul_function_call(&mut function_call_context)?;
            }
        }

        Ok(())
    }

    fn leave_yul_expression<'a, 'b, 'c>(
        &mut self,
        context: &mut YulExpressionContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_expression(context)?;
        }

        Ok(())
    }

    fn visit_yul_literal<'a, 'b, 'c>(
        &mut self,
        context: &mut YulLiteralContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_literal(context)?;
        }

        Ok(())
    }

    fn leave_yul_literal<'a, 'b, 'c>(
        &mut self,
        context: &mut YulLiteralContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_literal(context)?;
        }

        Ok(())
    }

    fn visit_yul_identifier<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIdentifierContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_identifier(context)?;
        }

        Ok(())
    }

    fn leave_yul_identifier<'a, 'b, 'c>(
        &mut self,
        context: &mut YulIdentifierContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_identifier(context)?;
        }

        Ok(())
    }

    fn visit_yul_function_call<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionCallContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_function_call(context)?;
        }

        let mut identifier_context = YulIdentifierContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            inline_assembly: context.inline_assembly,
            yul_blocks: context.yul_blocks,
            yul_statement: context.yul_statement,
            yul_expression: Some(context.yul_expression),
            yul_identifier: &context.yul_function_call.function_name,
        };

        self.visit_yul_identifier(&mut identifier_context)?;
        self.leave_yul_identifier(&mut identifier_context)?;

        for yul_expression in context.yul_function_call.arguments.iter() {
            let mut expression_context = YulExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                inline_assembly: context.inline_assembly,
                yul_blocks: context.yul_blocks,
                yul_statement: context.yul_statement,
                yul_expression,
            };

            self.visit_yul_expression(&mut expression_context)?;
            self.leave_yul_expression(&mut expression_context)?;
        }

        Ok(())
    }

    fn leave_yul_function_call<'a, 'b, 'c>(
        &mut self,
        context: &mut YulFunctionCallContext<'a, 'b, 'c>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_yul_function_call(context)?;
        }

        Ok(())
    }
}
