#![allow(clippy::borrowed_box)]
use std::{
    fs::{self, read_dir},
    path::PathBuf,
    process::ExitCode,
};

use aderyn_core::{detect::detector::IssueSeverity, watchtower::WatchTower};
use my_bot::bot_brain::custom_detectors::Metadata;

use crate::core_utils::{_apply_judgement, _display_metrics};

pub(crate) fn apply_judgement_from_nyth_with_force_register(
    watchtower: &Box<dyn WatchTower>,
    path_to_nyth: &str,
) -> ExitCode {
    let mut feedbacks_dir = PathBuf::from(path_to_nyth);
    feedbacks_dir.push("feedbacks");

    let mut metadata_path = PathBuf::from(path_to_nyth);
    metadata_path.push("metadata");
    metadata_path.push("custom_detectors.json");

    assert!(metadata_path.exists());
    assert!(metadata_path.is_file());

    let metadata: Metadata =
        serde_json::from_str(fs::read_to_string(metadata_path).unwrap().as_str()).unwrap();

    for detector in metadata.custom_detectors {
        let issue_severity = match detector.severity.as_str() {
            "NC (Non Critical)" => IssueSeverity::NC,
            "Low" => IssueSeverity::Low,
            "Medium" => IssueSeverity::Medium,
            "High" => IssueSeverity::High,
            "Critical" => IssueSeverity::Critical,
            &_ => panic!("corrupted metadat file!!"),
        };
        watchtower.register(detector.name, issue_severity);
    }

    assert!(feedbacks_dir.is_dir());
    for file in read_dir(feedbacks_dir).expect("Unable to access feedbacks") {
        let file = file.unwrap().path();
        assert!(file.is_file());
        _apply_judgement(watchtower, &file);
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
