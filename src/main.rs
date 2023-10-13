use aderyn::{compiler::foundry::read_config, run};
use clap::Parser;
use std::io::Result;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Foundry project root directory
    #[arg(short, long)]
    root: String,

    /// Contract file names (without .sol), space separated
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    contract_names: Vec<String>,
}

fn main() {
    let args = Args::parse();
    print!("Running Foundry with args: {:?}", args);

    let foundry_config_filepath = format!("{}foundry.toml", args.root);
    let foundry_config = read_config(foundry_config_filepath).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error reading Foundry config file");
        std::process::exit(1);
    });
    let foundry_out_path = format!("{}{}", args.root, foundry_config.profile.default.out);
    run(get_filepaths(foundry_out_path, &args.contract_names)).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error running aderyn");
        std::process::exit(1);
    });
}

fn get_filepaths(foundry_out_path: String, contract_names: &[String]) -> Vec<PathBuf> {
    let path = Path::new(&foundry_out_path);
    let subdirs = get_subdirectories(&path).unwrap_or_else(|_err| {
        // Exit with a non-zero exit code
        eprintln!("Error getting subdirectories of Foundry output directory");
        std::process::exit(1);
    });

    let matching_filepaths = get_matching_filepaths(&subdirs, &contract_names);
    println!("Loading foundry output files: {:?}", matching_filepaths);

    matching_filepaths
}

fn get_subdirectories(path: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            dirs.push(entry.path());
        }
    }
    Ok(dirs)
}

fn get_matching_filepaths(subdirs: &[PathBuf], contract_names: &[String]) -> Vec<PathBuf> {
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
