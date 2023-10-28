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
use std::{fs::read_dir, path::PathBuf};

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
            let file_paths = load_foundry(root_path).unwrap_or_else(|err| {
                // Exit with a non-zero exit code
                eprintln!("Error loading Foundry Root");
                eprintln!("{:?}", err);
                std::process::exit(1);
            });
            for filepath in file_paths {
                // read_foundry_output_file and print an error message if it fails
                if let Ok(foundry_output) = read_foundry_output_file(filepath.to_str().unwrap()) {
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
                        filepath.to_str().unwrap()
                    );
                }
            }
        }
        Framework::Hardhat => {
            println!("Framework detected. Hardhat mode engaged.");
            println!("Hardhat root path: {:?}", root_path);
            let hardhat_output = load_hardhat(root_path).unwrap_or_else(|err| {
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
                        })
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
