use crate::context::workspace::WorkspaceContext;
use cloc::count_code_lines;
use ignore::get_lines_to_ignore;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};
use token::tokenize;

pub mod cloc;
pub mod ignore;
pub mod token;
pub mod util;

#[derive(Debug)]
pub struct Stats {
    pub code: usize,
    pub ignore_lines: Vec<IgnoreLine>,
}

#[derive(Debug, Clone)]
pub enum When {
    Always,
    ForDetectorsWithNames(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct IgnoreLine {
    /// When to consider this ignore
    pub when: When,

    /// Which line number to ignore
    pub which: usize,
}

pub fn collect_stats(
    root: &Path,
    skip_cloc: bool,
    context: &WorkspaceContext,
) -> HashMap<String, Stats> {
    context
        .source_units()
        .par_iter()
        .map(|source_unit| {
            let path = source_unit.absolute_path.clone().expect("absolute path not inserted");
            let path = path.replace("//", "/"); // Condense entries that look like `contracts/templegold//AuctionBase.sol`
            if !context.included.contains(&PathBuf::from_str(&path).unwrap()) {
                return None;
            }
            let content = source_unit.source.as_ref().expect("source not filled");
            let stats = get_stats(content, skip_cloc);
            let full_path = root.join(&path).to_string_lossy().to_string();
            Some((full_path, stats))
        })
        .flatten()
        .collect()
}

pub fn get_stats(r_content: &str, skip_cloc: bool) -> Stats {
    if r_content.is_empty() {
        return Stats { code: 0, ignore_lines: vec![] };
    }

    let token_descriptors = tokenize(r_content);
    let code_lines = if skip_cloc { 0 } else { count_code_lines(&token_descriptors) };
    let ignore_lines = get_lines_to_ignore(&token_descriptors);

    Stats { code: code_lines, ignore_lines }
}
