#![allow(clippy::borrowed_box)]
use std::{path::PathBuf, process::ExitCode};

use aderyn_core::watchtower::{Feedback, WatchTower};

use crate::{extract::FeedbackExtractor, utils::_display_metrics};

pub(crate) fn apply_judgement_from_nyth_with_force_register(
    watchtower: &Box<dyn WatchTower>,
    feedback_file: &str,
) -> ExitCode {
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

pub(crate) fn display_metrics(
    watchtower: &Box<dyn WatchTower>,
    detector_name: Option<&str>,
) -> ExitCode {
    _display_metrics(watchtower, detector_name)
}
