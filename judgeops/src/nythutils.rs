#![allow(clippy::borrowed_box)]
use std::{fs::read_dir, path::PathBuf, process::ExitCode};

use aderyn_core::watchtower::WatchTower;

use crate::core_utils::{_apply_judgement, _display_metrics};

pub(crate) fn apply_judgement_from_nyth_with_force_register(
    watchtower: &Box<dyn WatchTower>,
    path_to_nyth: &str,
) -> ExitCode {
    let mut feedbacks_dir = PathBuf::from(path_to_nyth);
    feedbacks_dir.push("feedbacks");

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
