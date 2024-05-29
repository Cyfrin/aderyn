pub(crate) mod config_helpers;
pub mod driver;
pub(crate) mod foundry_compiler_helpers;
pub(crate) mod process_auto;
pub(crate) mod project_compiler_tests;
use std::path::Path;
use std::path::PathBuf;

pub use aderyn_core::ast as core_ast;
pub use aderyn_core::context;
pub use aderyn_core::detect as detection_modules;
pub use aderyn_core::detect::detector;
use cyfrin_foundry_compilers::utils;
pub use foundry_compiler_helpers::*;
pub use process_auto::with_project_root_at;

fn ensure_valid_root_path(root_path: &Path) -> PathBuf {
    if !root_path.exists() {
        eprintln!("{} does not exist!", root_path.to_string_lossy());
        std::process::exit(1);
    }
    utils::canonicalize(root_path).unwrap()
}

fn passes_src(src: &Option<Vec<PathBuf>>, solidity_file: &Path) -> bool {
    if let Some(sources) = src {
        return sources.iter().any(|s| solidity_file.starts_with(s));
    }
    true
}

fn passes_scope(
    scope: &Option<Vec<String>>,
    solidity_file: &Path,
    absolute_root_path_str: &str,
) -> bool {
    let window = solidity_file.strip_prefix(absolute_root_path_str).unwrap();
    let window_string = window.to_string_lossy().to_string();

    if let Some(scope) = scope {
        for include in scope {
            if window_string.contains(include) {
                return true;
            }
        }
        return false;
    }

    true
}

fn passes_exclude(
    exclude: &Option<Vec<String>>,
    solidity_file: &Path,
    absolute_root_path_str: &str,
) -> bool {
    let window = solidity_file.strip_prefix(absolute_root_path_str).unwrap();
    let window_string = window.to_string_lossy().to_string();

    if let Some(exclude) = exclude {
        for dont_include in exclude {
            if window_string.contains(dont_include) {
                return false;
            }
        }
        return true;
    }

    true
}

// Return a list of remappings in the format ["a=b", "c=d", "e=f"]
// where direct imports a,c,e map to b,d,f
fn read_remappings(root_path: &Path) -> Option<Vec<String>> {
    // Look for a file called `remappings` in the project root. If not present, assume project doesn't require remappings
    let remappings_file = root_path.join("remappings");
    let remappings_txt_file = root_path.join("remappings.txt");

    let r = {
        if remappings_file.exists() {
            remappings_file
        } else if remappings_txt_file.exists() {
            remappings_txt_file
        } else {
            return None;
        }
    };

    // .unwrap_or(root_path.join("remappings.txt").canonicalize().ok()?);
    let remappings_content = std::fs::read_to_string(r.canonicalize().unwrap()).unwrap();
    Some(
        remappings_content
            .lines()
            .filter_map(|x| {
                if !x.is_empty() {
                    Some(x.to_owned())
                } else {
                    None
                }
            })
            .collect(),
    )
}
