use std::{collections::BTreeMap, error::Error};

use crate::ast::{ASTNode, NodeID, NodeType};

use crate::{
    capture,
    context::{
        browser::{ExtractReferencedDeclarationsConditionally, GetClosestAncestorOfTypeX},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

use self::source_unit_graph_analysis::Graph;

#[derive(Default)]
pub struct UnusedImportDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnusedImportDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut graph = Graph::new();

        for source_unit in context.source_units() {
            for imported_source_unit in source_unit.import_directives() {
                let imported_symbols = imported_source_unit
                    .symbol_aliases
                    .iter()
                    .flat_map(|s| s.foreign.referenced_declaration)
                    .collect::<Vec<_>>();

                let imported_source_unit_id = imported_source_unit.source_unit;

                if imported_symbols.is_empty() {
                    // This means it's not a named import, so technically we're importing
                    // everything that the source unit exports
                    if let Some(ASTNode::SourceUnit(i)) =
                        context.nodes.get(&imported_source_unit_id)
                    {
                        if let Some(exported_symbols) = i.exported_symbols.as_ref() {
                            let exported_symbols =
                                exported_symbols.values().flatten().collect::<Vec<_>>();
                            // Create a relationship from source_unit -> imported source unit FOR
                            // ALL exported_symbols

                            graph.create_relationship_for_symbols(
                                source_unit.id,
                                imported_source_unit_id,
                                imported_source_unit.id,
                                exported_symbols.into_iter().cloned().collect::<Vec<_>>(),
                            );
                        }
                    }
                } else {
                    // This is a names import and we're only importing specific symbols
                    graph.create_relationship_for_symbols(
                        source_unit.id,
                        imported_source_unit_id,
                        imported_source_unit.id,
                        imported_symbols,
                    );
                }
            }
        }

        for source_unit in context.source_units() {
            let referenced_declarations = ExtractReferencedDeclarationsConditionally::from(
                source_unit,
                context,
                Box::new(|node_id, context| {
                    if let Some(node) = context.nodes.get(&node_id) {
                        return node
                            .closest_ancestor_of_type(context, NodeType::ImportDirective)
                            .is_none();
                    }
                    false
                }),
            )
            .extracted;

            for referenced_symbol in referenced_declarations {
                graph.mark_used_pathways(source_unit.id, referenced_symbol);
                if let Some(symbol_place) = context.nodes.get(&referenced_symbol) {
                    if let Some(ASTNode::ContractDefinition(contract)) =
                        symbol_place.closest_ancestor_of_type(context, NodeType::ContractDefinition)
                    {
                        graph.mark_used_pathways(source_unit.id, contract.id);
                    }
                }
            }
        }

        for unused_import_id in graph.collect_unused_imports() {
            if let Some(node) = context.nodes.get(&unused_import_id) {
                capture!(self, context, node);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Unused Import")
    }

    fn description(&self) -> String {
        String::from("Redundant import statement. Consider removing it.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnusedImport)
    }
}

mod source_unit_graph_analysis {

    //! Goal of this module is to create a graphical representation of all the source units
    //! connected  with import statements.
    //!
    //! Will be used to detect unused imports

    use std::collections::{HashMap, HashSet};

    use crate::ast::NodeID;

    #[derive(Default, Debug)]
    pub struct GNode {
        #[allow(dead_code)]
        source_unit: NodeID,
        edges: Vec<GEdge>,
    }

    #[derive(Default, Debug)]
    pub struct GEdge {
        symbols: Vec<NodeID>,
        to: NodeID,
        import_statement: NodeID,
    }

    #[derive(Default, Debug)]
    pub struct Graph {
        /// Key source units: source unit ID
        source_units: HashMap<NodeID, GNode>,

        /// Key: Import Statement, Value: Symbol that is imported
        useful_symbols: HashMap<NodeID, Vec<NodeID>>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        /// Each relationship edge corresponds to an import statement in `from` source unit that
        /// imports symbols `symbols` from `to` source unit
        pub fn create_relationship_for_symbols(
            &mut self,
            from_source_unit: NodeID,
            to_source_unit: NodeID,
            import_statement: NodeID,
            symbols: Vec<NodeID>,
        ) {
            // First we try to find the GNode with corresponding to the source unit. If we do we
            // return it's index in the graph otherwise, we insert a GNode

            let from_node = self
                .source_units
                .entry(from_source_unit)
                .or_insert_with(|| GNode { source_unit: to_source_unit, edges: vec![] });

            // Create the relationship edge
            let relationship = GEdge { symbols, to: to_source_unit, import_statement };

            from_node.edges.push(relationship);

            // Make sure the `to` source unit node is present in the graph
            _ = self
                .source_units
                .entry(to_source_unit)
                .or_insert_with(|| GNode { source_unit: to_source_unit, edges: vec![] });
        }

        pub fn mark_used_pathways(
            &mut self,
            source_unit_id: NodeID,
            symbol_id: NodeID,
        ) -> Option<()> {
            let mut visited_source_unit_ids = HashSet::new();
            self.dfs(source_unit_id, symbol_id, &mut visited_source_unit_ids)?;
            Some(())
        }

        fn dfs(
            &mut self,
            source_unit: NodeID,
            symbol_id: NodeID,
            visited: &mut HashSet<NodeID>,
        ) -> Option<()> {
            if visited.contains(&source_unit) {
                return Some(());
            }

            visited.insert(source_unit);

            let s = self.source_units.get(&source_unit)?;
            let mut to_ids = vec![];

            for relationship in &s.edges {
                if relationship.symbols.contains(&symbol_id) {
                    self.useful_symbols
                        .entry(relationship.import_statement)
                        .or_default()
                        .push(symbol_id);
                    to_ids.push(relationship.to);
                }
            }

            for to in to_ids {
                self.dfs(to, symbol_id, visited)?;
            }

            Some(())
        }

        pub fn collect_unused_imports(&mut self) -> Vec<NodeID> {
            let mut useless_imports: Vec<_> = vec![];

            for node in self.source_units.values() {
                for relationship in &node.edges {
                    if !self.useful_symbols.contains_key(&relationship.import_statement) {
                        useless_imports.push(relationship.import_statement);
                    }
                }
            }

            useless_imports
        }
    }
}

#[cfg(test)]
mod unused_imports_tests {
    use semver::Version;

    use crate::detect::{detector::IssueDetector, low::unused_import::UnusedImportDetector};

    #[test]

    fn test_unused_imports() {
        let context =
            crate::detect::test_utils::load_multiple_solidity_source_units_into_single_context(
                &[
                    "../tests/contract-playground/src/UnusedImport.sol",
                    "../tests/contract-playground/src/U2.sol",
                    "../tests/contract-playground/src/U3.sol",
                    "../tests/contract-playground/src/U4.sol",
                    "../tests/contract-playground/src/U5.sol",
                ],
                Version::new(0, 8, 19),
            );

        let mut detector = UnusedImportDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
