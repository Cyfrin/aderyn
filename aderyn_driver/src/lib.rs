pub mod driver;
pub(crate) mod process_auto;
use std::path::Path;
use std::path::PathBuf;

pub use aderyn_core::ast as core_ast;
pub use aderyn_core::context;
pub use aderyn_core::detect as detection_modules;
pub use aderyn_core::detect::detector;

fn ensure_valid_root_path(root_path: &Path) -> PathBuf {
    if !root_path.exists() {
        eprintln!("{} does not exist!", root_path.to_string_lossy());
        std::process::exit(1);
    }
    root_path.canonicalize().unwrap()
}
