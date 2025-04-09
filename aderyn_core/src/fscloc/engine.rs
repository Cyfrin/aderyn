use crate::context::workspace_context::WorkspaceContext;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use super::cloc::{self, Stats};

pub fn count_lines_of_code_and_collect_line_numbers_to_ignore(
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
            let stats = cloc::get_stats(content, skip_cloc);
            let full_path = root.join(&path).to_string_lossy().to_string();
            Some((full_path, stats))
        })
        .flatten()
        .collect()
}
