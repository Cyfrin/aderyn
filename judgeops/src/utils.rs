#![allow(clippy::borrowed_box)]
use std::{path::PathBuf, process::ExitCode};

use crate::watchtower::{Feedback, WatchTower};

use crate::extract::{DetectorsUsedExtractor, FeedbackExtractor};
use crate::IssueSeverity;

pub(crate) fn apply_judgement(watchtower: &Box<dyn WatchTower>, feedback_file: &str) -> ExitCode {
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
        let detector_extractor = DetectorsUsedExtractor {
            markdown_content: std::fs::read_to_string(feedback_file).unwrap(),
        };
        let used_detectors = detector_extractor.used_detectors();
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
            all_exposed_detectors: used_detectors
                .unwrap()
                .keys()
                .map(|x| x.to_string())
                .collect(),
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

pub(crate) fn display_metrics(
    watchtower: &Box<dyn WatchTower>,
    detector_name: Option<&str>,
) -> ExitCode {
    if let Some(detector_name) = detector_name {
        let detector_metrics = watchtower.metrics(detector_name.to_string());
        let detector_value = watchtower.value(detector_name.to_string());

        println!("Detector {}\n", detector_name);
        println!("Rating (0-1)   : {:.2}\n", detector_value);
        println!("True positives : {}", detector_metrics.true_positives);
        println!("False positives: {}", detector_metrics.false_positives);
        println!("Trigger count  : {}", detector_metrics.trigger_count);
        println!("Experience     : {}", detector_metrics.experience);

        print!("\nNOTE - The above metrics can vary based on the implementation of watchtower. ");
        print!("Any of the values are not guaranteed to actually reflect what's happening. ");
        print!("Ex: TP - FP is kept at max 5 for LightChaser impl although trigger count can increase indefinitely. ");
        println!("The only obligation for a watchtower is to give out a rating form 0 to 1");
    } else {
        let mut scoreboard = vec![];

        for detector_name in watchtower.get_registered_detectors_names() {
            let detector_value = watchtower.value(detector_name.clone());
            scoreboard.push((detector_value, detector_name));
        }

        if scoreboard.is_empty() {
            println!("There is nothing to show.");
            return ExitCode::SUCCESS;
        }

        scoreboard.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        println!("{: <30}\tRating\n", "Detector");
        for row in scoreboard {
            println!("{: <30}\t{:.2}", row.1, row.0);
        }
    }
    ExitCode::SUCCESS
}

pub(crate) fn register_unseen_detectors(
    watchtower: &Box<dyn WatchTower>,
    feedback_file: &str,
) -> ExitCode {
    let file = PathBuf::from(feedback_file);
    if !file.is_file() || !file.exists() {
        eprintln!("Invalid feedback file submitted! ");
        return ExitCode::FAILURE;
    }

    let extractor = DetectorsUsedExtractor {
        markdown_content: std::fs::read_to_string(feedback_file).unwrap(),
    };

    // Key - name, value - severity
    let detectors = extractor.used_detectors().expect("Corrupted judge report");
    let existing = watchtower.get_registered_detectors_names();
    for detector_name in detectors.keys() {
        if !existing.contains(detector_name) {
            let severity = detectors.get(detector_name).unwrap();

            println!("Registering {} {}", detector_name, severity);
            // let assigned_severity: IssueSeverity = serde_json::from_str(&severity).unwrap();

            let assigned_severity = if severity == &IssueSeverity::Critical.to_string() {
                IssueSeverity::Critical
            } else if severity == &IssueSeverity::High.to_string() {
                IssueSeverity::High
            } else if severity == &IssueSeverity::Medium.to_string() {
                IssueSeverity::Medium
            } else if severity == &IssueSeverity::Low.to_string() {
                IssueSeverity::Low
            } else if severity == &IssueSeverity::NC.to_string() {
                IssueSeverity::NC
            } else {
                panic!("Corrupt!");
            };
            watchtower.register(detector_name.to_string(), assigned_severity)
        }
    }

    ExitCode::SUCCESS
}
