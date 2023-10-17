use aderyn::{framework::foundry::load_foundry, run};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Foundry project root directory
    #[arg(short, long)]
    root: String,
}

fn main() {
    let args = Args::parse();
    print!("Running Foundry with args: {:?}", args);
    let foundry_root_path = PathBuf::from(&args.root);
    println!("Foundry root path: {:?}", foundry_root_path);

    let file_paths = load_foundry(foundry_root_path).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading Foundry Root");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });

    run(file_paths).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error running aderyn");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}
