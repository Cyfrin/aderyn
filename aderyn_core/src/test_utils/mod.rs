mod load_source_unit;

use std::path::PathBuf;

// Using `solc` to read AST given a source unit (i.e Solidity file)
pub use load_source_unit::{
    load_multiple_solidity_source_units_into_single_context, load_solidity_source_unit,
};

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
        eprintln!("Please make sure {} represents a solidity file!", filepath.to_string_lossy());
        std::process::exit(1);
    }

    std::fs::canonicalize(filepath).unwrap()
}
