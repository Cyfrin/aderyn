use std::collections::HashMap;
use eyre::Result;
use crate::ast::*;
use crate::visitor::ast_visitor::*;

#[derive(Debug)]

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

#[derive(Default, Debug)]
pub struct ContractLoader {
    pub nodes: HashMap<i64, ASTNode>,
    pub type_names: HashMap<TypeName, bool>,
    pub statements: HashMap<Statement, bool>,
    pub try_catch_clauses: HashMap<TryCatchClause, bool>,
}

impl ContractLoader {
    pub fn get(&self, id: i64) -> Option<&ASTNode> {
        self.nodes.get(&id)
    }
}

impl ASTConstVisitor for ContractLoader {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.type_names.insert(TypeName::ArrayTypeName(node.clone()), true);
        Ok(true)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Assignment(node.clone()));
        Ok(true)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::BinaryOperation(node.clone()));
        Ok(true)
    }

    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Block(node.clone()));
        Ok(true)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Conditional(node.clone()));
        Ok(true)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ContractDefinition(node.clone()));
        Ok(true)
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.type_names.insert( TypeName::ElementaryTypeName(node.clone()), true);
        Ok(true)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ElementaryTypeNameExpression(node.clone()));
        Ok(true)
    }

    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        self.statements.insert( Statement::EmitStatement(node.clone()), true);
        Ok(true)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EnumDefinition(node.clone()));
        Ok(true)
    }

    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EnumValue(node.clone()));
        Ok(true)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EventDefinition(node.clone()));
        Ok(true)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ErrorDefinition(node.clone()));
        Ok(true)
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        self.statements.insert(Statement::ExpressionStatement(node.clone()), true);
        Ok(true)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::FunctionCall(node.clone()));
        Ok(true)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::FunctionCallOptions(node.clone()));
        Ok(true)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::FunctionDefinition(node.clone()));
        Ok(true)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.type_names.insert(TypeName::FunctionTypeName(node.clone()), true);
        Ok(true)
    }

    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ForStatement(node.clone()));
        Ok(true)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Identifier(node.clone()));
        Ok(true)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IdentifierPath(node.clone()));
        Ok(true)
    }

    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IfStatement(node.clone()));
        Ok(true)
    }

    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ImportDirective(node.clone()));
        Ok(true)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IndexAccess(node.clone()));
        Ok(true)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::IndexRangeAccess(node.clone()));
        Ok(true)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::InheritanceSpecifier(node.clone()));
        Ok(true)
    }

    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::InlineAssembly(node.clone()));
        Ok(true)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Literal(node.clone()));
        Ok(true)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::MemberAccess(node.clone()));
        Ok(true)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::NewExpression(node.clone()));
        Ok(true)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.type_names.insert(TypeName::Mapping(node.clone()), true);
        Ok(true)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ModifierDefinition(node.clone()));
        Ok(true)
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ModifierInvocation(node.clone()));
        Ok(true)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::OverrideSpecifier(node.clone()));
        Ok(true)
    }

    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::ParameterList(node.clone()));
        Ok(true)
    }

    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::PragmaDirective(node.clone()));
        Ok(true)
    }

    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Return(node.clone()));
        Ok(true)
    }

    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        self.statements.insert( Statement::RevertStatement(node.clone()), true);
        Ok(true)
    }

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::SourceUnit(node.clone()));
        Ok(true)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::StructDefinition(node.clone()));
        Ok(true)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::StructuredDocumentation(node.clone()));
        Ok(true)
    }

    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        self.statements.insert( Statement::TryStatement(node.clone()), true);
        Ok(true)
    }

    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        self.try_catch_clauses.insert(node.clone(), true);
        Ok(true)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::TupleExpression(node.clone()));
        Ok(true)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::UnaryOperation(node.clone()));
        Ok(true)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.type_names.insert( TypeName::UserDefinedTypeName(node.clone()), true);
        Ok(true)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::UserDefinedValueTypeDefinition(node.clone()));
        Ok(true)
    }

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::UsingForDirective(node.clone()));
        Ok(true)
    }


    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::VariableDeclaration(node.clone()));
        Ok(true)
    }

    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::VariableDeclarationStatement(node.clone()));
        Ok(true)
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::WhileStatement(node.clone()));
        Ok(true)
    }
}

#[cfg(test)]
mod loader_tests {
    use crate::ast::*;
    use crate::visitor::ast_visitor::*;
    use eyre::Result;
    use crate::loader::loader::ContractLoader;

    fn read_abi_encode_packed() -> Result<SourceUnit> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            // std::fs::File::open("tests/ast-json/StateVariables.ast.json")?,
            std::fs::File::open("tests/ast-json/AbiEncodePacked.json")?,
        ))?)
    }

    #[test]
    fn test_contract_loader() -> Result<()> {
        let source_unit = read_abi_encode_packed()?;
        let mut loader = ContractLoader::default();
        source_unit.accept(&mut loader)?;
        loader.nodes.into_iter().for_each(|(id, entry_type)| {
            println!("{}: ", id);
        });
        Ok(())
    }
}