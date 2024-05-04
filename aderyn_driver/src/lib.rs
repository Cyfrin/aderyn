pub mod driver;
pub(crate) mod process_auto;
pub(crate) mod project_compiler_tests;
use std::path::Path;
use std::path::PathBuf;

pub use aderyn_core::ast as core_ast;
pub use aderyn_core::context;
pub use aderyn_core::detect as detection_modules;
pub use aderyn_core::detect::detector;
pub use process_auto::with_project_root_at;

fn ensure_valid_root_path(root_path: &Path) -> PathBuf {
    if !root_path.exists() {
        eprintln!("{} does not exist!", root_path.to_string_lossy());
        std::process::exit(1);
    }
    root_path.canonicalize().unwrap()
}

fn passes_scope(
    scope: &Option<Vec<String>>,
    solidity_file: &Path,
    absolute_root_path_str: &str,
) -> bool {
    let sol_path_string = solidity_file.to_string_lossy().to_string();
    let pos = sol_path_string.find(absolute_root_path_str).unwrap();
    let window = &sol_path_string[pos + absolute_root_path_str.len()..];

    if let Some(scope) = scope {
        for include in scope {
            if window.contains(include) {
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
    let sol_path_string = solidity_file.to_string_lossy().to_string();
    let pos = sol_path_string.find(absolute_root_path_str).unwrap();
    let window = &sol_path_string[pos + absolute_root_path_str.len()..];

    if let Some(exclude) = exclude {
        for dont_include in exclude {
            if window.contains(dont_include) {
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
    let remappings_file = root_path.join("remappings").canonicalize().ok()?;
    // .unwrap_or(root_path.join("remappings.txt").canonicalize().ok()?);
    let remappings_content = std::fs::read_to_string(remappings_file).unwrap();
    Some(remappings_content.lines().map(|x| x.to_owned()).collect())
}
