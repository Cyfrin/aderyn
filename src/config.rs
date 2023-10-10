use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use eyre::Result;
use std::env;
use std::error::Error;

use crate::compiler::compiler::FoundryOutput;
use crate::detector::delegate_call_in_loop::DelegateCallInLoopDetector;
use crate::detector::detector::Detector;
use crate::loader::loader::ContractLoader;
use crate::visitor::ast_visitor::Node;

pub struct Config {
    pub contract_names: Vec<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the program name
        let contract_names: Vec<String> = args.collect();
        if contract_names.len() < 1 {
            return Err("not enough arguments");
        }
        Ok(Config { contract_names })
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/contract-playground/out");
    let subdirs = get_subdirectories(&path)?;

    let config = Config::build(env::args()).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        std::process::exit(1);
    });

    let matching_filepaths = get_matching_filepaths(&subdirs, &config.contract_names);
    println!("Loading foundry output files: {:?}", matching_filepaths);

    let mut contract_loader = ContractLoader::default();

    for filepath in matching_filepaths {
        let foundry_output = read_foundry_output_file(filepath.to_str().unwrap())?;
        foundry_output.ast.accept(&mut contract_loader)?;
    }

    let mut first_detector = DelegateCallInLoopDetector::default();
    first_detector.detect(&contract_loader)?;

    Ok(())
}

pub fn get_subdirectories(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            dirs.push(entry.path());
        }
    }
    Ok(dirs)
}

pub fn get_matching_filepaths(subdirs: &[PathBuf], contract_names: &[String]) -> Vec<PathBuf> {
    let mut matching_filepaths = Vec::new();

    for subdir in subdirs {
        for contract_name in contract_names {
            // Check if subdir string representation contains the contract name with ".sol"
            if let Some(subdir_str) = subdir.to_str() {
                if subdir_str.contains(&format!("{}.sol", contract_name)) {
                    // Construct the JSON file path and add it to matching_filepaths
                    let json_path = subdir.join(format!("{}.json", contract_name));
                    matching_filepaths.push(json_path);
                }
            }
        }
    }

    matching_filepaths
}

pub fn read_foundry_output_file(filepath: &str) -> Result<FoundryOutput> {
    Ok(serde_json::from_reader(BufReader::new(
        File::open(filepath)?,
    ))?)
}