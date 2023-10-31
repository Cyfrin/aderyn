use aderyn::{
    context::loader::ContextLoader,
    framework::{
        foundry::{load_foundry, read_foundry_output_file},
        hardhat::load_hardhat,
    },
    run,
    visitor::ast_visitor::Node,
};
use clap::Parser;
use std::{
    fs::{read_dir, File},
    io::{Read, Result},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Foundry project root directory
    #[arg(short, long)]
    root: String,
}

enum Framework {
    Foundry,
    Hardhat,
}

fn main() {
    let args = Args::parse();

    // Detect the framework
    println!("Detecting framework...");
    let root_path = PathBuf::from(&args.root);
    let framework = detect_framework(root_path.clone()).unwrap_or_else(|| {
        // Exit with a non-zero exit code
        eprintln!("Error detecting framework");
        std::process::exit(1);
    });

    let mut context_loader = ContextLoader::default();
    // TODO: move much of this gutsy stuff into the foundry / hardhat modules.
    match framework {
        Framework::Foundry => {
            println!("Framework detected: Foundry mode engaged.");
            println!("Foundry root path: {:?}", root_path);
            let loaded_foundry = load_foundry(root_path).unwrap_or_else(|err| {
                // Exit with a non-zero exit code
                eprintln!("Error loading Foundry Root");
                eprintln!("{:?}", err);
                std::process::exit(1);
            });
            // Load the foundry output files into the context loader using the ASTs
            for output_filepath in loaded_foundry.output_filepaths {
                // read_foundry_output_file and print an error message if it fails
                if let Ok(foundry_output) =
                    read_foundry_output_file(output_filepath.to_str().unwrap())
                {
                    foundry_output
                        .ast
                        .accept(&mut context_loader)
                        .unwrap_or_else(|err| {
                            // Exit with a non-zero exit code
                            eprintln!("Error loading Foundry AST into ContextLoader");
                            eprintln!("{:?}", err);
                            std::process::exit(1);
                        })
                } else {
                    eprintln!(
                        "Error reading Foundry output file: {}",
                        output_filepath.to_str().unwrap()
                    );
                }
            }
            // Load the solidity source files into memory, and assign the content to the source_unit.source
            for source_filepath in loaded_foundry.src_filepaths {
                match read_file_to_string(&source_filepath) {
                    Ok(content) => {
                        // Convert the full_path to a string
                        let full_path_str = source_filepath.to_str().unwrap_or("");

                        // Find the index where "src/" starts
                        if let Some(start_index) = full_path_str.find("src/") {
                            let target_path = &full_path_str[start_index..];

                            // Search for a match and modify
                            for unit in context_loader.get_source_units() {
                                if let Some(ref abs_path) = unit.absolute_path {
                                    if abs_path == target_path {
                                        context_loader
                                            .set_source_unit_source_content(unit.id, content);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Error reading Solidity source file: {}",
                            source_filepath.to_str().unwrap()
                        );
                        eprintln!("{:?}", err);
                    }
                }
            }
        }
        Framework::Hardhat => {
            println!("Framework detected. Hardhat mode engaged.");
            println!("Hardhat root path: {:?}", root_path);
            let hardhat_output = load_hardhat(&root_path).unwrap_or_else(|err| {
                // Exit with a non-zero exit code
                eprintln!("Error loading Hardhat build info");
                eprintln!("{:?}", err);
                std::process::exit(1);
            });
            for (key, contract_source) in hardhat_output.output.sources.iter() {
                if key.starts_with("contracts/") {
                    contract_source
                        .ast
                        .accept(&mut context_loader)
                        .unwrap_or_else(|err| {
                            // Exit with a non-zero exit code
                            eprintln!("Error loading Hardhat AST into ContextLoader");
                            eprintln!("{:?}", err);
                            std::process::exit(1);
                        });
                    let source_path = root_path.join(key);
                    match read_file_to_string(&source_path) {
                        Ok(content) => {
                            for unit in context_loader.get_source_units() {
                                if let Some(ref abs_path) = unit.absolute_path {
                                    if abs_path == key {
                                        context_loader
                                            .set_source_unit_source_content(unit.id, content);
                                        break;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!(
                                "Error reading Solidity source file: {}",
                                source_path.to_str().unwrap()
                            );
                            eprintln!("{:?}", err);
                        }
                    }
                }
            }
        }
    }

    run(context_loader).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error running aderyn");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}

fn read_file_to_string(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn detect_framework(path: PathBuf) -> Option<Framework> {
    // Canonicalize the path
    let canonical_path = path.canonicalize().expect("Failed to canonicalize path");

    // Check if the directory exists
    if !canonical_path.is_dir() {
        return None;
    }

    // Read the contents of the directory
    let entries = read_dir(&canonical_path).expect("Failed to read directory");

    for entry in entries.flatten() {
        let filename = entry.file_name();
        match filename.to_str() {
            Some("foundry.toml") => return Some(Framework::Foundry),
            Some("hardhat.config.js") | Some("hardhat.config.ts") => {
                return Some(Framework::Hardhat)
            }
            _ => {}
        }
    }

    None
}
