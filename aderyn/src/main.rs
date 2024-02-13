#![allow(clippy::borrowed_box)]

use serde::Deserialize;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

use aderyn_driver::{
    detector::{
        get_all_detectors_names, get_all_issue_detectors, get_issue_detector_by_name,
        IssueDetector, IssueSeverity,
    },
    driver::{self, Args},
    get_fully_configured_watchtower, WatchTower,
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
        /// all    - View all available detectors
        ///
        /// <name> - Detail view of a single detector
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
        let config_contents = std::fs::read_to_string(&aderyn_config_path).unwrap();
        let aderyn_config: Result<AderynConfig, _> = serde_json::from_str(&config_contents);
        match aderyn_config {
            Ok(config) => {
                let all_detector_names = get_all_detectors_names();

                let mut subscriptions: Vec<Box<dyn IssueDetector>> = vec![];
                let mut scope_lines: Option<Vec<String>> = args.scope.clone();
                match config.detectors {
                    Some(config_detectors) => {
                        for detector_name in &config_detectors {
                            if !all_detector_names.contains(&detector_name.to_string()) {
                                println!(
                                            "Couldn't recognize detector with name {} in aderyn.config.json",
                                            detector_name
                                        );
                                return;
                            }
                            let det = get_issue_detector_by_name(detector_name);
                            subscriptions.push(det);
                        }
                    }
                    None => {
                        subscriptions.extend(get_all_issue_detectors());
                    }
                }

                let mut altered_by_scope_in_config = false;

                if let Some(scope_in_config) = config.scope {
                    let mut found_scope_lines = vec![];
                    for scope_line in scope_in_config {
                        found_scope_lines.push(scope_line.to_string());
                    }
                    if scope_lines.is_none() {
                        // CLI should override aderyn.config.json if present
                        scope_lines = Some(found_scope_lines);
                        altered_by_scope_in_config = true
                    }
                }

                if let Some(scope_file) = config.scope_file {
                    let mut scope_file_path = aderyn_config_path.clone();
                    scope_file_path.pop();
                    scope_file_path.push(PathBuf::from(scope_file));

                    let canonicalized_scope_file_path = std::fs::canonicalize(&scope_file_path);
                    match canonicalized_scope_file_path {
                        Ok(ok_scope_file_path) => {
                            assert!(ok_scope_file_path.exists());
                            let scope_lines_in_file =
                                std::fs::read_to_string(ok_scope_file_path).unwrap();
                            let mut found_scope_lines = vec![];
                            for scope_line in scope_lines_in_file.lines() {
                                found_scope_lines.push(scope_line.to_string());
                            }
                            if scope_lines.is_none() || altered_by_scope_in_config {
                                // CLI should override aderyn.config.json if present
                                if scope_lines.is_none() {
                                    scope_lines = Some(found_scope_lines);
                                } else {
                                    let mut added_to_existing = scope_lines.unwrap();
                                    added_to_existing.extend(found_scope_lines);
                                    scope_lines = Some(added_to_existing);
                                }
                            }
                        }
                        Err(_e) => {
                            println!(
                                "Scope file doesn't exist at {:?}",
                                Path::new(&scope_file_path).as_os_str()
                            );
                            return;
                        }
                    }
                }

                let new_args: Args = Args {
                    root: args.root,
                    output: args.output,
                    scope: scope_lines,
                    exclude: args.exclude,
                    no_snippets: args.no_snippets,
                };
                driver::drive_with(new_args, subscriptions);
            }
            Err(_e) => {
                println!("aderyn.config.json wasn't formatted properly! {:?}", _e);
            }
        }
    } else {
        driver::drive(args);
    }
}

#[derive(Deserialize)]
struct AderynConfig {
    /// Detector names separated by commas
    #[serde(rename = "use_detectors")]
    detectors: Option<Vec<String>>,

    /// Path to scope file relative to config file
    #[serde(rename = "scope_file")]
    scope_file: Option<String>,

    /// List scope as array
    #[serde(rename = "scope")]
    scope: Option<Vec<String>>,
}

fn print_detail_view(detector_name: &str) {
    let all_detector_names = get_all_detectors_names();
    if !all_detector_names.contains(&detector_name.to_string()) {
        println!("Couldn't recognize detector with name {}", detector_name);
        return;
    }
    let detector = get_issue_detector_by_name(detector_name);
    let watchtower = get_fully_configured_watchtower();
    println!("\nDetector {}", detector_name);
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
    println!("Watchtower Rating (LightChaser version)");
    println!("{}", watchtower.value(detector_name.to_string()));
}

fn print_all_detectors_view() {
    let all_detector_names = get_all_detectors_names();
    let watchtower = get_fully_configured_watchtower();
    println!("\nDetector Registry");
    println!();
    println!("{}   Title (Rating)", right_pad("Name", 30));
    println!();
    for severity in IssueSeverity::iter() {
        print_detectors_view_with_severity(&watchtower, severity, &all_detector_names);
        println!();
    }
    println!();
}

fn print_detectors_view_with_severity(
    watchtower: &Box<dyn WatchTower>,
    severity: IssueSeverity,
    detectors_names: &[String],
) {
    let concerned_detectors = detectors_names
        .iter()
        .filter(|name| {
            let detector = get_issue_detector_by_name(name);
            detector.severity() == severity
        })
        .collect::<Vec<_>>();

    if concerned_detectors.is_empty() {
        return;
    }

    println!("{}\n", severity);
    for name in concerned_detectors {
        let detector = get_issue_detector_by_name(name);
        println!(
            "{} - {} ({:.2})",
            right_pad(name, 30),
            detector.title(),
            watchtower.value(name.to_string()),
        );
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
