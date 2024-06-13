use aderyn_driver::{
    detector::{get_all_detectors_names, get_issue_detector_by_name, IssueSeverity},
    driver::Args,
};
use notify_debouncer_full::{
    new_debouncer,
    notify::{RecursiveMode, Watcher},
};
use semver::Version;
use serde_json::Value;
use std::{path::PathBuf, time::Duration};
use strum::IntoEnumIterator;

pub fn debounce_and_run<F>(driver_func: F, args: &Args, timeout: Duration)
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

pub fn aderyn_is_currently_running_newest_version() -> Result<bool, reqwest::Error> {
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
