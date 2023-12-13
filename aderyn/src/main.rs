use aderyn::{process_foundry, process_hardhat, virtual_foundry};
use aderyn_core::run;
use clap::Parser;
use std::{fs::read_dir, path::PathBuf};
use tokei::{Config, LanguageType};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Foundry or Hardhat project root directory (or path to single solidity file)
    root: String,

    /// Desired file path for the final report (will overwrite existing one)
    #[arg(short, long, default_value = "report.md")]
    output: String,
}

enum Framework {
    Foundry,
    Hardhat,
}

fn main() {
    let args = Args::parse();

    if !args.output.ends_with(".md") {
        eprintln!("Warning: output file lacks the \".md\" extension in its filename.");
    }

    let is_single_file = args.root.ends_with(".sol") && PathBuf::from(&args.root).is_file();
    let mut safe_space = PathBuf::new();

    let (src_path, mut context_loader) = {
        if is_single_file {
            safe_space = virtual_foundry::build_isolated_workspace_for_file(&args.root);
            process_foundry::with_project_root_at(&safe_space)
        } else {
            println!("Detecting framework...");
            let root_path = PathBuf::from(&args.root);
            let framework = detect_framework(root_path.clone()).unwrap_or_else(|| {
                // Exit with a non-zero exit code
                eprintln!("Error detecting framework");
                std::process::exit(1);
            });

            // This whole block loads the solidity files and ASTs into the context loader
            // TODO: move much of this gutsy stuff into the foundry / hardhat modules.
            match framework {
                Framework::Foundry => process_foundry::with_project_root_at(&root_path),
                Framework::Hardhat => process_hardhat::with_project_root_at(&root_path),
            }
        }
    };

    // Using the source path, get the sloc from tokei
    let mut languages = tokei::Languages::new();
    let tokei_config = Config::default();
    languages.get_statistics(&[src_path], &[], &tokei_config);
    context_loader.set_sloc_stats(languages[&LanguageType::Solidity].clone());

    // Load the context loader into the run function, which runs the detectors
    run(context_loader, args.output).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error running aderyn");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });

    if is_single_file {
        virtual_foundry::delete_safe_space(&safe_space);
    }
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
