use crate::ast::*;

impl ContractDefinitionNode {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                Some(using_for_directive.id)
            }
            ContractDefinitionNode::StructDefinition(struct_definition) => {
                Some(struct_definition.id)
            }
            ContractDefinitionNode::EnumDefinition(enum_definition) => Some(enum_definition.id),
            ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                Some(variable_declaration.id)
            }
            ContractDefinitionNode::EventDefinition(event_definition) => Some(event_definition.id),
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                Some(function_definition.id)
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                Some(modifier_definition.id)
            }
            ContractDefinitionNode::ErrorDefinition(error_definition) => Some(error_definition.id),
            ContractDefinitionNode::UserDefinedValueTypeDefinition(
                user_defined_value_type_definition,
            ) => Some(user_defined_value_type_definition.id),
        }
    }
}

impl Expression {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            Expression::Literal(literal) => Some(literal.id),
            Expression::Identifier(identifier) => Some(identifier.id),
            Expression::UnaryOperation(unary_operation) => Some(unary_operation.id),
            Expression::BinaryOperation(binary_operation) => Some(binary_operation.id),
            Expression::Conditional(conditional) => Some(conditional.id),
            Expression::Assignment(assignment) => Some(assignment.id),
            Expression::FunctionCall(function_call) => Some(function_call.id),
            Expression::FunctionCallOptions(function_call_options) => {
                Some(function_call_options.id)
            }
            Expression::IndexAccess(index_access) => Some(index_access.id),
            Expression::IndexRangeAccess(index_range_access) => Some(index_range_access.id),
            Expression::MemberAccess(member_access) => Some(member_access.id),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                Some(elementary_type_name_expression.id)
            }

            Expression::TupleExpression(tuple_expression) => Some(tuple_expression.id),
            Expression::NewExpression(new_expression) => Some(new_expression.id),
        }
    }
}

impl SourceUnitNode {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            SourceUnitNode::PragmaDirective(pragma_directive) => Some(pragma_directive.id),
            SourceUnitNode::ImportDirective(import_directive) => Some(import_directive.id),
            SourceUnitNode::ContractDefinition(contract_definition) => Some(contract_definition.id),
            SourceUnitNode::StructDefinition(struct_definition) => Some(struct_definition.id),
            SourceUnitNode::EnumDefinition(enum_definition) => Some(enum_definition.id),
            SourceUnitNode::ErrorDefinition(error_definition) => Some(error_definition.id),
            SourceUnitNode::VariableDeclaration(variable_declaration) => {
                Some(variable_declaration.id)
            }
            SourceUnitNode::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
                Some(user_defined_value_type_definition.id)
            }
            SourceUnitNode::FunctionDefinition(function_defn) => Some(function_defn.id),
            SourceUnitNode::UsingForDirective(using_for_directive) => Some(using_for_directive.id),
            SourceUnitNode::EventDefinition(event_definition) => Some(event_definition.id),
        }
    }
}

impl Statement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                Some(variable_declaration_statement.id)
            }
            Statement::IfStatement(if_statement) => Some(if_statement.id),
            Statement::ForStatement(for_statement) => Some(for_statement.id),
            Statement::WhileStatement(while_statement) => Some(while_statement.id),
            Statement::EmitStatement(emit_statement) => Some(emit_statement.id),
            Statement::UncheckedBlock(unchecked_statement) => Some(unchecked_statement.id),
            Statement::Return(return_statement) => Some(return_statement.id),
            Statement::RevertStatement(revert_statement) => Some(revert_statement.error_call.id),
            Statement::ExpressionStatement(expression_statement) => Some(expression_statement.id),
            Statement::InlineAssembly(inline_assembly) => Some(inline_assembly.id),
            Statement::TryStatement(try_statement) => Some(try_statement.id),
            Statement::Block(block) => Some(block.id),
            Statement::Break(break_statement) => Some(break_statement.id),
            Statement::Continue(continue_statement) => Some(continue_statement.id),
            Statement::DoWhileStatement(do_while_statement) => Some(do_while_statement.id),
            Statement::PlaceholderStatement(placeholder) => Some(placeholder.id),
        }
    }
}

impl UserDefinedTypeNameOrIdentifierPath {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(node) => Some(node.id),
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(node) => Some(node.id),
        }
    }
}

impl TypeName {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            TypeName::FunctionTypeName(node) => Some(node.id),
            TypeName::ArrayTypeName(node) => Some(node.id),
            TypeName::Mapping(node) => Some(node.id),
            TypeName::UserDefinedTypeName(node) => Some(node.id),
            TypeName::ElementaryTypeName(node) => Some(node.id),
            TypeName::Raw(_) => None,
        }
    }
}

impl IdentifierOrIdentifierPath {
    pub fn get_node_id(&self) -> NodeID {
        match self {
            IdentifierOrIdentifierPath::Identifier(n) => n.id,
            IdentifierOrIdentifierPath::IdentifierPath(n) => n.id,
        }
    }
}

impl BlockOrStatement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            BlockOrStatement::Block(block) => Some(block.id),
            BlockOrStatement::Statement(statement) => statement.get_node_id(),
        }
    }
}
