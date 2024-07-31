use std::collections::HashSet;

use crate::{
    ast::*,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

use super::macros::generate_extraction_library;

////////// PUBLICLY AVAILABLE EXTRACTION LIBRARY /////////////////////////

generate_extraction_library! {
    ExtractArrayTypeNames | visit_array_type_name => ArrayTypeName |,
    ExtractAssignments | visit_assignment => Assignment |,
    ExtractBinaryOperations | visit_binary_operation => BinaryOperation |,
    ExtractBlocks | visit_block => Block |,
    ExtractConditionals | visit_conditional => Conditional |,
    ExtractContractDefinitions | visit_contract_definition => ContractDefinition |,
    ExtractElementaryTypeNames | visit_elementary_type_name => ElementaryTypeName |,
    ExtractEmitStatements | visit_emit_statement => EmitStatement |,
    ExtractEnumDefinitions | visit_enum_definition => EnumDefinition |,
    ExtractEnumValues | visit_enum_value => EnumValue |,
    ExtractEventDefinitions | visit_event_definition => EventDefinition |,
    ExtractErrorDefinitions | visit_error_definition => ErrorDefinition |,
    ExtractExpressionStatements | visit_expression_statement => ExpressionStatement |,
    ExtractFunctionCalls | visit_function_call => FunctionCall |,
    ExtractFunctionCallOptions | visit_function_call_options => FunctionCallOptions |,
    ExtractFunctionDefinitions | visit_function_definition => FunctionDefinition |,
    ExtractFunctionTypeNames | visit_function_type_name => FunctionTypeName |,
    ExtractForStatements | visit_for_statement => ForStatement |,
    ExtractIdentifiers | visit_identifier => Identifier |,
    ExtractIdentifierPaths | visit_identifier_path => IdentifierPath |,
    ExtractIfStatements | visit_if_statement => IfStatement |,
    ExtractImportDirectives | visit_import_directive => ImportDirective |,
    ExtractIndexAccesses | visit_index_access => IndexAccess |,
    ExtractIndexRangeAccesses | visit_index_range_access => IndexRangeAccess |,
    ExtractInheritanceSpecifiers | visit_inheritance_specifier => InheritanceSpecifier |,
    ExtractInlineAssemblys | visit_inline_assembly => InlineAssembly |,
    ExtractLiterals | visit_literal => Literal |,
    ExtractMemberAccesses | visit_member_access => MemberAccess |,
    ExtractNewExpressions | visit_new_expression => NewExpression |,
    ExtractMappings | visit_mapping => Mapping |,
    ExtractModifierDefinitions | visit_modifier_definition => ModifierDefinition |,
    ExtractModifierInvocations | visit_modifier_invocation => ModifierInvocation |,
    ExtractOverrideSpecifiers | visit_override_specifier => OverrideSpecifier |,
    ExtractParameterLists | visit_parameter_list => ParameterList |,
    ExtractPragmaDirectives | visit_pragma_directive => PragmaDirective |,
    ExtractReturns | visit_return => Return |,
    ExtractRevertStatements | visit_revert_statement => RevertStatement |,
    ExtractStructDefinitions | visit_struct_definition => StructDefinition |,
    ExtractStructuredDocumentations | visit_structured_documentation => StructuredDocumentation |,
    ExtractTryStatements | visit_try_statement => TryStatement |,
    ExtractTryCatchClauses | visit_try_catch_clause => TryCatchClause |,
    ExtractTupleExpressions | visit_tuple_expression => TupleExpression |,
    ExtractUnaryOperations | visit_unary_operation => UnaryOperation |,
    ExtractUserDefinedTypeNames | visit_user_defined_type_name => UserDefinedTypeName |,
    ExtractUsingForDirectives | visit_using_for_directive => UsingForDirective |,
    ExtractVariableDeclarations | visit_variable_declaration => VariableDeclaration |,
    ExtractWhileStatements | visit_while_statement => WhileStatement |,
    ExtractDoWhileStatements | visit_do_while_statement => DoWhileStatement |,
    ExtractBreakStatements | visit_break_statement => Break |,
    ExtractContinueStatements | visit_continue_statement => Continue |,
    ExtractPlaceholderStatements | visit_placeholder_statement => PlaceholderStatement|,
}

/////////// EXTRACTION UTILS FOR CRATE - LEVEL ACCESS //////////////////

// ExtractImmediateChildren is an extractor that extracts immediate children from a node
#[derive(Default)]
pub(crate) struct ExtractImmediateChildrenIDs {
    pub extracted: Vec<NodeID>,
}

impl ExtractImmediateChildrenIDs {
    pub(crate) fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractImmediateChildrenIDs = Self::default();
        node.accept_metadata(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractImmediateChildrenIDs {
    fn visit_immediate_children(
        &mut self,
        _node_id: NodeID,
        node_children_ids: Vec<NodeID>,
    ) -> Result<()> {
        self.extracted.extend(node_children_ids);
        Ok(())
    }
}

// Extract Reference Declaration IDs
#[derive(Default)]
pub struct ExtractReferencedDeclarations {
    pub extracted: Vec<NodeID>,
}

impl ExtractReferencedDeclarations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractReferencedDeclarations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractReferencedDeclarations {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.extracted.push(node.referenced_declaration as i64);
        Ok(true)
    }
    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
}

// Extract Reference Declaration IDs
#[derive(Default)]
pub struct ExtractManipulatedStateVariablesIDs {
    pub deleted: HashSet<NodeID>,
    pub assigned: HashSet<NodeID>,
    pub pushed: HashSet<NodeID>,
    pub popped: HashSet<NodeID>,
}

impl ExtractManipulatedStateVariablesIDs {
    pub fn get_all_node_ids(&self) -> Vec<NodeID> {
        let mut all_nodes = [
            self.deleted.clone().into_iter().collect::<Vec<_>>(),
            self.assigned.clone().into_iter().collect::<Vec<_>>(),
            self.pushed.clone().into_iter().collect::<Vec<_>>(),
            self.popped.clone().into_iter().collect::<Vec<_>>(),
        ]
        .concat();
        // Some state variables can undergo more than 1 of the above operation.
        // Hence, we should deduplicate it
        all_nodes.dedup();
        all_nodes
    }

    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractManipulatedStateVariablesIDs = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractManipulatedStateVariablesIDs {
    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        // Catch delete operations
        if node.operator == "delete" {
            if let Some(id) = find_referenced_declaration_for_identifier_or_indexed_identifier(
                node.sub_expression.as_ref(),
            ) {
                self.deleted.insert(id);
            }
        }
        Ok(true)
    }

    fn visit_member_access(&mut self, member: &MemberAccess) -> Result<bool> {
        if let Some(id) = find_referenced_declaration_for_identifier_or_indexed_identifier(
            member.expression.as_ref(),
        ) {
            if member.member_name == "push" {
                self.pushed.insert(id);
            } else if member.member_name == "pop" {
                self.popped.insert(id);
            }
        }
        Ok(true)
    }

    fn visit_assignment(&mut self, assignment: &Assignment) -> Result<bool> {
        if let Some(id) = find_referenced_declaration_for_identifier_or_indexed_identifier(
            assignment.left_hand_side.as_ref(),
        ) {
            self.assigned.insert(id);
        }
        Ok(true)
    }
}

fn find_referenced_declaration_for_identifier_or_indexed_identifier(
    expr: &Expression,
) -> Option<NodeID> {
    match expr {
        Expression::Identifier(Identifier {
            referenced_declaration: Some(id),
            ..
        }) => {
            return Some(*id);
        }
        Expression::IndexAccess(IndexAccess {
            base_expression, ..
        }) => {
            return find_referenced_declaration_for_identifier_or_indexed_identifier(
                base_expression.as_ref(),
            );
        }
        Expression::MemberAccess(MemberAccess { expression, .. }) => {
            return find_referenced_declaration_for_identifier_or_indexed_identifier(
                expression.as_ref(),
            );
        }
        _ => (),
    };
    None
}

#[cfg(test)]
mod written_state_variables_tests {
    use crate::detect::test_utils::load_solidity_source_unit;

    use super::ExtractManipulatedStateVariablesIDs;

    #[test]
    fn has_variable_declarations() {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/StateVariablesWritten.sol");

        assert!(!context.variable_declarations().is_empty());
    }

    #[test]
    fn can_capture_deletes() {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/StateVariablesWritten.sol");

        let mut total_state_variables_deleted = 0;

        for contract in context.contract_definitions() {
            let state_variables_info = ExtractManipulatedStateVariablesIDs::from(contract);
            println!("{} - {}", contract.name, state_variables_info.deleted.len());
            println!("{:?}", state_variables_info.deleted);
            total_state_variables_deleted += state_variables_info.deleted.len();
        }

        assert_eq!(total_state_variables_deleted, 5);
    }

    #[test]
    fn can_capture_pushes() {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/StateVariablesWritten.sol");

        let mut total_state_variables_pushed_to = 0;

        for contract in context.contract_definitions() {
            let state_variables_info = ExtractManipulatedStateVariablesIDs::from(contract);
            println!("{} - {}", contract.name, state_variables_info.pushed.len());
            println!("{:?}", state_variables_info.pushed);
            total_state_variables_pushed_to += state_variables_info.pushed.len();
        }

        assert_eq!(total_state_variables_pushed_to, 2);
    }

    #[test]
    fn can_capture_pops() {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/StateVariablesWritten.sol");

        let mut total_state_variables_popped = 0;

        for contract in context.contract_definitions() {
            let state_variables_info = ExtractManipulatedStateVariablesIDs::from(contract);
            println!("{} - {}", contract.name, state_variables_info.popped.len());
            println!("{:?}", state_variables_info.popped);
            total_state_variables_popped += state_variables_info.popped.len();
        }

        assert_eq!(total_state_variables_popped, 1);
    }

    #[test]
    fn can_capture_assignments() {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/StateVariablesWritten.sol");

        let mut total_state_variables_assigned = 0;

        for contract in context.contract_definitions() {
            let state_variables_info = ExtractManipulatedStateVariablesIDs::from(contract);
            println!(
                "{} - {}",
                contract.name,
                state_variables_info.assigned.len()
            );
            println!("{:?}", state_variables_info.assigned);
            total_state_variables_assigned += state_variables_info.assigned.len();
        }

        assert_eq!(total_state_variables_assigned, 11);
    }
}
