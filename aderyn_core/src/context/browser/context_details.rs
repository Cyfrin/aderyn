pub use crate::ast::ASTNode;
use crate::{
    ast::*,
    context::{
        browser::GetImmediateParent,
        workspace::{Capturable, WorkspaceContext},
    },
    stats::IgnoreLine,
};
use std::{cmp::Ordering, collections::HashMap};

impl WorkspaceContext {
    // Setters
    pub fn set_sloc_stats(&mut self, sloc_stats: HashMap<String, usize>) {
        self.sloc_stats = sloc_stats;
    }

    pub fn set_ignore_lines_stats(&mut self, ignore_lines_stats: HashMap<String, Vec<IgnoreLine>>) {
        self.ignore_lines_stats = ignore_lines_stats;
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
    pub fn get_closest_ancestor_including_self(
        &self,
        node_id: NodeID,
        node_type: NodeType,
    ) -> Option<&ASTNode> {
        if let Some(node) = self.nodes.get(&node_id)
            && node.node_type() == node_type
        {
            return Some(node);
        }
        self.get_closest_ancestor(node_id, node_type)
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
            if let Some(content) = source_unit.source.as_ref()
                && offset + len < content.len()
            {
                let required_content = &content[offset..offset + len];
                return Some(required_content.to_string());
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
        let source_line =
            node.src().map(|src| source_unit.source_line(src).unwrap_or(0)).unwrap_or(0);

        let src_location = match node {
            ASTNode::ContractDefinition(contract_node) => contract_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| contract_node.src.clone(), |loc| loc.clone()),
            ASTNode::FunctionDefinition(function_node) => function_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| function_node.src.clone(), |loc| loc.clone()),
            ASTNode::ModifierDefinition(modifier_node) => modifier_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| modifier_node.src.clone(), |loc| loc.clone()),
            ASTNode::VariableDeclaration(variable_node) => variable_node
                .name_location
                .as_ref()
                .filter(|loc| !loc.contains("-1"))
                .map_or_else(|| variable_node.src.clone(), |loc| loc.clone()),
            _ => node.src().unwrap_or("").to_string(),
        };

        let chopped_location = src_location
            .rfind(':')
            .map(|index| src_location[..index].to_string())
            .unwrap_or(src_location);

        (absolute_path, source_line, chopped_location)
    }
    pub fn get_code_snippet(&self, node: &ASTNode) -> String {
        let (filepath, _, src_location) = self.get_node_sort_key_pure(node);
        let source_unit = self
            .source_units()
            .into_iter()
            .find(|s| s.absolute_path.as_ref().is_some_and(|p| *p == filepath))
            .expect("node not found");

        let source_content = source_unit.source.as_ref().expect("source not found");

        let (byte_offset_str, byte_len_str) = src_location.split_once(':').unwrap();
        let byte_offset: usize = byte_offset_str.parse().unwrap();
        let byte_length: usize = byte_len_str.parse().unwrap();

        let code_snippet = &source_content[byte_offset..byte_offset + byte_length];
        code_snippet.to_owned()
    }
}
