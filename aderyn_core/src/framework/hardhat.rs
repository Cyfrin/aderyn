use eyre::Result;
use rayon::iter::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::ast::SourceUnit;

pub struct CumulativeHardhatOutput {
    pub output: HashMap<String, ContractSource>,
}

#[derive(Debug, Deserialize)]
pub struct HardhatOutput {
    pub output: Output,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub sources: HashMap<String, ContractSource>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractSource {
    pub ast: SourceUnit,
}

pub fn load_hardhat(hardhat_root: &Path) -> Result<CumulativeHardhatOutput, Box<dyn Error>> {
    let config_path = hardhat_root.join("artifacts/build-info");
    let json_build_files = collect_json_files(config_path).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error getting Hardhat build-info files. Try compiling your contracts with Hardhat before running aderyn.");
        // print err
        eprintln!("{:?}", err);
        std::process::exit(1);
    });

    let mut cumulative_output = CumulativeHardhatOutput {
        output: HashMap::new(),
    };
    let cumulative_output_mutex = Mutex::new(cumulative_output);

    json_build_files.par_iter().for_each(|json_build_file| {
        match read_hardhat_build_info_file(json_build_file) {
            Ok(hardhat_output) => {
                let mut cumulative_output = cumulative_output_mutex.lock().unwrap();
                for (key, contract_source) in hardhat_output.output.sources.iter() {
                    if key.starts_with("contracts/") {
                        cumulative_output
                            .output
                            .insert(key.to_string(), contract_source.clone());
                    }
                }
            }
            Err(err) => {
                eprintln!(
                    "Error reading Hardhat build-info file {:?}: {:?}",
                    json_build_file, err
                );
            }
        }
    });
    // Retrieve the cumulative_output after the parallel processing
    cumulative_output = cumulative_output_mutex.into_inner().unwrap();

    Ok(cumulative_output)
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
