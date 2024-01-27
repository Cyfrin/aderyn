use serde::Deserialize;
use std::path::PathBuf;

use aderyn_driver::{
    detector::{get_all_detectors_ids, get_detector_by_id, Detector},
    driver::{self, Args},
};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Foundry or Hardhat project root directory (or path to single solidity file)
    #[arg(default_value = ".")]
    root: String,

    /// Desired file path for the final report (will overwrite existing one)
    #[arg(short, long, default_value = "report.md")]
    output: String,

    /// List of path strings to include, delimited by comma (no spaces).
    /// Any solidity file path not containing these strings will be ignored
    #[clap(short, long, use_value_delimiter = true)]
    scope: Option<Vec<String>>,

    /// List of path strings to exclude, delimited by comma (no spaces).
    /// Any solidity file path containing these strings will be ignored
    #[clap(short, long, use_value_delimiter = true)]
    exclude: Option<Vec<String>>,

    /// Do not include code snippets in the report (reduces report size in large repos)
    #[arg(short, long)]
    no_snippets: bool,

    /// Path to aderyn.config.json
    #[arg(short, long)]
    config_file: Option<String>,

    #[clap(subcommand, name = "registry")]
    registry: Option<RegistryCommand>,
}

#[derive(Debug, Subcommand)]
enum RegistryCommand {
    /// Browse detector registry
    Registry {
        /// all - View all available detectors
        ///
        /// <id> - Detail view of a single detector
        #[arg(default_value = "all")]
        detector: String,
    },
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    if let Some(reg) = cmd_args.registry {
        match reg {
            RegistryCommand::Registry { detector } => {
                if detector == "all" {
                    print_all_detectors_view();
                } else {
                    print_detail_view(&detector);
                }
            }
        }
        return;
    }

    let args: Args = Args {
        root: cmd_args.root,
        output: cmd_args.output,
        scope: cmd_args.scope,
        exclude: cmd_args.exclude,
        no_snippets: cmd_args.no_snippets,
    };

    let aderyn_config_path = match cmd_args.config_file {
        Some(f) => PathBuf::from(f),
        None => {
            let mut project_config_json = PathBuf::from(&args.root);
            project_config_json.push("aderyn.config.json");
            project_config_json
        }
    };

    if aderyn_config_path.exists() && aderyn_config_path.is_file() {
        let config_contents = std::fs::read_to_string(aderyn_config_path).unwrap();
        let aderyn_config: Result<AderynConfig, _> = serde_json::from_str(&config_contents);
        match aderyn_config {
            Ok(config) => {
                let all_detector_ids = get_all_detectors_ids();
                let mut subscriptions: Vec<Box<dyn Detector>> = vec![];
                for detector_id in config.detectors.split(',') {
                    if !all_detector_ids.contains(&detector_id.to_string()) {
                        println!(
                            "Couldn't recognize detector with ID {} in aderyn.config.json",
                            detector_id
                        );
                        return;
                    }
                    let det = get_detector_by_id(detector_id);
                    subscriptions.push(det);
                }
                driver::drive_with(args, subscriptions);
            }
            Err(_e) => {
                println!("aderyn.config.json wasn't formatted properly!");
            }
        }
    } else {
        driver::drive(args);
    }
}

#[derive(Deserialize)]
struct AderynConfig {
    /// Detector IDs separated by commas
    #[serde(rename = "use_detectors")]
    detectors: String,
}

fn print_detail_view(detector_id: &str) {
    let all_detector_ids = get_all_detectors_ids();
    if !all_detector_ids.contains(&detector_id.to_string()) {
        println!("Couldn't recognize detector with ID {}", detector_id);
        return;
    }
    let detector = get_detector_by_id(detector_id);
    println!("\nDetector {}", detector_id);
    println!();
    println!("Title");
    println!("{}", detector.title());
    println!();
    println!("Severity");
    println!("{}", detector.severity());
    println!();
    println!("Description");
    println!("{}", detector.description());
    println!();
}

fn print_all_detectors_view() {
    let all_detector_ids = get_all_detectors_ids();

    println!("\nDetector Registry");
    println!();
    println!("{}   Title", right_pad("ID", 30));
    println!();
    for id in all_detector_ids {
        let detector = get_detector_by_id(&id);
        println!("{} - {}", right_pad(&id, 30), detector.title());
    }
    println!();
}

fn right_pad(s: &str, by: usize) -> String {
    if s.len() > by {
        return s.to_string();
    }
    let extra_spaces = by - s.len();
    let spaces = " ".repeat(extra_spaces);
    let mut new_string = s.to_string();
    new_string.push_str(&spaces);
    new_string
}
