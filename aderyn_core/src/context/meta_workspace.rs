use crate::ast::*;
use crate::WorkspaceContext;

use super::macros::generate_get_source_unit;

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

    pub fn yul_function_calls(&self) -> Vec<&YulFunctionCall> {
        self.yul_function_calls_context.keys().collect()
    }

    pub fn yul_identifiers(&self) -> Vec<&YulIdentifier> {
        self.yul_identifiers_context.keys().collect()
    }

    pub fn yul_assignments(&self) -> Vec<&YulAssignment> {
        self.yul_assignments_context.keys().collect()
    }

    pub fn yul_literals(&self) -> Vec<&YulLiteral> {
        self.yul_literals_context.keys().collect()
    }
}

generate_get_source_unit! {
    ArrayTypeName => array_type_names_context,
    Assignment => assignments_context,
    BinaryOperation => binary_operations_context,
    Block => blocks_context,
    Conditional => conditionals_context,
    ContractDefinition => contract_definitions_context,
    ElementaryTypeName => elementary_type_names_context,
    ElementaryTypeNameExpression => elementary_type_name_expressions_context,
    EmitStatement => emit_statements_context,
    EnumDefinition => enum_definitions_context,
    EnumValue => enum_values_context,
    EventDefinition => event_definitions_context,
    ErrorDefinition => error_definitions_context,
    ExpressionStatement => expression_statements_context,
    FunctionCall => function_calls_context,
    FunctionCallOptions => function_call_options_context,
    FunctionDefinition => function_definitions_context,
    FunctionTypeName => function_type_names_context,
    ForStatement => for_statements_context,
    Identifier => identifiers_context,
    IdentifierPath => identifier_paths_context,
    IfStatement => if_statements_context,
    ImportDirective => import_directives_context,
    IndexAccess => index_accesses_context,
    IndexRangeAccess => index_range_accesses_context,
    InheritanceSpecifier => inheritance_specifiers_context,
    InlineAssembly => inline_assemblies_context,
    Literal => literals_context,
    MemberAccess => member_accesses_context,
    NewExpression => new_expressions_context,
    Mapping => mappings_context,
    ModifierDefinition => modifier_definitions_context,
    ModifierInvocation => modifier_invocations_context,
    OverrideSpecifier => override_specifiers_context,
    ParameterList => parameter_lists_context,
    PragmaDirective => pragma_directives_context,
    Return => returns_context,
    RevertStatement => revert_statements_context,
    StructDefinition => struct_definitions_context,
    StructuredDocumentation => structured_documentations_context,
    TryStatement => try_statements_context,
    TryCatchClause => try_catch_clauses_context,
    TupleExpression => tuple_expressions_context,
    UnaryOperation => unary_operations_context,
    UserDefinedTypeName => user_defined_type_names_context,
    UserDefinedValueTypeDefinition => user_defined_value_type_definitions_context,
    UsingForDirective => using_for_directives_context,
    VariableDeclaration => variable_declarations_context,
    VariableDeclarationStatement => variable_declaration_statements_context,
    WhileStatement => while_statements_context,
    DoWhileStatement => do_while_statements_context,
    Break => break_statements_context,
    Continue => continue_statements_context,
    PlaceholderStatement => placeholder_statements_context,
    YulFunctionCall => yul_function_calls_context,
    YulIdentifier => yul_identifiers_context,
    YulLiteral => yul_literals_context,
}
