#![allow(clippy::borrowed_box)]

use semver::Version;
use serde::Deserialize;
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
use strum::IntoEnumIterator;

use aderyn_driver::{
    detector::{
        get_all_detectors_names, get_all_issue_detectors, get_issue_detector_by_name,
        IssueDetector, IssueSeverity,
    },
    driver::{self, Args},
};

use clap::{Parser, Subcommand};
use notify_debouncer_full::{
    new_debouncer,
    notify::{RecursiveMode, Watcher},
};

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

    /// Print the output to stdout instead of a file
    #[arg(long)]
    stdout: bool,

    /// Path to aderyn.config.json
    #[arg(short, long)]
    config_file: Option<String>,

    /// Print every detector available
    #[clap(subcommand, name = "registry")]
    registry: Option<RegistryCommand>,

    /// Skip contract build step
    #[arg(long)]
    skip_build: bool,

    /// Skip cloc analysis (line numbers, etc.)
    #[arg(long)]
    skip_cloc: bool,

    /// Skip checking for new versions of Aderyn
    #[arg(long)]
    skip_update_check: bool,

    /// Watch for file changes and continuously generate report
    #[arg(short, long)]
    watch: bool,
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
        skip_build: cmd_args.skip_build,
        skip_cloc: cmd_args.skip_cloc,
        skip_update_check: cmd_args.skip_update_check,
        stdout: cmd_args.stdout,
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
                let mut detector_names = Vec::new();
                let mut subscriptions: Vec<Box<dyn IssueDetector>> = vec![];
                let mut scope_lines: Option<Vec<String>> = args.scope.clone();
                match config.detectors {
                    Some(config_detectors) => {
                        for detector_name in &config_detectors {
                            detector_names.push(detector_name.clone());
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
                        detector_names.extend(all_detector_names);
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
                    skip_build: args.skip_build,
                    skip_cloc: args.skip_cloc,
                    skip_update_check: args.skip_update_check,
                    stdout: args.stdout,
                };
                if cmd_args.watch {
                    println!("INFO: Aderyn is entering watch mode !");
                    // setup debouncer
                    let (tx, rx) = std::sync::mpsc::channel();

                    // no specific tickrate, max debounce time 2 seconds
                    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx).unwrap();

                    debouncer
                        .watcher()
                        .watch(
                            PathBuf::from(new_args.root.clone()).as_path(),
                            RecursiveMode::Recursive,
                        )
                        .unwrap();

                    // Run it once, for the first time
                    let mut subscriptions: Vec<Box<dyn IssueDetector>> = vec![];
                    for detector in &detector_names {
                        subscriptions.push(get_issue_detector_by_name(&detector));
                    }

                    driver::drive_with(new_args.clone(), subscriptions);

                    // Then run again only if file events are observed
                    for result in rx {
                        match result {
                            Ok(_) => {
                                let mut subscriptions: Vec<Box<dyn IssueDetector>> = vec![];
                                for detector in &detector_names {
                                    subscriptions.push(get_issue_detector_by_name(&detector));
                                }

                                driver::drive_with(new_args.clone(), subscriptions);
                            }
                            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
                        }
                        println!();
                    }
                } else {
                    driver::drive_with(new_args, subscriptions);
                }
            }
            Err(_e) => {
                println!("aderyn.config.json wasn't formatted properly! {:?}", _e);
            }
        }
    } else {
        if cmd_args.watch {
            println!("INFO: Aderyn is entering watch mode !");
            // setup debouncer
            let (tx, rx) = std::sync::mpsc::channel();

            // no specific tickrate, max debounce time 2 seconds
            let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx).unwrap();

            debouncer
                .watcher()
                .watch(
                    PathBuf::from(args.root.clone()).as_path(),
                    RecursiveMode::Recursive,
                )
                .unwrap();

            // Run it once, for the first time
            driver::drive(args.clone());

            // Then run only if file change events are observed
            for result in rx {
                match result {
                    Ok(_) => {
                        driver::drive(args.clone());
                    }
                    Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
                }
                println!();
            }
        } else {
            driver::drive(args);
        }
    }

    if !cmd_args.skip_update_check {
        if let Ok(yes) = aderyn_is_currently_running_newest_version() {
            if !yes {
                println!();
                println!(
                    "NEW VERSION OF ADERYN AVAILABLE! Please run `cargo install aderyn` to fully upgrade the current version"
                );
            }
        }
    }
}

#[derive(Deserialize, Clone)]
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
}

fn print_all_detectors_view() {
    let all_detector_names = get_all_detectors_names();
    println!("\nDetector Registry");
    println!();
    println!("{}   Title (Rating)", right_pad("Name", 30));
    println!();
    for severity in IssueSeverity::iter() {
        print_detectors_view_with_severity(severity, &all_detector_names);
        println!();
    }
    println!();
}

fn print_detectors_view_with_severity(severity: IssueSeverity, detectors_names: &[String]) {
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
        println!("{} - {}", right_pad(name, 30), detector.title(),);
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

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn aderyn_is_currently_running_newest_version() -> Result<bool, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let latest_version_checker = client
        .get("https://crates.io/api/v1/crates?q=aderyn&per_page=1")
        .send()?;

    let data = latest_version_checker.json::<Value>()?;

    let newest_version = data["crates"][0]["newest_version"].to_string();
    let newest_version = &newest_version[1..newest_version.len() - 1];

    let newest = Version::parse(newest_version).unwrap();
    let current = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();

    Ok(current >= newest)
}

#[cfg(test)]
mod latest_version_checker_tests {
    use super::*;

    #[test]
    fn can_get_latest_version_from_crate_registry() {
        assert!(aderyn_is_currently_running_newest_version().is_ok())
    }
}
