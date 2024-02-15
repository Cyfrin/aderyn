#![allow(clippy::borrowed_box)]
use std::{path::PathBuf, process::ExitCode};

use aderyn_core::{
    detect::detector::{get_all_detectors_names, get_issue_detector_by_name},
    watchtower::WatchTower,
};

use crate::core_utils::{_apply_judgement, _display_metrics};

pub(crate) fn auto_register_new_core_detectors(watchtower: &Box<dyn WatchTower>) {
    // Step 1 - Get the difference
    let existing_watchtower_detectors = watchtower.get_registered_detectors_names();
    let current_core_detectors_names = get_all_detectors_names();
    let mut extras = vec![];
    for core_detector in current_core_detectors_names {
        if !existing_watchtower_detectors
            .iter()
            .any(|t| t.clone() == core_detector)
        {
            extras.push(core_detector);
        }
    }

    if extras.is_empty() {
        println!("There are no new detectors to register!");
        return;
    }

    // Step 2 - Register each new detector
    for new_detector in extras {
        let detector = get_issue_detector_by_name(&new_detector);
        println!(
            "Registering {} with severity {} ",
            new_detector,
            detector.severity()
        );
        watchtower.register(new_detector, detector.severity());
    }
}

pub(crate) fn unregister_detector(
    watchtower: &Box<dyn WatchTower>,
    detector_name: &str,
) -> ExitCode {
    let existing_watchtower_detectors = watchtower.get_registered_detectors_names();
    if !existing_watchtower_detectors
        .iter()
        .any(|d| d == detector_name)
    {
        println!("Invalid detector name!");
        return ExitCode::FAILURE;
    }
    watchtower.unregister_detector(detector_name.to_string());
    ExitCode::SUCCESS
}

pub(crate) fn tag_detector(
    watchtower: &Box<dyn WatchTower>,
    detector_name: &str,
    message: &str,
) -> ExitCode {
    let existing_watchtower_detectors = watchtower.get_registered_detectors_names();
    if !existing_watchtower_detectors
        .iter()
        .any(|d| d == detector_name)
    {
        println!("Invalid detector name!");
        return ExitCode::FAILURE;
    }
    watchtower.explicity_tag(detector_name.to_string(), message.to_string());
    ExitCode::SUCCESS
}

pub(crate) fn remove_tag(watchtower: &Box<dyn WatchTower>, detector_name: &str) -> ExitCode {
    let existing_watchtower_detectors = watchtower.get_registered_detectors_names();
    if !existing_watchtower_detectors
        .iter()
        .any(|d| d == detector_name)
    {
        println!("Invalid detector name!");
        return ExitCode::FAILURE;
    }
    watchtower.remove_tag(detector_name.to_string());
    ExitCode::SUCCESS
}

pub(crate) fn give_feedback(watchtower: &Box<dyn WatchTower>, feedback_file: &str) -> ExitCode {
    // If the bare minimum demands are not met, then don't allow taking feedback.
    // Here, this can mean maintainer has not registered newly added detectors
    if !watchtower.is_ready_to_take_feedback() {
        eprintln!("Internal Watchtower Error: Not ready to take feedback");
        return ExitCode::FAILURE;
    }
    let file = PathBuf::from(feedback_file);
    _apply_judgement(watchtower, &file)
}

pub(crate) fn display_metrics(
    watchtower: &Box<dyn WatchTower>,
    detector_name: Option<&str>,
) -> ExitCode {
    // Check to see detector_name is valid

    if !watchtower.is_ready_to_get_metrics() || !watchtower.is_ready_to_calculate_value() {
        eprintln!(
            "Internal Watchtower Error: There are some demanding changes you need to satisfy first"
        );
        return ExitCode::FAILURE;
    }
    _display_metrics(watchtower, detector_name)
}
