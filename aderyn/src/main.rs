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

use clap::{ArgGroup, Parser, Subcommand};

use notify_debouncer_full::{
    new_debouncer,
    notify::{RecursiveMode, Watcher},
};

use cyfrin_foundry_compilers::utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("icf_dependent").requires("icf")))]
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

    /// Run in Auditor mode, which only outputs manual audit helpers
    #[arg(long)]
    auditor_mode: bool,

    /// Use the newer version of aderyn (in beta)
    #[arg(long, name = "icf")]
    icf: bool,

    /// Path relative to project root, inside which solidity contracts will be analyzed
    #[clap(long, use_value_delimiter = true, group = "icf_dependent")]
    src: Option<Vec<String>>,

    /// Watch for file changes and continuously generate report
    #[arg(short, long, group = "icf_dependent")]
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
        src: cmd_args.src,
        scope: cmd_args.scope,
        exclude: cmd_args.exclude,
        no_snippets: cmd_args.no_snippets,
        skip_build: cmd_args.skip_build,
        skip_cloc: cmd_args.skip_cloc,
        skip_update_check: cmd_args.skip_update_check,
        stdout: cmd_args.stdout,
        auditor_mode: cmd_args.auditor_mode,
        icf: cmd_args.icf || cmd_args.auditor_mode, // If auditor mode engaged, engage ICF
    };

    // Run it once, for the first time
    driver::drive(args.clone());

    // Then run only if file change events are observed
    if cmd_args.watch {
        println!("INFO: Aderyn is entering watch mode !");

        debounce_and_run(
            || {
                driver::drive(args.clone());
            },
            &args,
            Duration::from_millis(50),
        );
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

fn debounce_and_run<F>(driver_func: F, args: &Args, timeout: Duration)
where
    F: Fn() + Copy + Send,
{
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(timeout, None, tx).unwrap();

    debouncer
        .watcher()
        .watch(
            PathBuf::from(args.root.clone()).as_path(),
            RecursiveMode::Recursive,
        )
        .unwrap();

    // Then run again only if file events are observed
    for result in rx {
        match result {
            Ok(_) => {
                driver_func();
            }
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
        println!();
    }
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
