mod display;

pub mod compile;
pub mod config;
pub mod driver;
pub mod lsp_report;

use std::path::{Path, PathBuf};

pub use aderyn_core::{ast as core_ast, context, detect as detection_modules, detect::detector};

fn ensure_valid_root_path(root_path: &Path) -> PathBuf {
    if !root_path.exists() {
        eprintln!("{} does not exist!", root_path.to_string_lossy());
        std::process::exit(1);
    }
    std::fs::canonicalize(root_path).unwrap()
}
