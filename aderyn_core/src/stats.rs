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

/// Key used for per file stats (like ignore lines).
///
/// Both the writer (`make_context` in aderyn_driver) and the reader (`detect_issues`) must build
/// keys the same way, otherwise lookups miss. `dunce` gives a plain `C:\...` path on Windows
/// instead of the `\\?\` form, and falls back to the raw path if canonicalize fails.
pub fn normalized_path_key(path: &Path) -> String {
    dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()).to_string_lossy().to_string()
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

#[cfg(test)]
mod normalized_path_key_tests {
    use super::normalized_path_key;
    use std::path::Path;

    #[test]
    fn same_file_gives_same_key() {
        let dir = std::env::temp_dir();
        let file = dir.join("aderyn_norm_key_test.sol");
        std::fs::write(&file, "").unwrap();
        let messy = dir.join(".").join("aderyn_norm_key_test.sol");
        assert_eq!(normalized_path_key(&file), normalized_path_key(&messy));
        std::fs::remove_file(&file).unwrap();
    }

    #[test]
    fn missing_file_falls_back_to_raw_path() {
        let p = Path::new("does/not/exist.sol");
        assert_eq!(normalized_path_key(p), p.to_string_lossy());
    }
}
