#![allow(clippy::borrowed_box)]
use std::{path::PathBuf, process::ExitCode};

use aderyn_core::watchtower::{Feedback, WatchTower};

use crate::extract::FeedbackExtractor;

trait Forceful {
    fn force_register_detectors(&self, watchtower: &Box<dyn WatchTower>);
}

pub(crate) fn _apply_judgement(watchtower: &Box<dyn WatchTower>, file: &PathBuf) -> ExitCode {
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
            markdown_content: std::fs::read_to_string(file).unwrap(),
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
        eprintln!(
            "Invalid file format! {}",
            file.file_name().unwrap().to_string_lossy()
        );
        return ExitCode::FAILURE;
    }

    println!("Submitted feedback!");
    ExitCode::SUCCESS
}

pub(crate) fn _display_metrics(
    watchtower: &Box<dyn WatchTower>,
    detector_name: Option<&str>,
) -> ExitCode {
    // Check to see detector_name is valid
    if let Some(detector_name) = detector_name {
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
