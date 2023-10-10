use std::collections::HashMap;
use eyre::Result;
use crate::ast::*;
use crate::visitor::ast_visitor::*;

#[derive(Debug, Clone)]
pub enum ASTNode {
    ArrayTypeName(ArrayTypeName),
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Block(Block),
    Conditional(Conditional),
    ContractDefinition(ContractDefinition),
    ElementaryTypeName(ElementaryTypeName),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    EmitStatement(EmitStatement),
    EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    ExpressionStatement(ExpressionStatement),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    FunctionDefinition(FunctionDefinition),
    FunctionTypeName(FunctionTypeName),
    ForStatement(ForStatement),
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
    IfStatement(IfStatement),
    ImportDirective(ImportDirective),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    InheritanceSpecifier(InheritanceSpecifier),
    InlineAssembly(InlineAssembly),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    Mapping(Mapping),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ParameterList(ParameterList),
    PragmaDirective(PragmaDirective),
    Return(Return),
    RevertStatement(RevertStatement),
    SourceUnit(SourceUnit),
    StructDefinition(StructDefinition),
    StructuredDocumentation(StructuredDocumentation),
    TryStatement(TryStatement),
    TryCatchClause(TryCatchClause),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement)
}

impl ASTNode {
    pub fn src(&self) -> Option<&str> {
        match self {
            ASTNode::ArrayTypeName(_) => None,
            ASTNode::Assignment(node) => Some(&node.src),
            ASTNode::BinaryOperation(node) => Some(&node.src),
            ASTNode::Block(node) => Some(&node.src),
            ASTNode::Conditional(node) => Some(&node.src),
            ASTNode::ContractDefinition(node) => Some(&node.src),
            ASTNode::ElementaryTypeName(_) => None,
            ASTNode::ElementaryTypeNameExpression(node) => Some(&node.src),
            ASTNode::EmitStatement(_) => None,
            ASTNode::EnumDefinition(node) => Some(&node.src),
            ASTNode::EnumValue(node) => Some(&node.src),
            ASTNode::EventDefinition(node) => Some(&node.src),
            ASTNode::ErrorDefinition(node) => Some(&node.src),
            ASTNode::ExpressionStatement(_) => None,
            ASTNode::FunctionCall(node) => Some(&node.src),
            ASTNode::FunctionCallOptions(node) => Some(&node.src),
            ASTNode::FunctionDefinition(node) => Some(&node.src),
            ASTNode::FunctionTypeName(_) => None,
            ASTNode::ForStatement(node) => Some(&node.src),
            ASTNode::Identifier(node) => Some(&node.src),
            ASTNode::IdentifierPath(node) => Some(&node.src),
            ASTNode::IfStatement(node) => Some(&node.src),
            ASTNode::ImportDirective(node) => Some(&node.src),
            ASTNode::IndexAccess(node) => Some(&node.src),
            ASTNode::IndexRangeAccess(node) => Some(&node.src),
            ASTNode::InheritanceSpecifier(node) => Some(&node.src),
            ASTNode::InlineAssembly(node) => Some(&node.src),
            ASTNode::Literal(node) => Some(&node.src),
            ASTNode::MemberAccess(node) => Some(&node.src),
            ASTNode::NewExpression(node) => Some(&node.src),
            ASTNode::Mapping(_) => None,
            ASTNode::ModifierDefinition(node) => Some(&node.src),
            ASTNode::ModifierInvocation(node) => Some(&node.src),
            ASTNode::OverrideSpecifier(node) => Some(&node.src),
            ASTNode::ParameterList(node) => Some(&node.src),
            ASTNode::PragmaDirective(node) => Some(&node.src),
            ASTNode::Return(node) => Some(&node.src),
            ASTNode::RevertStatement(_) => None,
            ASTNode::SourceUnit(_) => None,
            ASTNode::StructDefinition(node) => Some(&node.src),
            ASTNode::StructuredDocumentation(node) => Some(&node.src),
            ASTNode::TryStatement(_) =>  None,
            ASTNode::TryCatchClause(_) => None,
            ASTNode::TupleExpression(node) => Some(&node.src),
            ASTNode::UnaryOperation(node) => Some(&node.src),
            ASTNode::UserDefinedTypeName(_) => None,
            ASTNode::UserDefinedValueTypeDefinition(node) => Some(&node.src),
            ASTNode::UsingForDirective(node) => Some(&node.src),
            ASTNode::VariableDeclaration(node) => Some(&node.src),
            ASTNode::VariableDeclarationStatement(node) => Some(&node.src),
            ASTNode::WhileStatement(node) => Some(&node.src),
        }
    }
}

#[derive(Default, Debug)]
pub struct ContractLoader {
    pub nodes: HashMap<i64, ASTNode>,
    pub last_source_unit_id: i64,

    // Hashmaps of all nodes => source_unit_id
    pub array_type_names: HashMap<ArrayTypeName, i64>,
    pub assignments: HashMap<Assignment, i64>,
    pub binary_operations: HashMap<BinaryOperation, i64>,
    pub blocks: HashMap<Block, i64>,
    pub conditionals: HashMap<Conditional, i64>,
    pub contract_definitions: HashMap<ContractDefinition, i64>,
    pub elementary_type_names: HashMap<ElementaryTypeName, i64>,
    pub elementary_type_name_expressions: HashMap<ElementaryTypeNameExpression, i64>,
    pub emit_statements: HashMap<EmitStatement, i64>,
    pub enum_definitions: HashMap<EnumDefinition, i64>,
    pub enum_values: HashMap<EnumValue, i64>,
    pub event_definitions: HashMap<EventDefinition, i64>,
    pub error_definitions: HashMap<ErrorDefinition, i64>,
    pub expression_statements: HashMap<ExpressionStatement, i64>,
    pub function_calls: HashMap<FunctionCall, i64>,
    pub function_call_options: HashMap<FunctionCallOptions, i64>,
    pub function_definitions: HashMap<FunctionDefinition, i64>,
    pub function_type_names: HashMap<FunctionTypeName, i64>,
    pub for_statements: HashMap<ForStatement, i64>,
    pub identifiers: HashMap<Identifier, i64>,
    pub identifier_paths: HashMap<IdentifierPath, i64>,
    pub if_statements: HashMap<IfStatement, i64>,
    pub import_directives: HashMap<ImportDirective, i64>,
    pub index_accesses: HashMap<IndexAccess, i64>,
    pub index_range_accesses: HashMap<IndexRangeAccess, i64>,
    pub inheritance_specifiers: HashMap<InheritanceSpecifier, i64>,
    pub inline_assemblies: HashMap<InlineAssembly, i64>,
    pub literals: HashMap<Literal, i64>,
    pub member_accesses: HashMap<MemberAccess, i64>,
    pub new_expressions: HashMap<NewExpression, i64>,
    pub mappings: HashMap<Mapping, i64>,
    pub modifier_definitions: HashMap<ModifierDefinition, i64>,
    pub modifier_invocations: HashMap<ModifierInvocation, i64>,
    pub override_specifiers: HashMap<OverrideSpecifier, i64>,
    pub parameter_lists: HashMap<ParameterList, i64>,
    pub pragma_directives: HashMap<PragmaDirective, i64>,
    pub returns: HashMap<Return, i64>,
    pub revert_statements: HashMap<RevertStatement, i64>,
    pub source_units: Vec<SourceUnit>,
    pub struct_definitions: HashMap<StructDefinition, i64>,
    pub structured_documentations: HashMap<StructuredDocumentation, i64>,
    pub try_statements: HashMap<TryStatement, i64>,
    pub try_catch_clauses: HashMap<TryCatchClause, i64>,
    pub tuple_expressions: HashMap<TupleExpression, i64>,
    pub unary_operations: HashMap<UnaryOperation, i64>,
    pub user_defined_type_names: HashMap<UserDefinedTypeName, i64>,
    pub user_defined_value_type_definitions: HashMap<UserDefinedValueTypeDefinition, i64>,
    pub using_for_directives: HashMap<UsingForDirective, i64>,
    pub variable_declarations: HashMap<VariableDeclaration, i64>,
    pub variable_declaration_statements: HashMap<VariableDeclarationStatement, i64>,
    pub while_statements: HashMap<WhileStatement, i64>,
}

impl ContractLoader {
    pub fn get_node(&self, id: i64) -> Option<&ASTNode> {
        self.nodes.get(&id)
    }

    pub fn get_for_statements(&self) -> Vec<&ForStatement> {
        self.for_statements.keys().collect()
    }

    pub fn get_while_statements(&self) -> Vec<&WhileStatement> {
        self.while_statements.keys().collect()
    }
}

impl ASTConstVisitor for ContractLoader {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.array_type_names.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Assignment(node.clone()));
        self.assignments.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::BinaryOperation(node.clone()));
        self.binary_operations.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Block(node.clone()));
        self.blocks.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Conditional(node.clone()));
        self.conditionals.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ContractDefinition(node.clone()));
        self.contract_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.elementary_type_names.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ElementaryTypeNameExpression(node.clone()));
        self.elementary_type_name_expressions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        self.emit_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EnumDefinition(node.clone()));
        self.enum_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EnumValue(node.clone()));
        self.enum_values.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EventDefinition(node.clone()));
        self.event_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ErrorDefinition(node.clone()));
        self.error_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        self.expression_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::FunctionCall(node.clone()));
        self.function_calls.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::FunctionCallOptions(node.clone()));
        self.function_call_options.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::FunctionDefinition(node.clone()));
        self.function_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.function_type_names.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ForStatement(node.clone()));
        self.for_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Identifier(node.clone()));
        self.identifiers.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IdentifierPath(node.clone()));
        self.identifier_paths.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IfStatement(node.clone()));
        self.if_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ImportDirective(node.clone()));
        self.import_directives.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IndexAccess(node.clone()));
        self.index_accesses.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IndexRangeAccess(node.clone()));
        self.index_range_accesses.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::InheritanceSpecifier(node.clone()));
        self.inheritance_specifiers.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::InlineAssembly(node.clone()));
        self.inline_assemblies.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Literal(node.clone()));
        self.literals.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::MemberAccess(node.clone()));
        self.member_accesses.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::NewExpression(node.clone()));
        self.new_expressions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.mappings.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ModifierDefinition(node.clone()));
        self.modifier_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ModifierInvocation(node.clone()));
        self.modifier_invocations.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::OverrideSpecifier(node.clone()));
        self.override_specifiers.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ParameterList(node.clone()));
        self.parameter_lists.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::PragmaDirective(node.clone()));
        self.pragma_directives.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Return(node.clone()));
        self.returns.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        self.revert_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::SourceUnit(node.clone()));
        self.source_units.push(node.clone());
        self.last_source_unit_id = node.id;
        Ok(true)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::StructDefinition(node.clone()));
        self.struct_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::StructuredDocumentation(node.clone()));
        self.structured_documentations.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        self.try_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        self.try_catch_clauses.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::TupleExpression(node.clone()));
        self.tuple_expressions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::UnaryOperation(node.clone()));
        self.unary_operations.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.user_defined_type_names.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::UserDefinedValueTypeDefinition(node.clone()));
        self.user_defined_value_type_definitions.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::UsingForDirective(node.clone()));
        self.using_for_directives.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }


    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::VariableDeclaration(node.clone()));
        self.variable_declarations.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::VariableDeclarationStatement(node.clone()));
        self.variable_declaration_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::WhileStatement(node.clone()));
        self.while_statements.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }
}

#[cfg(test)]
mod loader_tests {
    use crate::ast::*;
    use crate::compiler::compiler::FoundryOutput;
    use crate::visitor::ast_visitor::*;
    use eyre::Result;
    use crate::loader::loader::ContractLoader;

    fn read_compiler_output(filepath: &str) -> Result<FoundryOutput> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(filepath)?,
        ))?)
    }

    #[derive(Default, Debug)]
    pub struct DelegateCallInLoopDetector {
        pub found_delegate_call_in_loop: Vec<MemberAccess>,
    }

    impl ASTConstVisitor for DelegateCallInLoopDetector {
        fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
            if node.member_name == "delegatecall" {
                self.found_delegate_call_in_loop.push(node.clone());
            }
            Ok(true)
        }
    }

    #[test]
    fn test_delegate_call_in_loops() -> Result<()> {
        let mut loader = ContractLoader::default();
        let extended_inheritance = read_compiler_output("tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json")?;
        let inheritance_base = read_compiler_output("tests/contract-playground/out/InheritanceBase.sol/InheritanceBase.json")?;
        let i_contract_inheritance = read_compiler_output("tests/contract-playground/out/IContractInheritance.sol/IContractInheritance.json")?;
        extended_inheritance.ast.accept(&mut loader)?;
        inheritance_base.ast.accept(&mut loader)?;
        i_contract_inheritance.ast.accept(&mut loader)?;

        // Get all for statements, and check if there is a delegate call in the body of each for statement
        let mut delegate_call_in_loop_detector = DelegateCallInLoopDetector::default();
        let for_statements = loader.get_for_statements();
        for for_statement in for_statements {
            for_statement.accept(&mut delegate_call_in_loop_detector)?;
        }
        println!("Found delegate call in loop: {:?}", delegate_call_in_loop_detector.found_delegate_call_in_loop);

        Ok(())
    }
}