#![allow(clippy::borrowed_box)]
use std::{path::PathBuf, process::ExitCode};

use aderyn_core::{
    detect::detector::{get_all_detectors_names, get_issue_detector_by_name},
    watchtower::{Feedback, WatchTower},
};

use crate::extract::FeedbackExtractor;

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
    if !file.is_file() || !file.exists() {
        eprintln!("Invalid feedback file submitted! ");
        return ExitCode::FAILURE;
    }

    if file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .ends_with(".judge.md")
    {
        let extractor = FeedbackExtractor {
            markdown_content: std::fs::read_to_string(feedback_file).unwrap(),
        };
        let used_detectors = extractor.used_detectors_names();
        let negative_feedbacks = extractor.negative_feedbacks();
        let positive_feedbacks = extractor.positive_feedbacks();
        if used_detectors.is_none() || negative_feedbacks.is_none() || positive_feedbacks.is_none()
        {
            eprintln!("Error extracing feedback");
            return ExitCode::FAILURE;
        }

        let feedback = Feedback {
            positive_feedbacks: positive_feedbacks.unwrap(),
            negative_feedbacks: negative_feedbacks.unwrap(),
            all_exposed_detectors: used_detectors.unwrap(),
        };
        watchtower.take_feedback(feedback);
    } else if file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .ends_with(".json")
    {
        let feedback: Result<Feedback, _> =
            serde_json::from_str(&std::fs::read_to_string(&file).unwrap());

        if let Ok(feedback) = feedback {
            watchtower.take_feedback(feedback);
        } else {
            eprintln!("Invalid feedback JSON schema submitted! ");
            return ExitCode::FAILURE;
        }
    } else {
        eprintln!("Invalid file format!");
        return ExitCode::FAILURE;
    }

    println!("Submitted feedback!");
    ExitCode::SUCCESS
}

pub(crate) fn display_metrics(watchtower: &Box<dyn WatchTower>, detector_name: &str) -> ExitCode {
    // Check to see detector_name is valid

    if !watchtower.is_ready_to_get_metrics() || !watchtower.is_ready_to_calculate_value() {
        eprintln!(
            "Internal Watchtower Error: There are some demanding changes you need to satisfy first"
        );
        return ExitCode::FAILURE;
    }

    let detector_metrics = watchtower.metrics(detector_name.to_string());
    let detector_value = watchtower.value(detector_name.to_string());

    println!("Detector {}\n", detector_name);
    println!("Rating (0-1)   : {:.2}\n", detector_value);
    println!("True positives : {}", detector_metrics.true_positives);
    println!("False positives: {}", detector_metrics.false_positives);
    println!("Trigger count  : {}", detector_metrics.trigger_count);
    println!("Experience     : {}", detector_metrics.experience);

    if let Some(tag) = watchtower.request_tag(detector_name.to_string()) {
        println!("\nTAGS: {}", tag.messages.join(", "));
    }

    print!("\nNOTE - The above metrics can vary based on the implementation of watchtower. ");
    print!("Any of the values are not guaranteed to actually reflect what's happening. ");
    print!("Ex: TP - FP is kept at max 5 for LightChaser impl although trigger count can increase indefinitely. ");
    println!("The only obligation for a watchtower is to give out a rating form 0 to 1");

    ExitCode::SUCCESS
}
