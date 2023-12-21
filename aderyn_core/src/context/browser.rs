use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::ast::NodeID;

use std::collections::HashMap;

use super::loader::{ASTNode, ContextLoader};

pub struct ContextBrowser<'a> {
    insights: HashMap<NodeID, SourceUnitInsight>,
    loader: &'a ContextLoader,
}

impl<'a> ContextBrowser<'a> {
    pub fn default_from(loader: &'a ContextLoader) -> Self {
        ContextBrowser {
            insights: HashMap::new(),
            loader,
        }
    }

    // populate insights
    pub fn build_parallel(&mut self) {
        let source_units = &self.loader.source_units;
        let insights = source_units.par_iter().map(|src_unit| {
            let id = src_unit.id;
            let mut newline_char_indices = vec![];
            if let Some(s) = src_unit.source.as_deref() {
                for (idx, ch) in s.chars().enumerate() {
                    if ch == '\n' {
                        newline_char_indices.push(idx);
                    }
                }
                (
                    id,
                    SourceUnitInsight {
                        newline_char_indices,
                        absent_source: false,
                    },
                )
            } else {
                (
                    id,
                    SourceUnitInsight {
                        newline_char_indices,
                        absent_source: true,
                    },
                )
            }
        });

        self.insights = HashMap::from_par_iter(insights);
    }
}

pub struct SourceUnitInsight {
    newline_char_indices: Vec<usize>,
    absent_source: bool,
}

impl<'a> ContextBrowser<'a> {
    pub fn get_node_sort_key(&self, node: &ASTNode) -> (String, usize) {
        let source_unit = self.loader.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();

        let node_id: NodeID = source_unit.id;
        let insight = self.insights.get(&node_id).unwrap();

        if insight.absent_source || node.src().is_none() {
            return (absolute_path, 0);
        }

        let src = node.src().unwrap();
        let ch_pos: usize = src
            .split(":")
            .take(1)
            .map(|x| x.parse())
            .next()
            .unwrap()
            .unwrap();

        assert!(ch_pos < src.len());

        let line = self.get_source_line(insight, ch_pos);
        return (absolute_path, line);
    }

    fn get_source_line(&self, insight: &SourceUnitInsight, ch_pos: usize) -> usize {
        // edge cases for binary search
        if insight.newline_char_indices.is_empty() || ch_pos < insight.newline_char_indices[0] {
            return 1;
        }

        /*
            Example:
            newline_char_indices = [1, 5, 10, 24, 40, 50, 61]
            |==> This will always be sorted in ascending order
            |==> Given a character's position you can easily find
            |==> its corresponding line with binary search.
        */

        let idx = insight
            .newline_char_indices
            .partition_point(|x| x < &ch_pos);

        // idx + 1 is the number of newlines before ch_pos
        // Adding one to it gives the line number of the current position
        idx + 2
    }
}
