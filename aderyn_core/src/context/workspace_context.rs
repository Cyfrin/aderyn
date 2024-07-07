use crate::ast::*;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::browser::GetImmediateParent;
use super::capturable::Capturable;
pub use crate::ast::ASTNode;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeContext {
    pub source_unit_id: NodeID,
    pub contract_definition_id: Option<NodeID>,
    pub function_definition_id: Option<NodeID>,
    pub modifier_definition_id: Option<NodeID>,
}

#[derive(Default, Debug)]
pub struct WorkspaceContext {
    pub last_source_unit_id: NodeID,
    pub last_contract_definition_id: Option<NodeID>,
    pub last_function_definition_id: Option<NodeID>,
    pub last_modifier_definition_id: Option<NodeID>,

    pub parent_link: HashMap<NodeID, NodeID>,

    // relative source filepaths
    pub src_filepaths: Vec<String>,
    pub sloc_stats: HashMap<String, usize>,
    pub nodes: HashMap<NodeID, ASTNode>,

    // Hashmaps of all nodes => source_unit_id
    pub(crate) array_type_names_context: HashMap<ArrayTypeName, NodeContext>,
    pub(crate) assignments_context: HashMap<Assignment, NodeContext>,
    pub(crate) binary_operations_context: HashMap<BinaryOperation, NodeContext>,
    pub(crate) blocks_context: HashMap<Block, NodeContext>,
    pub(crate) conditionals_context: HashMap<Conditional, NodeContext>,
    pub(crate) contract_definitions_context: HashMap<ContractDefinition, NodeContext>,
    pub(crate) elementary_type_names_context: HashMap<ElementaryTypeName, NodeContext>,
    pub(crate) elementary_type_name_expressions_context:
        HashMap<ElementaryTypeNameExpression, NodeContext>,
    pub(crate) emit_statements_context: HashMap<EmitStatement, NodeContext>,
    pub(crate) enum_definitions_context: HashMap<EnumDefinition, NodeContext>,
    pub(crate) enum_values_context: HashMap<EnumValue, NodeContext>,
    pub(crate) event_definitions_context: HashMap<EventDefinition, NodeContext>,
    pub(crate) error_definitions_context: HashMap<ErrorDefinition, NodeContext>,
    pub(crate) expression_statements_context: HashMap<ExpressionStatement, NodeContext>,
    pub(crate) function_calls_context: HashMap<FunctionCall, NodeContext>,
    pub(crate) function_call_options_context: HashMap<FunctionCallOptions, NodeContext>,
    pub(crate) function_definitions_context: HashMap<FunctionDefinition, NodeContext>,
    pub(crate) function_type_names_context: HashMap<FunctionTypeName, NodeContext>,
    pub(crate) for_statements_context: HashMap<ForStatement, NodeContext>,
    pub(crate) identifiers_context: HashMap<Identifier, NodeContext>,
    pub(crate) identifier_paths_context: HashMap<IdentifierPath, NodeContext>,
    pub(crate) if_statements_context: HashMap<IfStatement, NodeContext>,
    pub(crate) import_directives_context: HashMap<ImportDirective, NodeContext>,
    pub(crate) index_accesses_context: HashMap<IndexAccess, NodeContext>,
    pub(crate) index_range_accesses_context: HashMap<IndexRangeAccess, NodeContext>,
    pub(crate) inheritance_specifiers_context: HashMap<InheritanceSpecifier, NodeContext>,
    pub(crate) inline_assemblies_context: HashMap<InlineAssembly, NodeContext>,
    pub(crate) literals_context: HashMap<Literal, NodeContext>,
    pub(crate) member_accesses_context: HashMap<MemberAccess, NodeContext>,
    pub(crate) new_expressions_context: HashMap<NewExpression, NodeContext>,
    pub(crate) mappings_context: HashMap<Mapping, NodeContext>,
    pub(crate) modifier_definitions_context: HashMap<ModifierDefinition, NodeContext>,
    pub(crate) modifier_invocations_context: HashMap<ModifierInvocation, NodeContext>,
    pub(crate) override_specifiers_context: HashMap<OverrideSpecifier, NodeContext>,
    pub(crate) parameter_lists_context: HashMap<ParameterList, NodeContext>,
    pub(crate) pragma_directives_context: HashMap<PragmaDirective, NodeContext>,
    pub(crate) returns_context: HashMap<Return, NodeContext>,
    pub(crate) revert_statements_context: HashMap<RevertStatement, NodeContext>,
    pub(crate) source_units_context: Vec<SourceUnit>,
    pub(crate) struct_definitions_context: HashMap<StructDefinition, NodeContext>,
    pub(crate) structured_documentations_context: HashMap<StructuredDocumentation, NodeContext>,
    pub(crate) try_statements_context: HashMap<TryStatement, NodeContext>,
    pub(crate) try_catch_clauses_context: HashMap<TryCatchClause, NodeContext>,
    pub(crate) tuple_expressions_context: HashMap<TupleExpression, NodeContext>,
    pub(crate) unary_operations_context: HashMap<UnaryOperation, NodeContext>,
    pub(crate) user_defined_type_names_context: HashMap<UserDefinedTypeName, NodeContext>,
    pub(crate) user_defined_value_type_definitions_context:
        HashMap<UserDefinedValueTypeDefinition, NodeContext>,
    pub(crate) using_for_directives_context: HashMap<UsingForDirective, NodeContext>,
    pub(crate) variable_declarations_context: HashMap<VariableDeclaration, NodeContext>,
    pub(crate) variable_declaration_statements_context:
        HashMap<VariableDeclarationStatement, NodeContext>,
    pub(crate) while_statements_context: HashMap<WhileStatement, NodeContext>,
    pub(crate) do_while_statements_context: HashMap<DoWhileStatement, NodeContext>,
    pub(crate) break_statements_context: HashMap<Break, NodeContext>,
    pub(crate) continue_statements_context: HashMap<Continue, NodeContext>,
    pub(crate) placeholder_statements_context: HashMap<PlaceholderStatement, NodeContext>,
}

impl WorkspaceContext {
    // Setters

    pub fn set_sloc_stats(&mut self, sloc_stats: HashMap<String, usize>) {
        self.sloc_stats = sloc_stats;
    }

    // Getters

    pub fn get_parent(&self, node_id: NodeID) -> Option<&ASTNode> {
        self.nodes.get(self.parent_link.get(&node_id)?)
    }

    pub fn get_ancestral_line(&self, node_id: NodeID) -> Vec<&ASTNode> {
        let mut chain = vec![];
        let mut parent = self.nodes.get(&node_id);
        while let Some(next_parent) = parent {
            chain.push(next_parent);
            parent = next_parent.parent(self);
        }
        chain
    }
    pub fn get_closest_ancestor(&self, node_id: NodeID, node_type: NodeType) -> Option<&ASTNode> {
        let mut current_node_id = self.parent_link.get(&node_id)?;
        while let Some(current) = self.nodes.get(current_node_id) {
            if current.node_type() == node_type {
                return Some(current);
            }
            current_node_id = self.parent_link.get(current_node_id)?;
        }
        None
    }

    pub fn get_source_code_of_node(&self, node_id: NodeID) -> Option<String> {
        let node = self.nodes.get(&node_id)?;
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let src_location = node.src().unwrap_or("");

        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        if let Some((offset, len)) = chopped_location.split_once(':') {
            let offset: usize = offset.parse().ok()?;
            let len: usize = len.parse().ok()?;
            if let Some(content) = source_unit.source.as_ref() {
                if offset + len < content.len() {
                    let requried_content = &content[offset..offset + len];
                    return Some(requried_content.to_string());
                }
            }
        }
        None
    }

    pub fn get_offset_and_length_of_node(&self, node_id: NodeID) -> Option<(usize, usize)> {
        let node = self.nodes.get(&node_id)?;
        let src_location = node.src().unwrap_or("");

        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        if let Some((offset, len)) = chopped_location.split_once(':') {
            let offset: usize = offset.parse().ok()?;
            let len: usize = len.parse().ok()?;
            return Some((offset, len));
        }
        None
    }

    pub fn get_node_sort_key_from_capturable(
        &self,
        capturable: &Capturable,
    ) -> (String, usize, String) {
        capturable.make_key(self)
    }

    pub fn get_node_id_of_capturable(&self, capturable: &Capturable) -> Option<NodeID> {
        capturable.id()
    }

    /// Returns the relative location of nodes in the source code (if they are in same file)
    pub fn get_relative_location_of_nodes(
        &self,
        first: NodeID,
        second: NodeID,
    ) -> Option<Ordering> {
        let f = self.get_node_sort_key_pure(self.nodes.get(&first)?);
        let s = self.get_node_sort_key_pure(self.nodes.get(&second)?);

        // If the nodes aren't in the same file location comparison doesn't make sense
        if f.0 != s.0 {
            return None;
        }

        match f.1.cmp(&s.1) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => {
                // If the nodes are on the same line, we must compare offset in the chopped_location
                let first_character_offset = f.2.split_once(':').unwrap();
                let second_character_offset = s.2.split_once(':').unwrap();
                Some(first_character_offset.0.cmp(second_character_offset.0))
            }
            Ordering::Greater => Some(Ordering::Greater),
        }
    }

    pub fn get_node_sort_key_pure(&self, node: &ASTNode) -> (String, usize, String) {
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();
        let source_line = node
            .src()
            .map(|src| source_unit.source_line(src).unwrap_or(0)) // If `src` is `Some`, get the line number, else return 0
            .unwrap_or(0); // If `src` is `None`, default to 0

        let src_location = node.src().unwrap_or("");

        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        (absolute_path, source_line, chopped_location)
    }

    pub fn get_node_sort_key(&self, node: &ASTNode) -> (String, usize, String) {
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();
        let source_line = node
            .src()
            .map(|src| source_unit.source_line(src).unwrap_or(0)) // If `src` is `Some`, get the line number, else return 0
            .unwrap_or(0); // If `src` is `None`, default to 0

        // If the node is one of these, and it has a `name_location`, use that instead of the full `src`
        let src_location = match node {
            ASTNode::ContractDefinition(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            ASTNode::FunctionDefinition(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            ASTNode::ModifierDefinition(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            ASTNode::VariableDeclaration(node) => {
                if let Some(name_location) = &node.name_location {
                    name_location
                } else {
                    &node.src
                }
            }
            _ => node.src().unwrap_or(""),
        };
        let chopped_location = match src_location.rfind(':') {
            Some(index) => &src_location[..index],
            None => src_location, // No colon found, return the original string
        }
        .to_string();

        (absolute_path, source_line, chopped_location)
    }
}
