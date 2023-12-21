use super::loader::ContextLoader;
use crate::ast::NodeID;
use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

mod node_locator;

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
