use super::ast_visitor::ASTConstVisitor;
use crate::{
    ast::*,
    context::workspace::{NodeContext, WorkspaceContext},
};
use eyre::Result;

macro_rules! generate_visit_methods_for_workspace_context_with_insert_node {
    ($( $node:ident ),* $(,)*) => {
        paste::paste! {
            $(
                fn [<visit_ $node:snake>](&mut self, node: &$node) -> Result<bool> {
                    self.nodes
                        .insert(node.id, ASTNode::$node(node.clone()));
                    self.[<$node:snake s_context>].insert(
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
            )*
        }
    };
}

impl ASTConstVisitor for WorkspaceContext {
    generate_visit_methods_for_workspace_context_with_insert_node! {
        ArrayTypeName,
        Assignment,
        BinaryOperation,
        Block,
        Break,
        Conditional,
        Continue,
        DoWhileStatement,
        ElementaryTypeName,
        ElementaryTypeNameExpression,
        EmitStatement,
        EnumDefinition,
        EnumValue,
        ErrorDefinition,
        EventDefinition,
        ExpressionStatement,
        ForStatement,
        FunctionCall,
        FunctionCallOptions,
        FunctionTypeName,
        Identifier,
        IdentifierPath,
        IfStatement,
        ImportDirective,
        IndexAccess,
        IndexRangeAccess,
        InheritanceSpecifier,
        InlineAssembly,
        Literal,
        Mapping,
        MemberAccess,
        ModifierInvocation,
        NewExpression,
        OverrideSpecifier,
        ParameterList,
        PlaceholderStatement,
        PragmaDirective,
        Return,
        RevertStatement,
        StructDefinition,
        StructuredDocumentation,
        TryCatchClause,
        TryStatement,
        TupleExpression,
        UnaryOperation,
        UncheckedBlock,
        UserDefinedTypeName,
        UserDefinedValueTypeDefinition,
        UsingForDirective,
        VariableDeclaration,
        VariableDeclarationStatement,
        WhileStatement,
    }

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
