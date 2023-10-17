use aderyn::{compiler::foundry::read_config, run};
use clap::Parser;
use eyre::Result;
use std::fs::canonicalize;
use std::{fs::read_dir, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Foundry project root directory
    #[arg(short, long)]
    root: String,

    /// Contract file names, space separated (e.g. "MyContract.sol")
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    contract_files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    print!("Running Foundry with args: {:?}", args);
    let foundry_root_path = PathBuf::from(&args.root);
    println!("Foundry root path: {:?}", foundry_root_path);
    let foundry_root_absolute = canonicalize(foundry_root_path).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error getting absolute path of Foundry root directory");
        // print err
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
    let foundry_config_filepath = foundry_root_absolute.join("foundry.toml");
    let foundry_config = read_config(&foundry_config_filepath).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error reading Foundry config file");
        std::process::exit(1);
    });

    // 1. Get the file names of all contracts in the Foundry src directory
    let foundry_src_path = foundry_root_absolute.join(&foundry_config.profile.default.src);
    let contract_files = collect_sol_files(&foundry_src_path).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error collecting Solidity files from Foundry src directory");
        std::process::exit(1);
    });
    // print the found files
    println!("Foundry src files: {:?}", contract_files);

    // 2. For each contract in the Foundry output directory, check if it is in the list of contracts in the Foundry src directory
    // (This is because some contracts may be imported but not deployed, or there may be old contracts in the output directory)
    let foundry_out_path = foundry_root_absolute.join(&foundry_config.profile.default.out);
    let file_paths = get_filepaths(foundry_out_path, &contract_files);
    // print the found files
    println!("Foundry output files: {:?}", file_paths);

    run(file_paths).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error running aderyn");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
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

    let matching_filepaths = get_matching_filepaths(&subdirs, &contract_files);
    println!("Loading foundry output files: {:?}", matching_filepaths);

    matching_filepaths
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
                if subdir_str.contains(&format!("{}", contract_name)) {
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
