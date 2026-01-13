use super::ast_visitor::ASTConstVisitor;
use crate::{
    ast::*,
    context::workspace::{NodeContext, WorkspaceContext},
};
use eyre::Result;

macro_rules! generate_visit_methods_for_workspace_context_with_insert_node {
    (
        regular:
        $( $node:ident ),* $(,)*;

        yul:
        $( $yul_node:ident ),* $(,)*;

    ) => {
        paste::paste! {
            // Regular nodes
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

            // Yul nodes (no ID)
            $(
                fn [<visit_ $yul_node:snake>](&mut self, node: &$yul_node) -> Result<bool> {
                    self.[<$yul_node:snake s_context>].insert(
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
        regular:
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
            WhileStatement;
        yul:
            YulAssignment,
            YulBlock,
            YulCase,
            YulExpression,
            YulExpressionStatement,
            YulForLoop,
            YulFunctionCall,
            YulFunctionDefinition,
            YulIdentifier,
            YulIf,
            YulLiteral,
            YulStatement,
            YulSwitch,
            YulTypedName,
            YulVariableDeclaration;
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
