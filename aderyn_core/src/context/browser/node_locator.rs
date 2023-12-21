use super::{ContextBrowser, SourceUnitInsight};
use crate::{ast::NodeID, context::loader::ASTNode};

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
            .split(':')
            .take(1)
            .map(|x| x.parse())
            .next()
            .unwrap()
            .unwrap();

        let line = self.get_source_line(insight, ch_pos);
        (absolute_path, line)
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

        // idx is the number of newlines before ch_pos
        // Adding one to it gives the line number of the current position
        idx + 1
    }
}
