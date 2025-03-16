pub(crate) mod config_helpers;
pub mod driver;
pub mod lsp_report;
pub(crate) mod process_auto;
pub(crate) mod project_compiler_tests;
use std::path::{Path, PathBuf};

pub use aderyn_core::{ast as core_ast, context, detect as detection_modules, detect::detector};
pub use process_auto::with_project_root_at;

fn ensure_valid_root_path(root_path: &Path) -> PathBuf {
    if !root_path.exists() {
        eprintln!("{} does not exist!", root_path.to_string_lossy());
        std::process::exit(1);
    }
    std::fs::canonicalize(root_path).unwrap()
}
