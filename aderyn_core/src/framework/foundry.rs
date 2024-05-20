use crate::ast::*;
use cyfrin_foundry_compilers::utils;
use eyre::Result;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::{read_dir, read_to_string, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Stdio;
use toml::Table;

// Foundry compiler output file
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FoundryOutput {
    pub ast: SourceUnit,
}

// Foundry TOML config file (according to relevant profile)
#[derive(Debug, Deserialize)]
struct FoundryConfig {
    src: String,
    out: String,
}

pub fn read_foundry_output_file(filepath: &str) -> Result<FoundryOutput> {
    Ok(serde_json::from_reader(BufReader::new(File::open(
        filepath,
    )?))?)
}

#[derive(Debug)]
pub struct LoadedFoundry {
    pub src_path: String,
    pub src_filepaths: Vec<PathBuf>,
    pub output_filepaths: Vec<PathBuf>,
}

// Load foundry and return a Vector of PathBufs to the AST JSON files
pub fn load_foundry(
    foundry_root: &PathBuf,
    skip_build: bool,
) -> Result<LoadedFoundry, Box<dyn Error>> {
    let foundry_root_absolute = utils::canonicalize(foundry_root).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error getting absolute path of Foundry root directory");
        // print err
        eprintln!("{:?}", err);
        std::process::exit(1);
    });

    if !skip_build {
        println!(
            "Running `forge build --ast` in {:?}",
            &foundry_root_absolute
        );

        // Run `forge build --ast` in the root
        let output = std::process::Command::new("forge")
            .arg("build")
            .arg("--ast")
            .current_dir(&foundry_root_absolute)
            .stdout(Stdio::inherit()) // This will stream the stdout
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to run `forge build --ast`");

        if !output.success() {
            eprintln!("The command `forge build --ast` did not execute successfully. Please run `foundryup` and try again otherwise, install foundry by following the official guide https://book.getfoundry.sh/getting-started/installation");
            std::process::exit(output.code().unwrap());
        }
    }

    let foundry_config_filepath = foundry_root_absolute.join("foundry.toml");
    let foundry_config = read_config(&foundry_config_filepath).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error reading Foundry config file");
        std::process::exit(1);
    });

    // Get the file names of all contracts in the Foundry src directory
    let foundry_src_path = foundry_root_absolute.join(&foundry_config.src);
    let contract_filepaths =
        collect_nested_files(&foundry_src_path, "sol").unwrap_or_else(|_err| {
            // Exit with a non-zero exit code
            eprintln!("Error collecting Solidity files from Foundry src directory");
            std::process::exit(1);
        });

    // For each contract in the Foundry output directory, check if it is in the list of contracts in the Foundry src directory
    // (This is because some contracts may be imported but not deployed, or there may be old contracts in the output directory)
    let foundry_out_path = foundry_root_absolute.join(&foundry_config.out);

    let json_output_filepaths = collect_nested_files(&foundry_out_path.clone(), "json")
        .unwrap_or_else(|_err| {
            // Exit with a non-zero exit code
            eprintln!("Error collecting JSON output files from Foundry output directory");
            std::process::exit(1);
        });
    let output_filepaths = get_matching_output_files(&json_output_filepaths, &contract_filepaths);

    Ok(LoadedFoundry {
        src_path: foundry_config.src,
        src_filepaths: contract_filepaths,
        output_filepaths,
    })
}

fn read_config(path: &PathBuf) -> Result<FoundryConfig, Box<dyn Error>> {
    let contents = read_to_string(path).unwrap();

    let foundry_config_toml = contents.parse::<Table>();

    let foundry_config = match foundry_config_toml {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing TOML: {:?}", e);
            std::process::exit(1);
        }
    };

    let foundry_profile = std::env::var("FOUNDRY_PROFILE")
        .unwrap_or("default".to_string())
        .to_lowercase();

    // Official Foundry docs defaults
    // https://book.getfoundry.sh/reference/config/project#src

    let default_foundry_src = std::env::var("FOUNDRY_SRC")
        .unwrap_or(std::env::var("DAPP_SRC").unwrap_or(String::from("src")));

    let default_foundry_out = std::env::var("FOUNDRY_OUT")
        .unwrap_or(std::env::var("DAPP_OUT").unwrap_or(String::from("out")));

    if let Some(foundry_profiles) = foundry_config["profile"].as_table() {
        // println!("{:#?}", foundry_profiles);

        for (profile_key, value) in foundry_profiles {
            if profile_key == &foundry_profile {
                if let Some(value) = value.as_table() {
                    let profile_src = {
                        if let Some(toml::Value::String(src)) = value.get("src") {
                            src.clone()
                        } else {
                            default_foundry_src
                        }
                    };

                    let profile_out = {
                        if let Some(toml::Value::String(out)) = value.get("out") {
                            out.clone()
                        } else {
                            default_foundry_out
                        }
                    };

                    println!("Scanning out folder - {}", profile_out);

                    return Ok(FoundryConfig {
                        src: profile_src.to_string(),
                        out: profile_out.to_string(),
                    });
                }
            }
        }
    }

    Ok(FoundryConfig {
        src: default_foundry_src,
        out: default_foundry_out,
    })
}

fn collect_nested_files(path: &PathBuf, extension: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut results = Vec::new();

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                results.extend(collect_nested_files(&entry_path, extension)?);
            } else if entry_path.extension().map_or(false, |ext| ext == extension) {
                results.push(entry_path);
            }
        }
    } else if path.extension().map_or(false, |ext| ext == extension) {
        results.push(path.clone());
    }

    Ok(results)
}

fn get_matching_output_files(
    json_output_filepaths: &[PathBuf],
    src_filepaths: &[PathBuf],
) -> Vec<PathBuf> {
    json_output_filepaths
        .iter()
        .filter(|output_filepath| {
            src_filepaths.iter().any(|src_filepath| {
                let contract_name = src_filepath.file_name().unwrap().to_str().unwrap();
                output_filepath
                    .to_str()
                    .map_or(false, |s| s.contains(contract_name))
            })
        })
        .cloned()
        .collect()
}
