use crate::ast::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{canonicalize, read_dir, read_to_string, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Stdio;

// Foundry compiler output file
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FoundryOutput {
    pub ast: SourceUnit,
}

// Foundry TOML config file
#[derive(Debug, Deserialize)]
struct FoundryConfig {
    profile: ProfileSection,
}

#[derive(Debug, Deserialize)]
struct ProfileSection {
    #[serde(rename = "default")]
    default: DefaultProfile,
}

#[derive(Debug, Deserialize)]
struct DefaultProfile {
    #[serde(default = "default_src")]
    src: String,
    #[serde(default = "default_out")]
    out: String,
}

fn default_src() -> String {
    "src".to_string()
}

fn default_out() -> String {
    "out".to_string()
}

pub fn read_foundry_output_file(filepath: &str) -> Result<FoundryOutput> {
    Ok(serde_json::from_reader(BufReader::new(File::open(
        filepath,
    )?))?)
}

// Load foundry and return a Vector of PathBufs to the AST JSON files
pub fn load_foundry(foundry_root: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let foundry_root_absolute = canonicalize(foundry_root).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error getting absolute path of Foundry root directory");
        // print err
        eprintln!("{:?}", err);
        std::process::exit(1);
    });

    // Run `forge build` in the root
    let output = std::process::Command::new("forge")
        .arg("build")
        .current_dir(&foundry_root_absolute)
        .stdout(Stdio::inherit()) // This will stream the stdout
        .stderr(Stdio::inherit())
        .status();
    println!("forge build output: {:?}", output);

    let foundry_config_filepath = foundry_root_absolute.join("foundry.toml");
    let foundry_config = read_config(&foundry_config_filepath).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error reading Foundry config file");
        std::process::exit(1);
    });

    // Get the file names of all contracts in the Foundry src directory
    let foundry_src_path = foundry_root_absolute.join(&foundry_config.profile.default.src);
    let contract_files = collect_sol_files(&foundry_src_path).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error collecting Solidity files from Foundry src directory");
        std::process::exit(1);
    });
    // print the found files
    println!("Foundry src files: {:?}", contract_files);

    // For each contract in the Foundry output directory, check if it is in the list of contracts in the Foundry src directory
    // (This is because some contracts may be imported but not deployed, or there may be old contracts in the output directory)
    let foundry_out_path = foundry_root_absolute.join(&foundry_config.profile.default.out);
    let file_paths = get_filepaths(foundry_out_path, &contract_files);
    Ok(file_paths)
}

fn read_config(path: &PathBuf) -> Result<FoundryConfig, Box<dyn Error>> {
    let contents = read_to_string(path).unwrap();
    let foundry_config_toml = toml::from_str(&contents);
    let foundry_config = match foundry_config_toml {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing TOML: {:?}", e);
            std::process::exit(1);
        }
    };
    println!("Foundry config: {:?}", foundry_config);
    Ok(foundry_config)
}

fn collect_sol_files(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let mut results = Vec::new();

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                results.extend(collect_sol_files(&entry_path)?);
            } else if entry_path.extension().map_or(false, |ext| ext == "sol") {
                if let Some(filename) = entry_path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        results.push(filename_str.to_string());
                    }
                }
            }
        }
    } else if path.extension().map_or(false, |ext| ext == "sol") {
        if let Some(filename) = path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                results.push(filename_str.to_string());
            }
        }
    }

    Ok(results)
}

fn get_filepaths(foundry_out_path: PathBuf, contract_files: &Vec<String>) -> Vec<PathBuf> {
    let subdirs = get_subdirectories(&foundry_out_path).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error getting subdirectories of Foundry output directory");
        std::process::exit(1);
    });

    

    get_matching_filepaths(&subdirs, contract_files)
}

fn get_subdirectories(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            dirs.push(entry.path());
        }
    }
    Ok(dirs)
}

fn get_matching_filepaths(subdirs: &[PathBuf], contract_files: &[String]) -> Vec<PathBuf> {
    let mut matching_filepaths = Vec::new();

    for subdir in subdirs {
        for contract_name in contract_files {
            // Check if subdir string representation contains the contract name with ".sol"
            if let Some(subdir_str) = subdir.to_str() {
                if subdir_str.contains(&format!("/{}", contract_name)) {
                    // Construct the JSON file path and add it to matching_filepaths
                    let contract_name_path = PathBuf::from(contract_name);
                    if let Some(name_without_extension) = contract_name_path.file_stem() {
                        let json_path = subdir
                            .join(format!("{}.json", name_without_extension.to_str().unwrap()));
                        matching_filepaths.push(json_path);
                    }
                }
            }
        }
    }

    matching_filepaths
}
