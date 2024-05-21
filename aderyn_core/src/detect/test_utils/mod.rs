mod load_source_unit;

use cyfrin_foundry_compilers::utils;
use once_cell::sync::Lazy;
use std::path::PathBuf;

// Using `solc` to read AST given a source unit (i.e Solidity file)
pub use load_source_unit::load_solidity_source_unit;

pub(crate) fn take_loader_lock() -> impl Drop {
    static LOCK: Lazy<std::sync::Mutex<()>> = Lazy::new(|| std::sync::Mutex::new(()));
    LOCK.lock().unwrap()
}

fn ensure_valid_solidity_file(filepath: &str) -> PathBuf {
    let filepath = PathBuf::from(filepath);

    if !filepath.exists() {
        eprintln!("{} does not exist!", filepath.to_string_lossy());
        std::process::exit(1);
    }

    let extension = filepath.extension().unwrap_or_else(|| {
        eprintln!("{} is not a solidity file!", filepath.to_string_lossy());
        std::process::exit(1);
    });

    if extension != "sol" {
        eprintln!(
            "Please make sure {} represents a solidity file!",
            filepath.to_string_lossy()
        );
        std::process::exit(1);
    }

    utils::canonicalize(filepath).unwrap()
}
