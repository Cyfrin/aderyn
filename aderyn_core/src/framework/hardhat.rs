use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

use crate::ast::SourceUnit;

#[derive(Debug, Deserialize)]
pub struct HardhatOutput {
    pub output: Output,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub sources: HashMap<String, ContractSource>,
}

#[derive(Debug, Deserialize)]
pub struct ContractSource {
    pub ast: SourceUnit,
}

pub fn load_hardhat(hardhat_root: &Path) -> Result<HardhatOutput, Box<dyn Error>> {
    let config_path = hardhat_root.join("artifacts/build-info");
    let json_build_files = collect_json_files(config_path).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error getting Hardhat build-info files. Try compiling your contracts with Hardhat before running aderyn.");
        // print err
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
    // print the found files
    println!(
        "Loading Hardhat build-info file: {:?}",
        json_build_files.get(0).unwrap()
    );

    Ok(
        read_hardhat_build_info_file(json_build_files.get(0).unwrap()).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error reading Hardhat build-info file");
            // print err
            eprintln!("{:?}", err);
            std::process::exit(1);
        }),
    )
}

fn collect_json_files(dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut json_files: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            json_files.append(&mut collect_json_files(path)?);
        } else {
            json_files.push(path);
        }
    }
    Ok(json_files)
}

pub fn read_hardhat_build_info_file<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<HardhatOutput, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let output: HardhatOutput = serde_json::from_reader(reader)?;
    Ok(output)
}
