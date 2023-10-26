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
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Foundry mode
    #[arg(long, group = "mode", conflicts_with = "hardhat")]
    foundry: bool,

    /// Hardhat mode
    #[arg(long, group = "mode", conflicts_with = "foundry")]
    hardhat: bool,

    /// Foundry project root directory
    #[arg(short, long)]
    root: String,
}

fn main() {
    let args = Args::parse();

    if !&args.foundry && !&args.hardhat {
        eprintln!("Error: Must specify either --foundry or --hardhat");
        std::process::exit(1);
    }

    let root_path = PathBuf::from(&args.root);

    let mut context_loader = ContextLoader::default();
    // TODO: move much of this gutsy stuff into the foundry / hardhat modules.
    if args.foundry {
        println!("Foundry mode");
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
    } else if args.hardhat {
        println!("Hardhat mode");
        println!("Hardhat root path: {:?}", root_path);
        let hardhat_output = load_hardhat(root_path).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error loading Hardhat build info");
            eprintln!("{:?}", err);
            std::process::exit(1);
        });
        for contract_source in hardhat_output.output.sources.values() {
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

    run(context_loader).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error running aderyn");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}
