use crate::ast::*;
use crate::WorkspaceContext;

impl WorkspaceContext {
    pub fn array_type_names(&self) -> Vec<&ArrayTypeName> {
        self.array_type_names_context.keys().collect()
    }
    pub fn assignments(&self) -> Vec<&Assignment> {
        self.assignments_context.keys().collect()
    }
    pub fn binary_operations(&self) -> Vec<&BinaryOperation> {
        self.binary_operations_context.keys().collect()
    }
    pub fn blocks(&self) -> Vec<&Block> {
        self.blocks_context.keys().collect()
    }
    pub fn conditionals(&self) -> Vec<&Conditional> {
        self.conditionals_context.keys().collect()
    }
    pub fn contract_definitions(&self) -> Vec<&ContractDefinition> {
        self.contract_definitions_context.keys().collect()
    }
    pub fn elementary_type_names(&self) -> Vec<&ElementaryTypeName> {
        self.elementary_type_names_context.keys().collect()
    }
    pub fn elementary_type_name_expressions(&self) -> Vec<&ElementaryTypeNameExpression> {
        self.elementary_type_name_expressions_context
            .keys()
            .collect()
    }
    pub fn emit_statements(&self) -> Vec<&EmitStatement> {
        self.emit_statements_context.keys().collect()
    }
    pub fn enum_definitions(&self) -> Vec<&EnumDefinition> {
        self.enum_definitions_context.keys().collect()
    }
    pub fn enum_values(&self) -> Vec<&EnumValue> {
        self.enum_values_context.keys().collect()
    }
    pub fn event_definitions(&self) -> Vec<&EventDefinition> {
        self.event_definitions_context.keys().collect()
    }
    pub fn error_definitions(&self) -> Vec<&ErrorDefinition> {
        self.error_definitions_context.keys().collect()
    }
    pub fn expression_statements(&self) -> Vec<&ExpressionStatement> {
        self.expression_statements_context.keys().collect()
    }
    pub fn function_calls(&self) -> Vec<&FunctionCall> {
        self.function_calls_context.keys().collect()
    }
    pub fn function_call_options(&self) -> Vec<&FunctionCallOptions> {
        self.function_call_options_context.keys().collect()
    }
    pub fn function_definitions(&self) -> Vec<&FunctionDefinition> {
        self.function_definitions_context.keys().collect()
    }
    pub fn function_type_names(&self) -> Vec<&FunctionTypeName> {
        self.function_type_names_context.keys().collect()
    }
    pub fn for_statements(&self) -> Vec<&ForStatement> {
        self.for_statements_context.keys().collect()
    }
    pub fn identifiers(&self) -> Vec<&Identifier> {
        self.identifiers_context.keys().collect()
    }
    pub fn identifier_paths(&self) -> Vec<&IdentifierPath> {
        self.identifier_paths_context.keys().collect()
    }
    pub fn if_statements(&self) -> Vec<&IfStatement> {
        self.if_statements_context.keys().collect()
    }
    pub fn import_directives(&self) -> Vec<&ImportDirective> {
        self.import_directives_context.keys().collect()
    }
    pub fn index_accesses(&self) -> Vec<&IndexAccess> {
        self.index_accesses_context.keys().collect()
    }
    pub fn index_range_accesses(&self) -> Vec<&IndexRangeAccess> {
        self.index_range_accesses_context.keys().collect()
    }
    pub fn inheritance_specifiers(&self) -> Vec<&InheritanceSpecifier> {
        self.inheritance_specifiers_context.keys().collect()
    }
    pub fn inline_assemblies(&self) -> Vec<&InlineAssembly> {
        self.inline_assemblies_context.keys().collect()
    }
    pub fn literals(&self) -> Vec<&Literal> {
        self.literals_context.keys().collect()
    }
    pub fn member_accesses(&self) -> Vec<&MemberAccess> {
        self.member_accesses_context.keys().collect()
    }
    pub fn new_expressions(&self) -> Vec<&NewExpression> {
        self.new_expressions_context.keys().collect()
    }
    pub fn mappings(&self) -> Vec<&Mapping> {
        self.mappings_context.keys().collect()
    }
    pub fn modifier_definitions(&self) -> Vec<&ModifierDefinition> {
        self.modifier_definitions_context.keys().collect()
    }
    pub fn modifier_invocations(&self) -> Vec<&ModifierInvocation> {
        self.modifier_invocations_context.keys().collect()
    }
    pub fn override_specifiers(&self) -> Vec<&OverrideSpecifier> {
        self.override_specifiers_context.keys().collect()
    }
    pub fn parameter_lists(&self) -> Vec<&ParameterList> {
        self.parameter_lists_context.keys().collect()
    }
    pub fn pragma_directives(&self) -> Vec<&PragmaDirective> {
        self.pragma_directives_context.keys().collect()
    }
    pub fn returns(&self) -> Vec<&Return> {
        self.returns_context.keys().collect()
    }
    pub fn revert_statements(&self) -> Vec<&RevertStatement> {
        self.revert_statements_context.keys().collect()
    }
    pub fn source_units(&self) -> Vec<&SourceUnit> {
        self.source_units_context.iter().collect()
    }
    pub fn struct_definitions(&self) -> Vec<&StructDefinition> {
        self.struct_definitions_context.keys().collect()
    }
    pub fn structured_documentations(&self) -> Vec<&StructuredDocumentation> {
        self.structured_documentations_context.keys().collect()
    }
    pub fn try_statements(&self) -> Vec<&TryStatement> {
        self.try_statements_context.keys().collect()
    }
    pub fn try_catch_clauses(&self) -> Vec<&TryCatchClause> {
        self.try_catch_clauses_context.keys().collect()
    }
    pub fn tuple_expressions(&self) -> Vec<&TupleExpression> {
        self.tuple_expressions_context.keys().collect()
    }
    pub fn unary_operations(&self) -> Vec<&UnaryOperation> {
        self.unary_operations_context.keys().collect()
    }
    pub fn user_defined_type_names(&self) -> Vec<&UserDefinedTypeName> {
        self.user_defined_type_names_context.keys().collect()
    }
    pub fn user_defined_value_type_definitions(&self) -> Vec<&UserDefinedValueTypeDefinition> {
        self.user_defined_value_type_definitions_context
            .keys()
            .collect()
    }
    pub fn using_for_directives(&self) -> Vec<&UsingForDirective> {
        self.using_for_directives_context.keys().collect()
    }
    pub fn variable_declarations(&self) -> Vec<&VariableDeclaration> {
        self.variable_declarations_context.keys().collect()
    }
    pub fn variable_declaration_statements(&self) -> Vec<&VariableDeclarationStatement> {
        self.variable_declaration_statements_context
            .keys()
            .collect()
    }
    pub fn while_statements(&self) -> Vec<&WhileStatement> {
        self.while_statements_context.keys().collect()
    }

    pub fn do_while_statements(&self) -> Vec<&DoWhileStatement> {
        self.do_while_statements_context.keys().collect()
    }

    pub fn break_statements(&self) -> Vec<&Break> {
        self.break_statements_context.keys().collect()
    }

    pub fn continue_statements(&self) -> Vec<&Continue> {
        self.continue_statements_context.keys().collect()
    }

    pub fn placeholder_statements(&self) -> Vec<&PlaceholderStatement> {
        self.placeholder_statements_context.keys().collect()
    }
}

impl WorkspaceContext {
    pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
        let source_unit_id = match node {
            ASTNode::ArrayTypeName(node) => self
                .array_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Assignment(node) => self
                .assignments_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::BinaryOperation(node) => self
                .binary_operations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Block(node) => self
                .blocks_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Conditional(node) => self
                .conditionals_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ContractDefinition(node) => self
                .contract_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ElementaryTypeName(node) => self
                .elementary_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ElementaryTypeNameExpression(node) => self
                .elementary_type_name_expressions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EmitStatement(node) => self
                .emit_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EnumDefinition(node) => self
                .enum_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EnumValue(node) => self
                .enum_values_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::EventDefinition(node) => self
                .event_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ErrorDefinition(node) => self
                .error_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ExpressionStatement(node) => self
                .expression_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionCall(node) => self
                .function_calls_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionCallOptions(node) => self
                .function_call_options_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionDefinition(node) => self
                .function_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::FunctionTypeName(node) => self
                .function_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ForStatement(node) => self
                .for_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Identifier(node) => self
                .identifiers_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IdentifierPath(node) => self
                .identifier_paths_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IfStatement(node) => self
                .if_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ImportDirective(node) => self
                .import_directives_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IndexAccess(node) => self
                .index_accesses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::IndexRangeAccess(node) => self
                .index_range_accesses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::InheritanceSpecifier(node) => self
                .inheritance_specifiers_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::InlineAssembly(node) => self
                .inline_assemblies_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Literal(node) => self
                .literals_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::MemberAccess(node) => self
                .member_accesses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::NewExpression(node) => self
                .new_expressions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Mapping(node) => self
                .mappings_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ModifierDefinition(node) => self
                .modifier_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ModifierInvocation(node) => self
                .modifier_invocations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::OverrideSpecifier(node) => self
                .override_specifiers_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::ParameterList(node) => self
                .parameter_lists_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::PragmaDirective(node) => self
                .pragma_directives_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Return(node) => self
                .returns_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::RevertStatement(node) => self
                .revert_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::SourceUnit(node) => Some(node.id),
            ASTNode::StructDefinition(node) => self
                .struct_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::StructuredDocumentation(node) => self
                .structured_documentations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TryStatement(node) => self
                .try_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TryCatchClause(node) => self
                .try_catch_clauses_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::TupleExpression(node) => self
                .tuple_expressions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UnaryOperation(node) => self
                .unary_operations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UserDefinedTypeName(node) => self
                .user_defined_type_names_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UserDefinedValueTypeDefinition(node) => self
                .user_defined_value_type_definitions_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::UsingForDirective(node) => self
                .using_for_directives_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::VariableDeclaration(node) => self
                .variable_declarations_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::VariableDeclarationStatement(node) => self
                .variable_declaration_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::WhileStatement(node) => self
                .while_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::DoWhileStatement(node) => self
                .do_while_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Break(node) => self
                .break_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::Continue(node) => self
                .continue_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
            ASTNode::PlaceholderStatement(node) => self
                .placeholder_statements_context
                .get(node)
                .map(|context| context.source_unit_id),
        };

        // iterate through self.source_units until the source unit with the id matching `source_unit_id` is found, then return its `absolute_path`

        source_unit_id.and_then(|id| {
            self.source_units_context
                .iter()
                .find(|source_unit| source_unit.id == id)
        })
    }
}
