use aderyn_driver::detector::{get_all_detectors_names, get_issue_detector_by_name, IssueSeverity};
use semver::Version;
use serde_json::Value;
use std::{cmp::Ordering, fs::File, io::Write, path::PathBuf, str::FromStr};
use strum::IntoEnumIterator;

pub mod birdsong;
pub mod lsp;
mod panic;

pub fn create_aderyn_toml_file_at(directory: String) {
    let solidity_dir = find_solidity_dir(&directory);
    let aderyn_toml_path = PathBuf::from_str(&directory).unwrap().join("aderyn.toml");
    let mut file = File::create_new(aderyn_toml_path.clone()).expect("File already exists!");
    file.write_fmt(format_args!(
        include_str!("../templates/aderyn.toml"),
        format!("\"{}\"", &solidity_dir)
    ))
    .expect("unable to write to aderyn.toml");
    println!("Created aderyn.toml at {}", aderyn_toml_path.display());
}

pub fn find_solidity_dir(root: &str) -> String {
    let path = PathBuf::from_str(root).expect("invalid path root");
    let indicators = ["hardhat.config.ts", "hardhat.config.js", "foundry.toml", "soldeer.toml"];

    // Check for indicators in the same directory level
    for indicator in indicators {
        let target = path.join(indicator);
        if target.is_file() {
            return path.to_string_lossy().to_string();
        }
    }

    // Check for indicators one level below
    for indicator in indicators {
        let mut nodes = std::fs::read_dir(path.clone())
            .expect("reading path failed")
            .flatten()
            .collect::<Vec<_>>();
        nodes.sort_by(|a, _| {
            if a.file_name().to_string_lossy().contains("contract") {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        for node in nodes {
            if !node.path().is_dir() {
                continue;
            }
            let target = node.path().join(indicator);
            if target.is_file() {
                let location = node.path();
                let toml_entry = location.strip_prefix(path).expect("stripping failed");
                return toml_entry.to_string_lossy().to_string();
            }
        }
    }

    root.to_string()
}

pub fn initialize_niceties() {
    // Crash with a nice message on panic
    panic::add_handler()
}

pub fn print_detail_view(detector_name: &str) {
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

pub fn print_all_detectors_view() {
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

pub fn print_detectors_view_with_severity(severity: IssueSeverity, detectors_names: &[String]) {
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

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn aderyn_is_currently_running_newest_version() -> Option<bool> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("client is unable to initialize");

    let latest_version_checker =
        client.get("https://api.github.com/repos/Cyfrin/aderyn/releases/latest").send().ok()?;

    let data = latest_version_checker.json::<Value>().ok()?;
    let version_string = data.get("tag_name")?.as_str()?;
    let newest = Version::parse(version_string.replace("aderyn-v", "").as_str()).ok()?;
    let current = Version::parse(env!("CARGO_PKG_VERSION")).expect("Pkg version not available");

    Some(current >= newest)
}

#[cfg(test)]
mod latest_version_checker_tests {
    use super::*;

    #[test]
    #[ignore = "fails when frequently run as github will rate limit"]
    fn can_get_latest_version_from_github_releases() {
        assert!(aderyn_is_currently_running_newest_version().is_some())
    }
}
